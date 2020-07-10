use std::cmp;
use std::fmt::{self, Debug, Formatter};

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
    T: Ord,
    B: Bounds<T>,
{
    fn map(inner: T) -> Option<T> {
        Some(wrap(inner, B::min_value(), B::max_value()))
    }
}

impl<T, B> Debug for Wrapped<T, B>
where
    T: Debug + Ord,
    B: Bounds<T>,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Wrapped").field(self.as_ref()).finish()
    }
}

#[macro_export]
macro_rules! wrapped {
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
        $crate::wrapped::Wrapped::<$t, B>::from($n)
    }};
}

// TODO: Decide on type bounds and an implementation.
pub fn wrap<T>(value: T, min: T, max: T) -> T
where
    T: Ord,
{
    cmp::min(cmp::max(value, min), max)
}

#[cfg(test)]
mod tests {
    #[test]
    fn wrapped_macro_types() {
        let _ = wrapped!(i32 => 0, [0, 16]);
        let _ = wrapped!(i32 => 0, [0, 32]);
    }
}
