use super::Camera;

// it's a little overkill...
#[derive(Default)]
pub struct CameraBuilder {
    aspect_ratio: Option<f64>,
    image_width: Option<u64>,
    samples_per_pixel: Option<u64>,
    max_bounces: Option<u64>,
}

impl CameraBuilder {
    pub fn with_aspect_ratio(self, aspect_ratio: f64) -> Self {
        CameraBuilder {
            aspect_ratio: Some(aspect_ratio),
            ..self
        }
    }

    pub fn with_image_width(self, image_width: u64) -> Self {
        CameraBuilder {
            image_width: Some(image_width),
            ..self
        }
    }

    pub fn with_samples_per_pixel(self, samples_per_pixel: u64) -> Self {
        CameraBuilder {
            samples_per_pixel: Some(samples_per_pixel),
            ..self
        }
    }

    pub fn with_max_bounces(self, max_bounces: u64) -> Self {
        CameraBuilder {
            max_bounces: Some(max_bounces),
            ..self
        }
    }

    pub fn build(self) -> Camera {
        Camera::new(
            self.aspect_ratio.unwrap_or(1.),
            self.image_width.unwrap_or(100),
            self.samples_per_pixel.unwrap_or(10),
            self.max_bounces.unwrap_or(10),
        )
    }
}
