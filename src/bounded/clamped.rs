use std::cmp;
use std::fmt::{self, Debug, Formatter};

use crate::bounded::{Bounds, NOneOne, ZeroMax, ZeroOne};
use crate::proxy::{Constraint, Proxy};

mod inner {
    pub enum ClampedKind {}
}
use self::inner::*;

pub type Clamped<T, B> = Proxy<ClampedKind, T, B>;
pub type Positive<T> = Clamped<T, ZeroMax<T>>;
pub type Unit<T> = Clamped<T, ZeroOne<T>>;
pub type SignedUnit<T> = Clamped<T, NOneOne<T>>;

impl<T, B> Constraint<ClampedKind, T> for B
where
    T: Ord,
    B: Bounds<T>,
{
    fn map(inner: T) -> Option<T> {
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
        impl $crate::Bounds<$t> for B {
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
    fn clamped_macro_types() {
        let _ = clamped!(u8 => 0, [0, 16]);
        let _ = clamped!(u8 => 0, [0, 32]);
    }
}
