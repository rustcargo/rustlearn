use {
    futures::future,
    futures::task::{ArcWake, waker_ref},
    std::{
        future::Future,
        pin::Pin,
        sync::{Arc, Mutex},
        sync::mpsc::{sync_channel, SyncSender, Receiver},
        task::{Context, Poll, Waker},
        thread,
        time::Duration,
    },
};

pub struct TimerFuture{
    shared_state:Arc<Mutex<SharedState>>,
}

struct SharedState{
    completed:bool,
    waker:Option<Waker>,
}

impl Future for TimerFuture{
    type Output = ();
    fn poll(self:Pin<&mut Self>,ctx:&mut Context<'_>)->Poll<Self::Output>{
        let mut shared_state = self.shared_state.lock().unwrap();

        if shared_state.completed{
            return Poll::Ready(());
        }else{
            //为什么要不断更新waker，因为Future可能在不同线程间迁移，有时候在线程A轮询，有时候在线程B轮询，不同线程下的waker是不一样的
            shared_state.waker = Some(ctx.waker().clone());
            return Poll::Pending;
        }
    }
}
impl TimerFuture{
    //new的时候才是触发动作的时机
    pub fn new(duration:Duration)->Self{
        let shared_state = Arc::new(Mutex::new(SharedState{
            completed:false,
            waker:None,
        }));

        let thread_shared_state = shared_state.clone();

        thread::spawn(move||{
            thread::sleep(duration);

            let mut shared_state = thread_shared_state.lock().unwrap();

            shared_state.completed = true;

            //取出waker的数据，然后放入if let里面做模式匹配，这时候的Some(waker)的waker获取了所有权
            if let Some(waker) = shared_state.waker.take(){
                waker.wake();
            }
        });
        return TimerFuture{
            shared_state:shared_state,
        }
    }
}

///负责执行接收任务，然后执行
struct Executor {
    ready_queue: Receiver<Arc<Task>>,
}

///负责将任务放入队列中
struct Spawner {
    task_sender: SyncSender<Arc<Task>>,
}

struct Task {
    //Task是需要放入Arc里面的，所以，它需要满足Sync的Trait，而它的字段必须满足Sync，所以加Mutex
    //为什么是Option的，因为Futre可能是已经完成的，这个时候Mutex包装的内容是None的。
    //要考虑到在Task完成后，依然有可能触发waker，造成将自己这个重复的任务再次推入到队列中，所以，一旦完成任务，我们就将其设置为None，再次推入到队列也不怕
    future: Mutex<Option<future::BoxFuture<'static, ()>>>,

    //记录自己的SyncSender
    task_sender: SyncSender<Arc<Task>>,
}

fn new_executor_and_spawner() -> (Executor, Spawner) {
    //最大的队列长度
    const MAX_QUEUED_TASKS: usize = 10_000;
    let (task_sender, ready_queue) = sync_channel(MAX_QUEUED_TASKS);
    (Executor { ready_queue }, Spawner { task_sender})
}

impl Spawner {
    fn spawn(&self, future: impl Future<Output = ()> + 'static + Send) {
        let future = future.boxed();
        let task = Arc::new(Task {
            future: Mutex::new(Some(future)),
            task_sender: self.task_sender.clone(),
        });
        //首先发送初始任务到队列中
        self.task_sender.send(task).expect("too many tasks queued");
    }
}

impl ArcWake for Task {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        //将Task包装成Waker，注意，参数是Arc<Self>，
        let cloned = arc_self.clone();

        //Waker的默认实现是，当waker触发时，将自身推送到队列中
        arc_self.task_sender.send(cloned).expect("too many tasks queued");
    }
}

impl Executor {
    fn run(&self) {
        //注意，当所有的task_sender都被关闭的时候，recv的返回就是Err，而不是Ok
        while let Ok(task) = self.ready_queue.recv() {

            //获取future，经过mutex
            let mut future_slot = task.future.lock().unwrap();
            if let Some(mut future) = future_slot.take() {
                //从task创建waker
                let waker = waker_ref(&task);

                //从waker创建context
                let context = &mut Context::from_waker(&*waker);
                // `BoxFuture<T>` 就是`Pin<Box<dyn Future<Output = T> + Send + 'static>>`的别名
                // 触发Poll操作，轮询任务的状态
                if let Poll::Pending = future.as_mut().poll(context) {
                    //一个任务可能会需要多次Pending才能全部完成。
                    //如果任务take了以后，就直接设置为完成状态None，就会造成再次触发时，推入队列的task内容的future为空
                    *future_slot = Some(future);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn play_future() {
        let (executor, spawner) = new_executor_and_spawner();

        //通过spawner启动一个任务
        spawner.spawn(async {
            println!("begin!");
            //触发我们的时间future为2秒
            TimerFuture::new(Duration::new(2, 0)).await;
            println!("end!");
        });

        //释放spawner，从而释放它持有的task_sender
        drop(spawner);

        //运行任务，直至所有的task_sender都释放完了，就自动停止
        executor.run();
    }
}