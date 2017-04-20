use std::slice::Iter;

/// Experiment to combine multiple iterators that return the same value
/// It's similar to Iter::chain and might be obsolete
pub struct CombinedIterator<'a, T>
    where T: 'a
{
    iterators: Vec<Iter<'a, T>>,
    index: usize,
}

impl<'a, T> CombinedIterator<'a, T> {
    /// Construct a new CombinedIterator
    pub fn new(iter: Iter<'a, T>) -> CombinedIterator<'a, T> {
        CombinedIterator {
            iterators: vec![iter],
            index: 0,
        }
    }

    /// Add a new iterator to the CombinedIterator
    pub fn and(&mut self, iter: Iter<'a, T>) -> &mut Self {
        self.iterators.push(iter);
        self
    }
}

impl<'a, T> Iterator for CombinedIterator<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            // if we're past the last iterator, return None
            if self.index >= self.iterators.len() {
                return None;
            
            // Else if the current iterator still has some values left, return the next value
            } else if let Some(item) = self.iterators[self.index].next() {
                return Some(item);
            
            // Else move to the next iterator and try again
            } else {
                self.index += 1;
            }
        }
    }
}
