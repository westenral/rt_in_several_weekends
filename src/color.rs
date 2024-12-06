use crate::vec3::*;

type Color = Vec3;

impl Color {
    fn write_color(&self) {
        let scaled = |f| (f * 255.999) as usize;
        println!(
            "{} {} {}",
            scaled(self.r()),
            scaled(self.g()),
            scaled(self.b())
        );
    }

    fn r(&self) -> f64 {
        self.0
    }

    fn g(&self) -> f64 {
        self.1
    }

    fn b(&self) -> f64 {
        self.2
    }
}
