macro_rules! impl_vec3_binop {
    ($trait_name:ident, $method_name:ident, Vec3) => {
        impl std::ops::$trait_name<Vec3> for Vec3 {
            type Output = Vec3;

            fn $method_name(self, rhs: Vec3) -> Self::Output {
                Self(
                    self.0.$method_name(rhs.0),
                    self.1.$method_name(rhs.1),
                    self.2.$method_name(rhs.2),
                )
            }
        }
    };
    ($trait_name:ident, $method_name:ident, f64) => {
        impl std::ops::$trait_name<f64> for Vec3 {
            type Output = Vec3;

            fn $method_name(self, rhs: f64) -> Self::Output {
                Self(
                    self.0.$method_name(rhs),
                    self.1.$method_name(rhs),
                    self.2.$method_name(rhs),
                )
            }
        }
    };
    ($trait_name:ident, $method_name:ident, Vec3, assign) => {
        impl std::ops::$trait_name<Vec3> for Vec3 {
            fn $method_name(&mut self, rhs: Vec3) {
                self.0.$method_name(rhs.0);
                self.1.$method_name(rhs.1);
                self.2.$method_name(rhs.2);
            }
        }
    };
    ($trait_name:ident, $method_name:ident, f64, assign) => {
        impl std::ops::$trait_name<f64> for Vec3 {
            fn $method_name(&mut self, rhs: f64) {
                self.0.$method_name(rhs);
                self.1.$method_name(rhs);
                self.2.$method_name(rhs);
            }
        }
    };
}

pub struct Vec3(pub f64, pub f64, pub f64);

impl_vec3_binop!(Add, add, Vec3);
impl_vec3_binop!(Add, add, f64);
impl_vec3_binop!(AddAssign, add_assign, Vec3, assign);
impl_vec3_binop!(AddAssign, add_assign, f64, assign);
impl_vec3_binop!(Sub, sub, Vec3);
impl_vec3_binop!(Sub, sub, f64);
impl_vec3_binop!(SubAssign, sub_assign, Vec3, assign);
impl_vec3_binop!(SubAssign, sub_assign, f64, assign);
impl_vec3_binop!(Mul, mul, f64);
impl_vec3_binop!(MulAssign, mul_assign, f64, assign);
impl_vec3_binop!(Div, div, f64);
impl_vec3_binop!(DivAssign, div_assign, f64, assign);

type Pos = Vec3;

impl Pos {
    fn x(&self) -> f64 {
        self.0
    }

    fn y(&self) -> f64 {
        self.1
    }

    fn z(&self) -> f64 {
        self.2
    }
}
