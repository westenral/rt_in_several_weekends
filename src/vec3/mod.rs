#[macro_use]
mod macros;

#[derive(Clone, Copy, Debug)]
pub struct Vec3(pub f64, pub f64, pub f64);

impl Vec3 {
    // vector math

    #[inline(always)]
    pub fn dot(&self, rhs: &Vec3) -> f64 {
        self.0 * rhs.0 + self.1 * rhs.1 + self.2 * rhs.2
    }

    #[inline(always)]
    pub fn length_squared(&self) -> f64 {
        self.dot(self)
    }

    #[inline(always)]
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    #[inline(always)]
    pub fn cross(&self, rhs: &Vec3) -> Vec3 {
        Vec3(
            self.1 * rhs.2 - self.2 * rhs.1,
            -self.0 * rhs.2 + self.2 * rhs.0,
            self.0 * rhs.1 - self.1 * rhs.0,
        )
    }

    #[inline(always)]
    pub fn normalized(&self) -> Vec3 {
        self / self.length()
    }
}

impl std::ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Self(-self.0, -self.1, -self.2)
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

type Pos = Vec3;

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
