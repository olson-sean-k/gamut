use std::marker::PhantomData;
use std::ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign,
};

use crate::IntoPrimitive;

pub trait Constraint<K, T> {
    fn map(inner: T) -> Option<T> {
        // It is not possible to implement an identity constraint for any kind
        // `K`, because it would conflict with more specific `Constraint`
        // implementations. Each proxy kind that does not constrain its inner
        // type must provide its own identity constraint (and can use this
        // default implementation).
        Some(inner)
    }
}

pub trait Member<E> {}

#[derive(Debug)]
pub enum NegativeSet {}

#[repr(transparent)]
pub struct Proxy<K, T, C>
where
    C: Constraint<K, T>,
{
    inner: T,
    phantom: PhantomData<(K, C)>,
}

impl<K, T, C> Proxy<K, T, C>
where
    C: Constraint<K, T>,
{
    pub(in crate) fn from_inner_unchecked(inner: T) -> Self {
        Proxy {
            inner,
            phantom: PhantomData,
        }
    }

    pub(in crate) fn try_from_inner(inner: T) -> Option<Self> {
        C::map(inner).map(Self::from_inner_unchecked)
    }

    pub(in crate) fn map<F>(self, f: F) -> Self
    where
        F: FnOnce(T) -> T,
    {
        Self::from(f(self.into_inner()))
    }

    pub(in crate) fn zip_map<F>(self, other: Self, f: F) -> Self
    where
        F: FnOnce(T, T) -> T,
    {
        Self::from(f(self.into_inner(), other.into_inner()))
    }
}

impl<K, T, C> Proxy<K, T, C>
where
    C: Constraint<K, T>,
{
    pub fn into_inner(self) -> T {
        self.inner
    }
}

impl<K, T, C> Add for Proxy<K, T, C>
where
    T: Add<Output = T>,
    C: Constraint<K, T>,
{
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        self.zip_map(other, |x, y| x + y)
    }
}

impl<K, T, C> Add<T> for Proxy<K, T, C>
where
    T: Add<Output = T>,
    C: Constraint<K, T>,
{
    type Output = Self;

    fn add(self, y: T) -> Self::Output {
        self.map(|x| x + y)
    }
}

impl<K, T, C> AddAssign for Proxy<K, T, C>
where
    Self: Clone,
    T: Add<Output = T>,
    C: Constraint<K, T>,
{
    fn add_assign(&mut self, other: Self) {
        *self = self.clone() + other;
    }
}

impl<K, T, C> AddAssign<T> for Proxy<K, T, C>
where
    Self: Clone,
    T: Add<Output = T>,
    C: Constraint<K, T>,
{
    fn add_assign(&mut self, y: T) {
        *self = self.clone() + y;
    }
}

impl<K, T, C> AsRef<T> for Proxy<K, T, C>
where
    C: Constraint<K, T>,
{
    fn as_ref(&self) -> &T {
        &self.inner
    }
}

impl<K, T, C> Clone for Proxy<K, T, C>
where
    T: Clone,
    C: Constraint<K, T>,
{
    fn clone(&self) -> Self {
        Self::from_inner_unchecked(self.inner.clone())
    }
}

impl<K, T, C> Copy for Proxy<K, T, C>
where
    T: Copy,
    C: Constraint<K, T>,
{
}

impl<K, T, C> Div for Proxy<K, T, C>
where
    T: Div<Output = T>,
    C: Constraint<K, T>,
{
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        self.zip_map(other, |x, y| x / y)
    }
}

impl<K, T, C> Div<T> for Proxy<K, T, C>
where
    T: Div<Output = T>,
    C: Constraint<K, T>,
{
    type Output = Self;

    fn div(self, y: T) -> Self::Output {
        self.map(|x| x / y)
    }
}

impl<K, T, C> DivAssign for Proxy<K, T, C>
where
    Self: Clone,
    T: Div<Output = T>,
    C: Constraint<K, T>,
{
    fn div_assign(&mut self, other: Self) {
        *self = self.clone() / other;
    }
}

