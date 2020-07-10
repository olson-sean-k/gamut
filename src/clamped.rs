use num_traits::{Bounded, Zero};
use std::cmp;
use std::fmt::{self, Debug, Formatter};
use std::marker::PhantomData;

use crate::proxy::{Constraint, Proxy};

mod inner {
    use super::*;

    pub enum ClampedKind {}

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
}
use self::inner::*;

pub type Clamped<T, B> = Proxy<ClampedKind, T, B>;
pub type Positive<T> = Clamped<T, ZeroMax<T>>;

pub trait Bounds<T>
where
    T: Ord,
{
    //const MIN_VALUE: T;
    //const MAX_VALUE: T;

    fn min_value() -> T;
    fn max_value() -> T;
}

impl<T, B> Constraint<ClampedKind, T> for B
where
    T: Ord,
    B: Bounds<T>,
{
    fn map(inner: T) -> Option<T> {
        //Some(clamp(inner, B::MIN_VALUE, B::MAX_VALUE))
        Some(clamp(inner, B::min_value(), B::max_value()))
    }
}

impl<T, B> Debug for Clamped<T, B>
where
    T: Debug + Ord,
    B: Bounds<T>,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Clamped").field(self.as_ref()).finish()
    }
}

#[macro_export]
macro_rules! clamped {
    ($t:ty => $n:expr, [ $min:expr, $max:expr ]) => {{
        use static_assertions::const_assert;

        const_assert!($min <= $max);
        struct B;
        impl $crate::clamped::Bounds<$t> for B {
            //const MIN_VALUE: $t = $min;
            //const MAX_VALUE: $t = $max;

            fn min_value() -> $t {
                $min
            }

            fn max_value() -> $t {
                $max
            }
        }
        $crate::clamped::Clamped::<$t, B>::from($n)
    }};
}

pub fn clamp<T>(value: T, min: T, max: T) -> T
where
    T: Ord,
{
    cmp::min(cmp::max(value, min), max)
}

#[cfg(test)]
mod tests {
    #[test]
    fn foo() {
        let _ = clamped!(u8 => 0, [0, 16]);
        let _ = clamped!(u8 => 0, [0, 32]);
    }
}
