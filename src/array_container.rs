use crate::ArrayElement;
use crate::array_adapter::ArrayAdapter;

pub trait ArrayContainer<T: ArrayElement> {
    fn len(&self) -> usize;

    #[inline]
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn index(&self, index: usize) -> &T;
}

impl<T: ArrayElement, A: ArrayAdapter<T>> ArrayContainer<T> for A {
    #[inline]
    fn len(&self) -> usize {
        self.len()
    }
    #[inline]
    fn index(&self, index: usize) -> &T {
        self.index(index)
    }
    #[inline]
    fn is_empty(&self) -> bool {
        self.is_empty()
    }
}