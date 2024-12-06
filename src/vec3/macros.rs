// traits have to be implemented for (Vec, Vec), (&Vec, Vec), (Vec, &Vec), and
// (&Vec, &Vec), and i have no idea what the best way to do that is

macro_rules! impl_vec3_binop {
    ($trait_name:ident, $method_name:ident, Vec3) => {
        impl std::ops::$trait_name<Vec3> for Vec3 {
            type Output = Vec3;

            #[inline(always)]
            fn $method_name(self, rhs: Vec3) -> Self::Output {
                Self(
                    self.0.$method_name(rhs.0),
                    self.1.$method_name(rhs.1),
                    self.2.$method_name(rhs.2),
                )
            }
        }

        impl std::ops::$trait_name<&Vec3> for Vec3 {
            type Output = Vec3;

            #[inline(always)]
            fn $method_name(self, rhs: &Vec3) -> Self::Output {
                Self(
                    self.0.$method_name(rhs.0),
                    self.1.$method_name(rhs.1),
                    self.2.$method_name(rhs.2),
                )
            }
        }

        impl std::ops::$trait_name<Vec3> for &Vec3 {
            type Output = Vec3;

            #[inline(always)]
            fn $method_name(self, rhs: Vec3) -> Self::Output {
                Vec3(
                    self.0.$method_name(rhs.0),
                    self.1.$method_name(rhs.1),
                    self.2.$method_name(rhs.2),
                )
            }
        }

        impl std::ops::$trait_name<&Vec3> for &Vec3 {
            type Output = Vec3;

            #[inline(always)]
            fn $method_name(self, rhs: &Vec3) -> Self::Output {
                Vec3(
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

            #[inline(always)]
            fn $method_name(self, rhs: f64) -> Self::Output {
                Self(
                    self.0.$method_name(rhs),
                    self.1.$method_name(rhs),
                    self.2.$method_name(rhs),
                )
            }
        }

        impl std::ops::$trait_name<&f64> for Vec3 {
            type Output = Vec3;

            #[inline(always)]
            fn $method_name(self, rhs: &f64) -> Self::Output {
                Self(
                    self.0.$method_name(rhs),
                    self.1.$method_name(rhs),
                    self.2.$method_name(rhs),
                )
            }
        }

        impl std::ops::$trait_name<f64> for &Vec3 {
            type Output = Vec3;

            #[inline(always)]
            fn $method_name(self, rhs: f64) -> Self::Output {
                Vec3(
                    self.0.$method_name(rhs),
                    self.1.$method_name(rhs),
                    self.2.$method_name(rhs),
                )
            }
        }

        impl std::ops::$trait_name<&f64> for &Vec3 {
            type Output = Vec3;

            #[inline(always)]
            fn $method_name(self, rhs: &f64) -> Self::Output {
                Vec3(
                    self.0.$method_name(rhs),
                    self.1.$method_name(rhs),
                    self.2.$method_name(rhs),
                )
            }
        }
    };
    ($trait_name:ident, $method_name:ident, Vec3, assign) => {
        impl std::ops::$trait_name<&Vec3> for Vec3 {
            #[inline(always)]
            fn $method_name(&mut self, rhs: &Vec3) {
                self.0.$method_name(rhs.0);
                self.1.$method_name(rhs.1);
                self.2.$method_name(rhs.2);
            }
        }

        impl std::ops::$trait_name<Vec3> for Vec3 {
            #[inline(always)]
            fn $method_name(&mut self, rhs: Vec3) {
                self.0.$method_name(rhs.0);
                self.1.$method_name(rhs.1);
                self.2.$method_name(rhs.2);
            }
        }
    };
    ($trait_name:ident, $method_name:ident, f64, assign) => {
        impl std::ops::$trait_name<f64> for Vec3 {
            #[inline(always)]
            fn $method_name(&mut self, rhs: f64) {
                self.0.$method_name(rhs);
                self.1.$method_name(rhs);
                self.2.$method_name(rhs);
            }
        }

        impl std::ops::$trait_name<&f64> for Vec3 {
            #[inline(always)]
            fn $method_name(&mut self, rhs: &f64) {
                self.0.$method_name(rhs);
                self.1.$method_name(rhs);
                self.2.$method_name(rhs);
            }
        }
    };
}
