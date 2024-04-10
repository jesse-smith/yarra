use core::marker::PhantomData;
use core::ops::Index;
use crate::{ArrayContainer, ArrayElement};

#[derive(Clone, Debug, PartialEq)]
pub struct Array<T: ArrayElement, const D: usize, C: ArrayContainer<T> = Vec<T>>
{
    shape: [usize; D],
    data: C,
    _type: PhantomData<T>,
}

impl<T: ArrayElement, const D: usize, C: ArrayContainer<T>> Array<T, D, C> {
    pub fn init(shape: [usize; D], data: C) -> Result<Self, &'static str> {
        let size: usize = shape.iter().product();
        if size != data.len() {
            Err("Shape and data length mismatch")
        } else {
            Ok(Self {
                shape,
                data,
                _type: PhantomData,
            })
        }
    }

    #[inline]
    pub const fn shape(&self) -> &[usize; D] {
        &self.shape
    }

    #[inline]
    pub const fn data(&self) -> &C {
        &self.data
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.data.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    fn index_to_offset(&self, index: [usize; D]) -> usize {
        index
            .iter()
            .zip(self.shape.iter())
            .fold(0, |acc, (&i, &s)| acc * s + i)
    }
}

impl<T: ArrayElement, const D: usize, C: ArrayContainer<T>> Index<[usize; D]> for Array<T, D, C> {
    type Output = T;

    #[inline]
    fn index(&self, index: [usize; D]) -> &Self::Output {
        self.data.index(self.index_to_offset(index))
    }
}