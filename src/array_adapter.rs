use core::ops::Deref;
use core::borrow::Borrow;

use crate::ArrayElement;

pub trait ArrayAdapter<T: ArrayElement> {
    fn len(&self) -> usize;
    fn index(&self, index: usize) -> &T;

    #[inline]
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

pub struct DerefSliceAdapter<T: ArrayElement>(dyn Deref<Target = [T]>);
pub struct AsRefSliceAdapter<T: ArrayElement>(dyn AsRef<[T]>);
pub struct BorrowSliceAdapter<T: ArrayElement>(dyn Borrow<[T]>);

impl<T: ArrayElement> ArrayAdapter<T> for DerefSliceAdapter<T> {
    #[inline]
    fn len(&self) -> usize {
        self.0.len()
    }
    #[inline]
    fn index(&self, index: usize) -> &T {
        &self.0[index]
    }
    #[inline]
    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl<T: ArrayElement> ArrayAdapter<T> for AsRefSliceAdapter<T> {
    #[inline]
    fn len(&self) -> usize {
        self.0.as_ref().len()
    }
    #[inline]
    fn index(&self, index: usize) -> &T {
        &self.0.as_ref()[index]
    }
    #[inline]
    fn is_empty(&self) -> bool {
        self.0.as_ref().is_empty()
    }
}

impl<T: ArrayElement> ArrayAdapter<T> for BorrowSliceAdapter<T> {
    #[inline]
    fn len(&self) -> usize {
        self.0.borrow().len()
    }
    #[inline]
    fn index(&self, index: usize) -> &T {
        &self.0.borrow()[index]
    }
    #[inline]
    fn is_empty(&self) -> bool {
        self.0.borrow().is_empty()
    }
}
