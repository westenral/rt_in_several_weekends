#[macro_use]
mod macros;

pub use Vec3 as Pos;

#[derive(Clone, Copy, Debug)]
pub struct Vec3(pub f64, pub f64, pub f64);

impl Vec3 {
    // vector math

    // #[inline(always)]
    pub fn dot(&self, rhs: &Vec3) -> f64 {
        self.0 * rhs.0 + self.1 * rhs.1 + self.2 * rhs.2
    }

    // #[inline(always)]
    pub fn length_squared(&self) -> f64 {
        self.dot(self)
    }

    // #[inline(always)]
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    // #[inline(always)]
    pub fn cross(&self, rhs: &Vec3) -> Vec3 {
        Vec3(
            self.1 * rhs.2 - self.2 * rhs.1,
            -self.0 * rhs.2 + self.2 * rhs.0,
            self.0 * rhs.1 - self.1 * rhs.0,
        )
    }

    // #[inline(always)]
    pub fn unit_vec(&self) -> Vec3 {
        self / self.length()
    }

    pub fn rand(min: f64, max: f64) -> Self {
        Self(randf64(min, max), randf64(min, max), randf64(min, max))
    }

    pub fn random_on_hemisphere(norm: &Vec3) -> Vec3 {
        let dir = Vec3::rand_unit_vec();
        if dir.dot(norm) > 0. {
            dir
        } else {
            -dir
        }
    }

    pub fn rand_unit_vec() -> Vec3 {
        // generate random in unit cube, rejection method to get it in sphere
        loop {
            let vec = Vec3::rand(-1., 1.);
            let len_sqrd = vec.length_squared();
            if 1e-160 < len_sqrd && len_sqrd <= 1. {
                break vec / len_sqrd.sqrt();
            }
        }
    }

    pub fn rand_in_unit_disk() -> Vec3 {
        loop {
            let vec = Vec3(randf64(-1., 1.), randf64(-1., 1.), 0.);
            if vec.length_squared() < 1. {
                return vec;
            }
        }
    }

    /// Checks whether the vector is near zero
    pub fn near_zero(&self) -> bool {
        let margin = 1e-8;
        self.0.abs() < margin && self.1.abs() < margin && self.2.abs() < margin
    }

    /// Reflect a vector around a unit-length direction
    pub fn reflect(&self, norm: &Vec3) -> Vec3 {
        self - 2. * self.dot(norm) * norm
    }

    pub fn refract(&self, norm: &Vec3, rel_refract_index: f64) -> Vec3 {
        let cos_theta = (-self).dot(norm).min(1.);
        let r_prime_perp = rel_refract_index * (self + cos_theta * norm);
        let r_prime_parallel = -((1.0 - r_prime_perp.length_squared()).abs().sqrt()) * norm;
        r_prime_perp + r_prime_parallel
    }
}

fn randf64(min: f64, max: f64) -> f64 {
    let scale = max - min;
    let bruh = fastrand::f64();
    // FMA??? join the cargo cult
    bruh * scale + min
}

impl std::ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Self(-self.0, -self.1, -self.2)
    }
}

impl std::ops::Neg for &Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3(-self.0, -self.1, -self.2)
    }
}

impl std::iter::Sum for Vec3 {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.reduce(|acc, x| acc + x).unwrap_or(Self(0., 0., 0.))
    }
}

impl_vec3_binop!(Add, add, Vec3);
impl_vec3_binop!(Add, add, f64);
impl_vec3_binop!(AddAssign, add_assign, Vec3, assign);
impl_vec3_binop!(AddAssign, add_assign, f64, assign);

impl_vec3_binop!(Sub, sub, Vec3);
impl_vec3_binop!(Sub, sub, f64);
impl_vec3_binop!(SubAssign, sub_assign, Vec3, assign);
impl_vec3_binop!(SubAssign, sub_assign, f64, assign);

impl_vec3_binop!(Mul, mul, Vec3);
impl_vec3_binop!(Mul, mul, f64);
impl_vec3_binop!(MulAssign, mul_assign, f64, assign);
impl_vec3_binop!(MulAssign, mul_assign, Vec3, assign);

impl_vec3_binop!(Div, div, Vec3);
impl_vec3_binop!(Div, div, f64);
impl_vec3_binop!(DivAssign, div_assign, f64, assign);
impl_vec3_binop!(DivAssign, div_assign, Vec3, assign);

impl Pos {
    #[inline(always)]
    pub fn x(&self) -> f64 {
        self.0
    }

    #[inline(always)]
    pub fn y(&self) -> f64 {
        self.1
    }

    #[inline(always)]
    pub fn z(&self) -> f64 {
        self.2
    }
}
