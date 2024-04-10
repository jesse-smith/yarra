use core::ops::{Neg, Add, Sub, Mul, Div};
use num_traits::{Zero, One};


pub trait ArrayElement:
    Sized +
    Neg<Output = Self> +
    Add<Output = Self> +
    Sub<Output = Self> +
    Mul<Output = Self> +
    Div<Output = Self>
{
    fn zero() -> Self;
    fn one() -> Self;
}

impl<T> ArrayElement for T where T:
    Sized +
    Neg<Output = Self> +
    Add<Output = Self> +
    Sub<Output = Self> +
    Mul<Output = Self> +
    Div<Output = Self> +
    Zero +
    One
{
    fn zero() -> Self {
        Self::zero()
    }

    fn one() -> Self {
        Self::one()
    }
}