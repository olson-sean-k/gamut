mod bounded;
mod proxy;

pub use crate::bounded::clamped::*;
pub use crate::bounded::wrapped::*;
pub use crate::bounded::Bounds;
pub use crate::proxy::*;

pub trait Primitive {}

impl Primitive for isize {}
impl Primitive for usize {}
impl Primitive for i8 {}
impl Primitive for i16 {}
impl Primitive for i32 {}
impl Primitive for i64 {}
impl Primitive for u8 {}
impl Primitive for u16 {}
impl Primitive for u32 {}
impl Primitive for u64 {}
impl Primitive for f32 {}
impl Primitive for f64 {}

pub trait IntoPrimitive {
    type Primitive: Primitive;

    fn into_primitive(self) -> Self::Primitive;
}

impl<T> IntoPrimitive for T
where
    T: Primitive,
{
    type Primitive = Self;

    fn into_primitive(self) -> Self::Primitive {
        self
    }
}

// The following functions may cause logic errors and should not be used
// directly. They are exposed for downstream crates that implement additional
// proxy types.

#[doc(hidden)]
pub fn proxy_from_inner_unchecked<K, T, C>(inner: T) -> Proxy<K, T, C>
where
    C: Constraint<K, T>,
{
    Proxy::from_inner_unchecked(inner)
}

#[doc(hidden)]
pub fn try_proxy_from_inner<K, T, C>(inner: T) -> Option<Proxy<K, T, C>>
where
    C: Constraint<K, T>,
{
    Proxy::try_from_inner(inner)
}

#[doc(hidden)]
pub fn map_proxy<K, T, C, F>(proxy: Proxy<K, T, C>, f: F) -> Proxy<K, T, C>
where
    C: Constraint<K, T>,
    F: FnOnce(T) -> T,
{
    proxy.map(f)
}

#[doc(hidden)]
pub fn zip_map_proxy<K, T, C, F>(
    left: Proxy<K, T, C>,
    right: Proxy<K, T, C>,
    f: F,
) -> Proxy<K, T, C>
where
    C: Constraint<K, T>,
    F: FnOnce(T, T) -> T,
{
    left.zip_map(right, f)
}
