extern crate num;
#[allow(unused_imports)]
#[macro_use]
extern crate static_assertions;

mod clamp;

pub use clamp::{clamp, Bounds, Clamped};

use num::Num;

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

// `Proxy` types should NOT implement `Deref`. This is tempting, but it
// provides access to methods that betray the proxy (like `max_value`).
pub trait Proxy<T>: Sized
where
    T: Num,
{
    fn from_inner(value: T) -> Self;
    fn into_inner(self) -> T;
}
