pub use crate::vec3::Vec3 as Color;

impl Color {
    pub fn write_color(&self) {
        let transform = |x: f64| {
            // linear to gamma 2 space
            // F(x) = x^2
            // F-1(x) = sqrt(x)
            // then scale [0, 1) to [0, 255]
            (x.max(0.).sqrt().clamp(0., 0.999) * 256.) as usize
        };
        println!(
            "{} {} {}",
            transform(self.r()),
            transform(self.g()),
            transform(self.b())
        );
    }

    pub fn random() -> Self {
        Color(fastrand::f64(), fastrand::f64(), fastrand::f64())
    }

    pub fn random_range(min: f64, max: f64) -> Self {
        Color(randf64(min, max), randf64(min, max), randf64(min, max))
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

fn randf64(min: f64, max: f64) -> f64 {
    fastrand::f64() * (max - min) + min
}
