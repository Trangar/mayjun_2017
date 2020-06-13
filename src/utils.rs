pub trait VecUtils<T> {
    /// Try to remove a card at a given index
    /// Returns Some(T) if the index is valid
    /// Returns None if the index is not valid, either < 0 or >= len()
    fn try_remove(&mut self, index: usize) -> Option<T>;

    /// Try to insert a card
    /// If index >= 0 and index < len(), then it calls self.insert()
    /// if index == len() then it calls self.push()
    /// else it returns false
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
        match index {
            x if x < self.len() => {
                self.insert(index, value);
                true
            }
            x if x == self.len() => {
                self.push(value);
                true
            }
            _ => false,
        }
    }
}
