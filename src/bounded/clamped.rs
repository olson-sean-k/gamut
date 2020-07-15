use num_traits::Bounded;
use std::cmp::{self, Ordering};
use std::fmt::{self, Debug, Formatter};

use crate::bounded::{Bounds, NOneOne, ZeroMax, ZeroOne};
use crate::proxy::{Constraint, Proxy, ProxyExt};

mod inner {
    pub enum ClampedKind {}
}
use self::inner::*;

pub type Clamped<T, B> = Proxy<ClampedKind, T, B>;
pub type ClampedPositive<T> = Clamped<T, ZeroMax<T>>;
pub type ClampedUnit<T> = Clamped<T, ZeroOne<T>>;
pub type ClampedSignedUnit<T> = Clamped<T, NOneOne<T>>;

impl<T, B> Constraint<ClampedKind, T> for B
where
    T: Ord,
    B: Bounds<T>,
{
    fn map(inner: T) -> Option<T> {
        Some(clamp(inner, B::min_value(), B::max_value()))
    }
}

impl<T, B> Bounded for Clamped<T, B>
where
    T: Ord,
    B: Bounds<T>,
{
    fn min_value() -> Self {
        Self::from_inner_unchecked(B::min_value())
    }

    fn max_value() -> Self {
        Self::from_inner_unchecked(B::max_value())
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

impl<T, B> Eq for Clamped<T, B>
where
    T: Ord + Eq,
    B: Bounds<T>,
{
}

impl<T, B> Ord for Clamped<T, B>
where
    T: Ord,
    B: Bounds<T>,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl<T, B> PartialEq for Clamped<T, B>
where
    T: Ord + PartialEq,
    B: Bounds<T>,
{
    fn eq(&self, other: &Self) -> bool {
        self.as_ref().eq(other.as_ref())
    }
}

impl<T, B> PartialEq<T> for Clamped<T, B>
where
    T: Ord + PartialEq,
    B: Bounds<T>,
{
    fn eq(&self, inner: &T) -> bool {
        self.as_ref().eq(inner)
    }
}

impl<T, B> PartialOrd for Clamped<T, B>
where
    T: Ord,
    B: Bounds<T>,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.as_ref().partial_cmp(other.as_ref())
    }
}

impl<T, B> PartialOrd<T> for Clamped<T, B>
where
    T: Ord,
    B: Bounds<T>,
{
    fn partial_cmp(&self, inner: &T) -> Option<Ordering> {
        self.as_ref().partial_cmp(inner)
    }
}

#[macro_export]
macro_rules! clamped {
    ($t:ty => $n:expr, [ $min:expr, $max:expr ]) => {{
        use static_assertions::const_assert;

        const_assert!($min <= $max);
        struct B;
        impl $crate::Bounds<$t> for B {
            fn min_value() -> $t {
                $min
            }

            fn max_value() -> $t {
                $max
            }
        }
        $crate::Clamped::<$t, B>::from($n)
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
    fn clamped_macro_types() {
        let x = clamped!(i8 => 0, [0, 16]);
        let y = x + 32;
        let z = x - 32;

        assert_eq!(y, 16);
        assert_eq!(z, 0);
    }
}
