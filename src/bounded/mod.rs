pub mod clamped;
pub mod wrapped;

use num_traits::{Bounded, Zero};
use std::marker::PhantomData;

pub trait Bounds<T>
where
    T: Ord,
{
    //const MIN_VALUE: T;
    //const MAX_VALUE: T;

    fn min_value() -> T;
    fn max_value() -> T;
}

pub struct ZeroMax<T>(PhantomData<T>);

impl<T> Bounds<T> for ZeroMax<T>
where
    T: Bounded + Ord + Zero,
{
    fn min_value() -> T {
        T::zero()
    }

    fn max_value() -> T {
        T::max_value()
    }
}
