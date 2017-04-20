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
