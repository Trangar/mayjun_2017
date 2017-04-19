use std::slice::Iter;

pub struct CombinedIterator<'a, T>
    where T: 'a
{
    iterators: Vec<Iter<'a, T>>,
    index: usize,
}

impl<'a, T> CombinedIterator<'a, T> {
    pub fn new(iter: Iter<'a, T>) -> CombinedIterator<'a, T> {
        CombinedIterator {
            iterators: vec![iter],
            index: 0,
        }
    }

    pub fn and(&mut self, iter: Iter<'a, T>) -> &mut Self {
        self.iterators.push(iter);
        self
    }
}

impl<'a, T> Iterator for CombinedIterator<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.index >= self.iterators.len() {
                return None;
            } else if let Some(item) = self.iterators[self.index].next() {
                return Some(item);
            } else {
                self.index += 1;
            }
        }
    }
}
