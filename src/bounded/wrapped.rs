use num_traits::Bounded;
use std::cmp::Ordering;
use std::fmt::{self, Debug, Formatter};
use std::ops::{Add, Rem, Sub};

use crate::bounded::clamped;
use crate::bounded::{Bounds, NOneOne, ZeroMax, ZeroOne};
use crate::proxy::{Constraint, Proxy};

mod inner {
    pub enum WrappedKind {}
}
use self::inner::*;

pub type Wrapped<T, B> = Proxy<WrappedKind, T, B>;
pub type WrappedPositive<T> = Wrapped<T, ZeroMax<T>>;
pub type WrappedUnit<T> = Wrapped<T, ZeroOne<T>>;
pub type WrappedSignedUnit<T> = Wrapped<T, NOneOne<T>>;

impl<T, B> Constraint<WrappedKind, T> for B
where
    T: Add<Output = T> + Copy + Ord + Sub<Output = T> + Rem<Output = T>,
    B: Bounds<T>,
{
    fn map(inner: T) -> Option<T> {
        Some(wrap(inner, B::min_value(), B::max_value()))
    }
}

impl<T, B> Bounded for Wrapped<T, B>
where
    T: Add<Output = T> + Copy + Ord + Sub<Output = T> + Rem<Output = T>,
    B: Bounds<T>,
{
    fn min_value() -> Self {
        Self::from_inner_unchecked(B::min_value())
    }

    fn max_value() -> Self {
        Self::from_inner_unchecked(B::max_value())
    }
}

impl<T, B> Debug for Wrapped<T, B>
where
    T: Add<Output = T> + Copy + Debug + Ord + Sub<Output = T> + Rem<Output = T>,
    B: Bounds<T>,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Wrapped").field(self.as_ref()).finish()
    }
}

impl<T, B> Eq for Wrapped<T, B>
where
    T: Add<Output = T> + Copy + Eq + Ord + Sub<Output = T> + Rem<Output = T>,
    B: Bounds<T>,
{
}

impl<T, B> Ord for Wrapped<T, B>
where
    T: Add<Output = T> + Copy + Ord + Sub<Output = T> + Rem<Output = T>,
    B: Bounds<T>,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl<T, B> PartialEq for Wrapped<T, B>
where
    T: Add<Output = T> + Copy + Ord + PartialEq + Sub<Output = T> + Rem<Output = T>,
    B: Bounds<T>,
{
    fn eq(&self, other: &Self) -> bool {
        self.as_ref().eq(other.as_ref())
    }
}

impl<T, B> PartialEq<T> for Wrapped<T, B>
where
    T: Add<Output = T> + Copy + Ord + PartialEq + Sub<Output = T> + Rem<Output = T>,
    B: Bounds<T>,
{
    fn eq(&self, inner: &T) -> bool {
        self.as_ref().eq(inner)
    }
}

impl<T, B> PartialOrd for Wrapped<T, B>
where
    T: Add<Output = T> + Copy + Ord + Sub<Output = T> + Rem<Output = T>,
    B: Bounds<T>,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.as_ref().partial_cmp(other.as_ref())
    }
}

impl<T, B> PartialOrd<T> for Wrapped<T, B>
where
    T: Add<Output = T> + Copy + Ord + Sub<Output = T> + Rem<Output = T>,
    B: Bounds<T>,
{
    fn partial_cmp(&self, inner: &T) -> Option<Ordering> {
        self.as_ref().partial_cmp(inner)
    }
}

#[macro_export]
macro_rules! wrapped {
    ($t:ty => $n:expr, [ $min:expr, $max:expr ]) => {{
        use static_assertions::const_assert;

        // TODO: This requires a constant expression, which is difficult to
        //       achieve with non-primitive types on stable Rust today. See
        //       https://github.com/rust-lang/rust/issues/49146
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
        $crate::Wrapped::<$t, B>::from($n)
    }};
}

pub fn wrap<T>(value: T, min: T, max: T) -> T
where
    T: Add<Output = T> + Copy + Ord + Sub<Output = T> + Rem<Output = T>,
{
    // Output is clamped to avoid breaching bounds due to error.
    if value > max {
        clamped::clamp(min + ((value - min) % (max - min)), min, max)
    }
    else if value < min {
        clamped::clamp(max - ((value - max) % (max - min)), min, max)
    }
    else {
        value
    }
}

#[cfg(test)]
mod tests {
    // TODO: Implement a minimal wrapper for testing against numeric types that
    //       use a floating-point representation.
    #[test]
    fn wrapped_macro_types() {
        let x = wrapped!(i32 => 1, [1, 16]);
        let y = x + 33;
        let z = x - 1;

        assert_eq!(y, 4);
        assert_eq!(z, 16);
    }
}
