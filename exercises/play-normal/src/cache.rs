pub struct Cache<T, F, R>
where
    F: Fn(&T) -> R,
{
    t: T,
    f: F,
    r: Option<R>,
}

impl<T, F, R> Cache<T, F, R>
where
    F: Fn(&T) -> R,
    R: Clone,
{
    pub fn new(t: T, f: F) -> Self {
        Self { t, f, r: None }
    }

    pub fn update(&mut self, n: T) {
        self.r = None;
        self.t = n;
    }

    pub fn eval(&mut self) -> R {
        if let Some(r) = &self.r {
            r.clone()
        } else {
            let r = (self.f)(&self.t);
            self.r = Some(r.clone());
            r
        }
    }
}

#[cfg(test)]
mod test {
    use super::Cache;

    #[test]
    fn new() {
        let square: fn(&isize) -> isize = |&x| x * x;
        let c = Cache::new(2isize, square);
        assert_eq!(c.r, None);
        assert_eq!(c.t, 2);
    }

    #[test]
    fn update() {
        let square: fn(&isize) -> isize = |&x| x * x;
        let mut c = Cache::new(2isize, square);
        let _ = c.eval();
        assert_ne!(c.r, None);
        c.update(3);
        assert_eq!(c.r, None);
        assert_eq!(c.t, 3);
    }

    #[test]
    fn eval() {
        let square: fn(&isize) -> isize = |&x| x * x;
        let mut c = Cache::new(2isize, square);
        assert_eq!(c.eval(), 4);
        c.update(3);
        assert_eq!(c.eval(), 9);
        assert_eq!(c.eval(), 9);
    }
}
