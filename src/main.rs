
enum MyOption<T> {
    None,
    Some(T),
}

impl<T> MyOption<T> {
    fn unwrap(self) -> T {
        match self {
            MyOption::Some(v) => v,
            MyOption::None => panic!("called `MyOption::unwrap()` on a `None` value"),
        }
    }

    fn expect(self, error: &str) -> T {
        match self {
            MyOption::Some(v) => v,
            MyOption::None => panic!("{}", error),
        }
    }
}

fn main() {
    let cm: MyOption<u8> = MyOption::Some(3u8);
    println!("max is {}", cm.expect("error!!!"));

}
