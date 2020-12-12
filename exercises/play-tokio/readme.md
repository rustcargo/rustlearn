## 1.说明
Tokio 它是Rust语言的一种异步**运行时** 可以用来编写可靠，异步的Rust应用. 它有以下几个特点:
* 快速: Tokio是零成本抽象的，可以带给你接近裸机的性能.
* 可靠的: Tokio基于Rust语言的生命周期，类型系统，并发模型来减少bug和确保线程安全.
* 可扩展: Tokio有非常小的占用，并能处理背压(backpressure)和取消(cancellation)操作.

Tokio是一个事件驱动的非阻塞I/O平台，用于使用Rust编写异步应用. 在较高的层次上，它提供了几个主要的组件:
* 基于多线程与工作流窃取的 任务调度器 [scheduler](https://docs.rs/tokio/latest/tokio/runtime/index.html).
* 响应式的，基于操作系统的事件队列(比如，epoll, kqueue, IOCP, 等...).
* 异步的[TCP and UDP](https://docs.rs/tokio/latest/tokio/net/index.html) socket.

这些组件提供了用来构建异步应用所需要的运行时组件.

[官方原文指南](https://tokio.rs/tokio/tutorial).