impl<K, T, C> DivAssign<T> for Proxy<K, T, C>
where
    Self: Clone,
    T: Div<Output = T>,
    C: Constraint<K, T>,
{
    fn div_assign(&mut self, y: T) {
        *self = self.clone() / y;
    }
}

impl<K, T, C> From<T> for Proxy<K, T, C>
where
    C: Constraint<K, T>,
{
    fn from(inner: T) -> Self {
        Proxy {
            inner: C::map(inner).expect("proxy constraint violated"),
            phantom: PhantomData,
        }
    }
}

impl<K, T, C> IntoPrimitive for Proxy<K, T, C>
where
    T: IntoPrimitive,
    C: Constraint<K, T>,
{
    type Primitive = T::Primitive;

    fn into_primitive(self) -> Self::Primitive {
        self.into_inner().into_primitive()
    }
}

impl<K, T, C> Mul for Proxy<K, T, C>
where
    T: Mul<Output = T>,
    C: Constraint<K, T>,
{
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        self.zip_map(other, |x, y| x * y)
    }
}

impl<K, T, C> Mul<T> for Proxy<K, T, C>
where
    T: Mul<Output = T>,
    C: Constraint<K, T>,
{
    type Output = Self;

    fn mul(self, y: T) -> Self::Output {
        self.map(|x| x * y)
    }
}

impl<K, T, C> MulAssign for Proxy<K, T, C>
where
    Self: Clone,
    T: Mul<Output = T>,
    C: Constraint<K, T>,
{
    fn mul_assign(&mut self, other: Self) {
        *self = self.clone() * other;
    }
}

impl<K, T, C> MulAssign<T> for Proxy<K, T, C>
where
    Self: Clone,
    T: Mul<Output = T>,
    C: Constraint<K, T>,
{
    fn mul_assign(&mut self, y: T) {
        *self = self.clone() * y;
    }
}

impl<K, T, C> Neg for Proxy<K, T, C>
where
    T: Neg<Output = T>,
    C: Constraint<K, T> + Member<NegativeSet>,
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        self.map(|x| -x)
    }
}

impl<K, T, C> Rem for Proxy<K, T, C>
where
    T: Rem<Output = T>,
    C: Constraint<K, T>,
{
    type Output = Self;

    fn rem(self, other: Self) -> Self::Output {
        self.zip_map(other, |x, y| x % y)
    }
}

impl<K, T, C> Rem<T> for Proxy<K, T, C>
where
    T: Rem<Output = T>,
    C: Constraint<K, T>,
{
    type Output = Self;

    fn rem(self, y: T) -> Self::Output {
        self.map(|x| x % y)
    }
}

impl<K, T, C> RemAssign for Proxy<K, T, C>
where
    Self: Clone,
    T: Rem<Output = T>,
    C: Constraint<K, T>,
{
    fn rem_assign(&mut self, other: Self) {
        *self = self.clone() % other;
    }
}

impl<K, T, C> RemAssign<T> for Proxy<K, T, C>
where
    Self: Clone,
    T: Rem<Output = T>,
    C: Constraint<K, T>,
{
    fn rem_assign(&mut self, y: T) {
        *self = self.clone() % y;
    }
}

impl<K, T, C> Sub for Proxy<K, T, C>
where
    T: Sub<Output = T>,
    C: Constraint<K, T>,
{
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        self.zip_map(other, |x, y| x - y)
    }
}

impl<K, T, C> Sub<T> for Proxy<K, T, C>
where
    T: Sub<Output = T>,
    C: Constraint<K, T>,
{
    type Output = Self;

    fn sub(self, y: T) -> Self::Output {
        self.map(|x| x - y)
    }
}

impl<K, T, C> SubAssign for Proxy<K, T, C>
where
    Self: Clone,
    T: Sub<Output = T>,
    C: Constraint<K, T>,
{
    fn sub_assign(&mut self, other: Self) {
        *self = self.clone() - other;
    }
}

impl<K, T, C> SubAssign<T> for Proxy<K, T, C>
where
    Self: Clone,
    T: Sub<Output = T>,
    C: Constraint<K, T>,
{
    fn sub_assign(&mut self, y: T) {
        *self = self.clone() - y;
    }
}
