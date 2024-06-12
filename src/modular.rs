use core::fmt;
use std::ops::{Add, AddAssign, Deref, Rem, Sub, SubAssign};

mod prelude {
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
pub use prelude::*;

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
    type Output = Self;

    fn add(self, rhs: T) -> Self::Output {
        Self {
            limit: self.limit,
            num: ((self.num + rhs) % self.limit),
        }
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

impl<T> Sub<T> for Modular<T>
where
    T: NumOps,
{
    type Output = Self;

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

impl<T> SubAssign<T> for Modular<T>
where
    T: NumOps,
{
    fn sub_assign(&mut self, rhs: T) {
        *self = *self - rhs
    }
}

#[cfg(test)]
mod tests {
    use super::Modular;

    #[test]
    fn wrapping_add() {
        const LIMIT: usize = 1000;

        let wrapping = Modular::from_value(456, LIMIT);

        assert_eq!(*(wrapping + 544), 0);
        assert_eq!(*(wrapping + 543), 999);
        assert_eq!(*(wrapping + 657), 113);

        assert_eq!(*(wrapping + 1544), 0);
        assert_eq!(*(wrapping + 1543), 999);
        assert_eq!(*(wrapping + 1657), 113);
    }

    #[test]
    fn wrapping_sub() {
        let wrapping = Modular::from_value(456, 1000);

        assert_eq!(*(wrapping - 456), 0);
        assert_eq!(*(wrapping - 457), 999);
        assert_eq!(*(wrapping - 584), 872);

        assert_eq!(*(wrapping - 1456), 0);
        assert_eq!(*(wrapping - 1457), 999);
        assert_eq!(*(wrapping - 1584), 872);
    }
}
