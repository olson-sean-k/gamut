use num::Num;
use std::cmp;
use std::marker::PhantomData;

use Proxy;

pub trait Bounds<T>
where
    T: Num,
{
    const MIN_VALUE: T;
    const MAX_VALUE: T;
}

#[derive(Clone, Copy, Debug)]
pub struct Clamped<T, B>
where
    T: Num + Ord,
    B: Bounds<T>,
{
    inner: T,
    phantom: PhantomData<B>,
}

impl<T, B> From<T> for Clamped<T, B>
where
    T: Num + Ord,
    B: Bounds<T>,
{
    fn from(inner: T) -> Self {
        Proxy::from_inner(inner)
    }
}

impl<T, B> Proxy<T> for Clamped<T, B>
where
    T: Num + Ord,
    B: Bounds<T>,
{
    fn from_inner(inner: T) -> Self {
        Clamped {
            inner: clamp(inner, B::MIN_VALUE, B::MAX_VALUE),
            phantom: PhantomData,
        }
    }

    fn into_inner(self) -> T {
        self.inner
    }
}

#[macro_export]
macro_rules! clamped {
    ($t:ty => $n:expr, [ $min:expr, $max:expr ]) => {{
        const_assert!(bounds; $min <= $max);
        struct B;
        impl Bounds<$t> for B {
            const MIN_VALUE: $t = $min;
            const MAX_VALUE: $t = $max;
        }
        Clamped::<$t, B>::from_inner($n)
    }};
}

pub fn clamp<T>(value: T, min: T, max: T) -> T
where
    T: Num + Ord
{
    cmp::min(cmp::max(value, min), max)
}

#[cfg(test)]
mod tests {
    use Proxy;
    use clamp::{Bounds, Clamped};

    #[test]
    fn foo() {
        let _ = clamped!(u8 => 0, [0, 16]);
        let _ = clamped!(u8 => 0, [0, 32]);
    }
}
