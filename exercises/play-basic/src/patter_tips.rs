#[derive(Default, Debug)]
struct Foo {
    message: String,
}

#[derive(Default, Debug)]
struct FooBuilder {
    foo: Foo,
}

impl FooBuilder {
    pub fn new() -> FooBuilder {
        Self::default()
    }

    pub fn first_name(mut self, name: &str) -> Self {
        self.foo.message.push_str(name);
        self
    }

    pub fn last_name(mut self, name: &str) -> Self {
        self.foo.message.push_str(name);
        self
    }

    pub fn finish(self) -> Foo {
        self.foo
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_pattern() {
        let f = FooBuilder::new().first_name("hh").last_name("ok").finish();

        assert_eq!(f.message, String::from("hhok"));
    }
}
