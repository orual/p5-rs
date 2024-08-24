use num_traits::Num;

pub fn map<T>(x: T, in_min: T, in_max: T, out_min: T, out_max: T) -> T
where
    T: Num + Copy,
{
    (x - in_min) * (out_max - out_min) / (in_max - in_min) + out_min
}

pub fn map_clamped<T>(x: T, in_min: T, in_max: T, out_min: T, out_max: T) -> T
where
    T: Num + Copy + Constrain<Output = T>,
{
    map(x, in_min, in_max, out_min, out_max).constrain(out_min, out_max)
}

pub fn norm<T>(n: T, min: T, max: T) -> T
where
    T: Num + Copy,
{
    map(n, min, max, T::zero(), T::one())
}

pub fn lerp<T>(start: T, stop: T, amt: T) -> T
where
    T: Num + Copy,
{
    amt * (stop - start) + start
}

pub trait Constrain {
    type Output;
    fn constrain(self, min: Self, max: Self) -> Self::Output;
}

impl<T> Constrain for T
where
    T: Num + Copy + PartialOrd,
{
    type Output = T;
    fn constrain(self, min: Self, max: Self) -> Self::Output {
        num_traits::clamp(self, min, max)
    }
}

pub trait ExpE {
    type Output;
    fn exp(self) -> Self::Output;
}

impl<T> ExpE for T
where
    T: Num + Copy + From<f64> + Into<f64> + Into<i32>,
{
    type Output = T;
    fn exp(self) -> Self::Output {
        if self.rem(T::one()).is_zero() {
            let power: i32 = Into::<i32>::into(self);
            std::f64::consts::E.powi(power).into()
        } else {
            let power: f64 = self.into();
            std::f64::consts::E.powf(power).into()
        }
    }
}

pub trait Sq {
    type Output;
    fn sq(self) -> Self::Output;
}

impl<T> Sq for T
where
    T: Num + Copy,
{
    type Output = T;
    fn sq(self) -> Self::Output {
        self * self
    }
}

pub trait Fract {
    type Output;
    fn fract(self) -> Self::Output;
}

impl<T> Fract for T
where
    T: Num + Copy,
{
    type Output = T;
    fn fract(self) -> Self::Output {
        self.rem(T::one())
    }
}

#[test]
fn basic_test_floats() {
    assert_eq!(map(0.0, 0.0, 1024.0, 0.0, 2459.0), 0.0);
    assert_eq!(map(1024.0, 0.0, 1024.0, 0.0, 2459.0), 2459.0);
    assert_eq!(map(-10.0, 1.0, 101.0, 1.0, 201.0), -21.0);
    assert_eq!(map(300.0, 0.0, 100.0, 100.0, 300.0), 700.0);
}

#[test]
fn basic_test_ints() {
    assert_eq!(map(0, 0, 1024, 0, 2459), 0);
    assert_eq!(map(1024, 0, 1024, 0, 2459), 2459);
    assert_eq!(map(-10, -1, 99, -14, 186), -32);
    assert_eq!(map(300, 0, 100, 100, 300), 700);
}
