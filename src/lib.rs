mod bounded;
mod proxy;

pub use crate::bounded::{clamped, wrapped, Bounds};
pub use crate::proxy::Proxy;

pub trait Primitive {}

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
