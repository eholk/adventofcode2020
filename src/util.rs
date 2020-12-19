pub trait Unwrap<T> {
    fn unwrap(self) -> T;
}

impl<T> Unwrap<T> for Option<T> {
    fn unwrap(self) -> T {
        Option::unwrap(self)
    }
}

impl<T, E: std::fmt::Debug> Unwrap<T> for std::result::Result<T, E> {
    fn unwrap(self) -> T {
        Result::unwrap(self)
    }
}
