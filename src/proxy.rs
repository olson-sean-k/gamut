use std::marker::PhantomData;
use std::ops::Add;

pub trait Constraint<K, T> {
    fn map(inner: T) -> Option<T>;
}

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

impl<K, T, C> From<T> for Proxy<K, T, C>
where
    C: Constraint<K, T>,
{
    fn from(inner: T) -> Self {
        Proxy {
            inner: C::map(inner).expect(""),
            phantom: PhantomData,
        }
    }
}
