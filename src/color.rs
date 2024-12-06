use crate::vec3::*;

type Color = Vec3;

impl Color {
    pub fn write_color(&self) {
        let scaled = |f| (f * 255.999) as usize;
        println!(
            "{} {} {}",
            scaled(self.r()),
            scaled(self.g()),
            scaled(self.b())
        );
    }

    #[inline(always)]
    pub fn r(&self) -> f64 {
        self.0
    }

    #[inline(always)]
    pub fn g(&self) -> f64 {
        self.1
    }

    #[inline(always)]
    pub fn b(&self) -> f64 {
        self.2
    }
}
