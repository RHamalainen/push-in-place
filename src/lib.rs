pub trait PushInPlace<T> {
    fn push_in_place(self, value: T) -> Self;
}

impl<T> PushInPlace<T> for Vec<T> {
    fn push_in_place(mut self, value: T) -> Self {
        self.push(value);
        self
    }
}
