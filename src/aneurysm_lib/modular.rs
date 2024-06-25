use core::fmt;
use std::ops::{Add, AddAssign, Deref, Rem, Sub, SubAssign};

pub mod prelude {
    use super::*;

    pub trait NumOps:
        Add<Output = Self>
        + AddAssign
        + Sub<Output = Self>
        + SubAssign
        + Rem<Output = Self>
        + PartialOrd
        + Clone
        + Copy
        + Sized
    {
    }
    impl<T> NumOps for T where
        T: Add<Output = Self>
            + AddAssign
            + Sub<Output = Self>
            + SubAssign
            + Rem<Output = Self>
            + PartialOrd
            + Clone
            + Copy
            + Sized
    {
    }
}
use prelude::*;

/// Simulates modular arithmetic (used for the data pointer)
#[derive(Clone, Copy)]
pub struct Modular<T>
where
    T: NumOps,
{
    pub limit: T,
    num: T,
}

impl<T> Modular<T>
where
    T: NumOps,
{
    #[allow(dead_code)]
    pub fn from_value(num: T, limit: T) -> Self {
        Self { limit, num }
    }
}

impl<T> Modular<T>
where
    T: NumOps + Default,
{
    pub fn with_limit(limit: T) -> Self {
        Self {
            limit,
            num: T::default(),
        }
    }

    pub fn reset(&mut self) {
        self.num = T::default()
    }
}

impl<T> Deref for Modular<T>
where
    T: NumOps,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.num
    }
}

impl<T> fmt::Display for Modular<T>
where
    T: NumOps + fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.num)
    }
}

impl<T> Add<T> for Modular<T>
where
    T: NumOps,
{
    type Output = Modular<T>;

    fn add(self, rhs: T) -> Self::Output {
        Self {
            limit: self.limit,
            num: ((self.num + rhs) % self.limit),
        }
    }
}

impl<T> Add<&T> for Modular<T>
where
    T: NumOps,
{
    type Output = Modular<T>;

    fn add(self, rhs: &T) -> Self::Output {
        self + *rhs
    }
}

impl<T> Add<T> for &Modular<T>
where
    T: NumOps,
{
    type Output = Modular<T>;

    fn add(self, rhs: T) -> Self::Output {
        *self + rhs
    }
}

impl<T> Add<&T> for &Modular<T>
where
    T: NumOps,
{
    type Output = Modular<T>;

    fn add(self, rhs: &T) -> Self::Output {
        *self + *rhs
    }
}

impl<T> AddAssign<T> for Modular<T>
where
    T: NumOps,
{
    fn add_assign(&mut self, rhs: T) {
        *self = *self + rhs
    }
}

impl<T> AddAssign<&T> for Modular<T>
where
    T: NumOps,
{
    fn add_assign(&mut self, rhs: &T) {
        *self += *rhs
    }
}

impl<T> AddAssign<T> for &mut Modular<T>
where
    T: NumOps,
{
    fn add_assign(&mut self, rhs: T) {
        **self = **self + rhs
    }
}

impl<T> AddAssign<&T> for &mut Modular<T>
where
    T: NumOps,
{
    fn add_assign(&mut self, rhs: &T) {
        **self += *rhs
    }
}

impl<T> Sub<T> for Modular<T>
where
    T: NumOps,
{
    type Output = Modular<T>;

    fn sub(self, rhs: T) -> Self::Output {
        Self {
            limit: self.limit,
            num: if self.num >= rhs {
                self.num - rhs
            } else {
                (self.limit - (rhs - self.num) % self.limit) % self.limit
            },
        }
    }
}

impl<T> Sub<&T> for Modular<T>
where
    T: NumOps,
{
    type Output = Modular<T>;

    fn sub(self, rhs: &T) -> Self::Output {
        self - *rhs
    }
}

impl<T> Sub<T> for &Modular<T>
where
    T: NumOps,
{
    type Output = Modular<T>;

    fn sub(self, rhs: T) -> Self::Output {
        *self - rhs
    }
}

impl<T> Sub<&T> for &Modular<T>
where
    T: NumOps,
{
    type Output = Modular<T>;

    fn sub(self, rhs: &T) -> Self::Output {
        *self - *rhs
    }
}

impl<T> SubAssign<T> for Modular<T>
where
    T: NumOps,
{
    fn sub_assign(&mut self, rhs: T) {
        *self = *self - rhs
    }
}

impl<T> SubAssign<&T> for Modular<T>
where
    T: NumOps,
{
    fn sub_assign(&mut self, rhs: &T) {
        *self -= *rhs
    }
}

impl<T> SubAssign<T> for &mut Modular<T>
where
    T: NumOps,
{
    fn sub_assign(&mut self, rhs: T) {
        **self = **self - rhs
    }
}

impl<T> SubAssign<&T> for &mut Modular<T>
where
    T: NumOps,
{
    fn sub_assign(&mut self, rhs: &T) {
        **self -= *rhs
    }
}

#[cfg(test)]
mod tests {
    use super::Modular;

    #[test]
    fn modular_add() {
        const LIMIT: usize = 1000;

        let modular = Modular::from_value(456, LIMIT);

        for i in 0..3 {
            assert_eq!(*(modular + 544 + i * LIMIT), 0);
            assert_eq!(*(modular + &543 + i * LIMIT), 999);
            assert_eq!(*(&modular + 657 + i * LIMIT), 113);
            assert_eq!(*(&modular + &732 + i * LIMIT), 188)
        }
    }

    #[test]
    fn modular_sub() {
        const LIMIT: usize = 1000;

        let modular = Modular::from_value(456, LIMIT);

        for i in 0..3 {
            assert_eq!(*(modular - 456 + i * LIMIT), 0);
            assert_eq!(*(modular - &457 + i * LIMIT), 999);
            assert_eq!(*(&modular - 584 + i * LIMIT), 872);
            assert_eq!(*(&modular - &338 + i * LIMIT), 118)
        }
    }
}
