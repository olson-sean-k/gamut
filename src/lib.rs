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
