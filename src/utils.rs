pub trait VecUtils<T> {
    fn try_remove(&mut self, index: usize) -> Option<T>;
    fn push_or_insert(&mut self, index: usize, value: T) -> bool;
}

impl<T> VecUtils<T> for Vec<T> {
    fn try_remove(&mut self, index: usize) -> Option<T> {
        if self.len() <= index {
            None
        } else {
            Some(self.remove(index))
        }
    }

    fn push_or_insert(&mut self, index: usize, value: T) -> bool {
        if self.len() > index {
            self.insert(index, value);
            true
        } else if self.len() == index {
            self.push(value);
            true
        } else {
            false
        }
    }
}
