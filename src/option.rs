#[derive(Debug)]
pub enum Option<T> {
    None,
    Some(T),
}

impl<T> Option<T> {
    pub fn unwrap(self) -> T {
        self.expect("called `Option::unwrap()` on a `None` value")
    }

    pub fn expect(self, error: &str) -> T {
        match self {
            Option::Some(v) => v,
            Option::None => panic!("{}", error),
        }
    }
}
