pub mod clamped;
pub mod wrapped;

use num_traits::{Bounded, One, Signed, Zero};
use std::marker::PhantomData;

use crate::proxy::{Member, NegativeSet, OneSet, ZeroSet};

pub trait Bounds<T>
where
    T: Ord,
{
    //const MIN_VALUE: T;
    //const MAX_VALUE: T;

    fn min_value() -> T;
    fn max_value() -> T;
}

pub struct NOneOne<T>(PhantomData<T>);

impl<T> Bounds<T> for NOneOne<T>
where
    T: One + Ord + Signed,
{
    fn min_value() -> T {
        -T::one()
    }

    fn max_value() -> T {
        T::one()
    }
}

impl<T> Member<NegativeSet> for NOneOne<T> {}
impl<T> Member<OneSet> for NOneOne<T> {}
impl<T> Member<ZeroSet> for NOneOne<T> {}

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

pub struct ZeroOne<T>(PhantomData<T>);

impl<T> Bounds<T> for ZeroOne<T>
where
    T: One + Ord + Zero,
{
    fn min_value() -> T {
        T::zero()
    }

    fn max_value() -> T {
        T::one()
    }
}

impl<T> Member<OneSet> for ZeroOne<T> {}
impl<T> Member<ZeroSet> for ZeroOne<T> {}
