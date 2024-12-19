mod camera_builder;

pub use camera_builder::*;

use crate::{color::*, hit::*, ray::*, vec3::*};

pub struct Camera {
    // pub aspect_ratio: f64,
    image_width: u64,
    image_height: u64,
    // vfov: f64,
    // focal_length: f64,
    // viewport_upper_left: Pos,
    pixel_00_pos: Pos,
    // viewport_u: Vec3,
    // viewport_v: Vec3,
    pixel_du: Vec3,
    pixel_dv: Vec3,
    center: Pos,

    samples_per_pixel: u64,
    pixel_sample_scale: f64,
    max_bounces: u64,

    defocus_angle: f64,
    defocus_u: Vec3,
    defocus_v: Vec3,
}

impl Camera {
    #[allow(clippy::too_many_arguments)]
    /// Use CameraBuilder instead.
    pub fn new(
        aspect_ratio: f64,
        image_width: u64,
        samples_per_pixel: u64,
        max_bounces: u64,
        vfov: f64,
        lookat: Pos,
        lookfrom: Pos,
        vup: Vec3,
        defocus_angle: f64,
        focus_dist: f64,
    ) -> Self {
        let pixel_sample_scale = 1.0 / samples_per_pixel as f64;

        // minimum height of 1
        let image_height = ((image_width as f64 / aspect_ratio) as u64).max(1);

        // camera info
        let camera_position = lookfrom;
        // let focal_length = (lookfrom - lookat).length();
        let theta = vfov;
        let h = (theta / 2.).tan();

        // camera viewport info
        let viewport_height = 2. * h * focus_dist;
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);

        // calculate orthonormal basis (u, v, w)
        let w = (lookfrom - lookat).unit_vec();
        let u = vup.cross(&w).unit_vec();
        let v = w.cross(&u);

        // vectors along viewport top and left edges
        let viewport_u = viewport_width * u;
        let viewport_v = viewport_height * -v;

        // horizontal and vertical vec between in-world pixel centers
        let pixel_du = viewport_u / image_width as f64;
        let pixel_dv = viewport_v / image_height as f64;

        // position of upper left pixel
        let viewport_upper_left =
            camera_position - (focus_dist * w) - viewport_u / 2. - viewport_v / 2.;
        let pixel_00_pos = viewport_upper_left + pixel_du / 2. + pixel_dv / 2.;

        let defocus_radius = focus_dist * (defocus_angle / 2.).tan();
        let defocus_u = u * defocus_radius;
        let defocus_v = v * defocus_radius;

        Self {
            // aspect_ratio,
            image_width,
            image_height,
            // vfov,
            // focal_length,
            // viewport_upper_left,
            pixel_00_pos,
            // viewport_u,
            // viewport_v,
            pixel_du,
            pixel_dv,
            center: camera_position,

            samples_per_pixel,
            pixel_sample_scale,
            max_bounces,

            defocus_angle,
            defocus_u,
            defocus_v,
        }
    }

    pub fn render_parallel(&self, world: &(impl Hit + Sync)) {
        use std::{sync::mpsc, thread};

        let start_time = std::time::Instant::now();
        eprint!("Starting render\r");

        // get number of threads available
        let threads = num_cpus::get();

        // channels for assigning work
        // assigns which line needs to be rendered
        let assign_rxs = {
            let mut assign_txs = vec![];
            let mut assign_rxs = vec![];

            for _ in 0..threads {
                let (tx, rx) = mpsc::channel::<u64>();
                assign_txs.push(tx);
                assign_rxs.push(rx);
            }

            // assign every line to be rendered
            for i in 0..self.image_height {
                assign_txs[i as usize % threads].send(i).unwrap();
            }

            assign_rxs
        };

        // channel for receiving work
        let (report_tx, report_rx) = mpsc::channel::<(usize, Vec<Color>)>();

        // create "accumulator" thread
        // responsible for  receiving data from threads
        let image_height = self.image_height;
        let image_width = self.image_width;
        let start_time_d = start_time;
        let accumulator = thread::spawn(move || {
            let mut lines_completed = 0;

            let mut image = vec![Color(0., 0., 0.); (image_height * image_width) as usize];
            while let Ok((line, pixels)) = report_rx.recv() {
                // gross clone..............agggghghh
                image[(line * image_width as usize)..((line + 1) * image_width as usize)]
                    .clone_from_slice(&pixels);
                lines_completed += 1;
                eprint!(
                    "\rRender Progress: {:>6.2} %\tTime: {:.1?}               \r",
                    lines_completed as f64 / image_height as f64 * 100.,
                    start_time_d.elapsed()
                )
            }

            image
        });

        // create "worker" threads
        // responsible for rendering the lines they are assigned
        // closure is "move" because it needs to take ownership of report_tx
        // so that the dispatcher can correctly finish when all worker
        // threads finish
        thread::scope(move |s| {
            for assign_rx in assign_rxs {
                let report_tx = report_tx.clone();
                s.spawn(move || {
                    while let Ok(line) = assign_rx.recv() {
                        let pixels = (0..self.image_width)
                            .map(|x| self.pixel_color(world, x, line))
                            .collect();

                        report_tx.send((line as usize, pixels)).unwrap();
                    }
                });
            }
        });

        // output final image
        let image = accumulator.join().unwrap();

        println!("P3\n{} {}\n255", self.image_width, self.image_height);
        // write pixel colors to stdout
        for pixel in image {
            pixel.write_color();
        }
        eprintln!(
            "\rFinished rendering in {:.4} seconds                           ",
            start_time.elapsed().as_millis() as f64 / 1000.0
        );
    }

    pub fn _render_parallel(&self, world: &(impl Hit + Sync)) {
        use std::thread;

        // for tracking
        let start_time = std::time::Instant::now();

        // calculate how many lines each thread gets
        let cores = num_cpus::get();
        let leftover = self.image_height % cores as u64;
        let lines_per_core = (self.image_height - leftover) / cores as u64;

        // buffer for pixels
        let mut pixels: Vec<Color> =
            Vec::with_capacity((self.image_width * self.image_height) as usize);

        // for tracking
        let (tx, rx) = std::sync::mpsc::channel::<usize>();

        // progress watcher
        let image_height = self.image_height;
        thread::spawn(move || {
            let mut lines = 0;

            while let Ok(i) = rx.recv() {
                lines += i;
                eprint!(
                    "\rRender Progress: {:>6.2} %\tTime: {:.1?}                 \r",
                    lines as f64 / image_height as f64 * 100.,
                    start_time.elapsed()
                )
            }
        });

        thread::scope(|s| {
            let mut handles = vec![];
            for i in 0..cores {
                let tx = tx.clone();
                handles.push(s.spawn(move || {
                    // determine start and end lines to render
                    let start: u64 = i as u64 * lines_per_core;
                    let lines = if i == (cores - 1) {
                        self.image_height - start
                    } else {
                        lines_per_core
                    };
                    let end: u64 = start + lines;

                    // rendering
                    let mut pixels = Vec::with_capacity((lines * self.image_width) as usize);
                    for y in start..end {
                        tx.send(1).unwrap();
                        for x in 0..self.image_width {
                            pixels.push(self.pixel_color(world, x, y));
                        }
                    }

                    pixels
                }));
            }

            // concat all the pixels
            for handle in handles {
                pixels.append(&mut handle.join().unwrap());
            }
        });

        println!("P3\n{} {}\n255", self.image_width, self.image_height);
        // write pixel colors to stdout
        for pixel in pixels {
            pixel.write_color();
        }
        eprintln!(
            "\rFinished rendering in {:.4} seconds                           ",
            start_time.elapsed().as_millis() as f64 / 1000.0
        );
    }

    fn pixel_color(&self, world: &impl Hit, x: u64, y: u64) -> Color {
        (0..self.samples_per_pixel)
            .map(|_| self.ray_color(&self.get_ray(x, y), world, 0))
            .sum::<Color>()
            * self.pixel_sample_scale
    }

    // outputs to stdout rn...
    pub fn _render(&self, world: &impl Hit) {
        let start_time = std::time::Instant::now();

        println!("P3\n{} {}\n255", self.image_width, self.image_height);

        for y in 0..self.image_height {
            eprint!("\rLines remaining: {:>6}", self.image_height - y);

            for x in 0..self.image_width {
                let mut color = Color(0., 0., 0.);

                for _ in 0..self.samples_per_pixel {
                    let ray = self.get_ray(x, y);
                    color += self.ray_color(&ray, world, 0);
                }

                (color * self.pixel_sample_scale).write_color();
            }
        }

        eprintln!(
            "\rFinished rendering in {:.4} seconds                           ",
            start_time.elapsed().as_millis() as f64 / 1000.0
        );
    }

    fn ray_color(&self, ray: &Ray, world: &impl Hit, bounces: u64) -> Color {
        if bounces > self.max_bounces {
            return Color(0., 0., 0.);
        }

        // object intersection
        if let Some(hit_info) = world.hit(ray, 0.001..f64::INFINITY) {
            // uniform distribution
            // let next_dir = Vec3::random_on_hemisphere(&hit_info.normal);
            // lambertian distribution
            // let next_dir = Vec3::rand_unit_vec() + hit_info.normal;
            // let next_ray = Ray {
            //     origin: hit_info.pos,
            //     dir: next_dir,
            // };
            // return 0.5 * self.ray_color(&next_ray, world, bounces + 1);

            if let Some((ray, attenuation)) = hit_info.mat.scatter(ray, &hit_info) {
                return attenuation * self.ray_color(&ray, world, bounces + 1);
            }
            return Color(0., 0., 0.);
        }

        // background color
        let unit_ray = ray.dir.unit_vec();
        let scaled_y = (unit_ray.y() + 1.0) * 0.5;
        let c1 = Color(1., 1., 1.);
        let c2 = Color(0.5, 0.7, 1.0);
        c1 * (1.0 - scaled_y) + c2 * scaled_y

        // Color(1., 1., 1.)
    }

    fn get_ray(&self, i: u64, j: u64) -> Ray {
        let origin = if self.defocus_angle <= 0. {
            self.center
        } else {
            self.defocus_disk_sample()
        };

        let offset = sample_square();
        // let offset = offset.0 * self.pixel_du + offset.1 * self.pixel_dv;
        let viewport_target = self.pixel_00_pos
            + self.pixel_du * (offset.0 + i as f64)
            + self.pixel_dv * (offset.1 + j as f64);
        let dir = viewport_target - origin;

        Ray { origin, dir }
    }

    fn defocus_disk_sample(&self) -> Pos {
        let p = Vec3::rand_in_unit_disk();
        self.center + p.0 * self.defocus_u + p.1 * self.defocus_v
    }
}

// return [-0.5, -0.5] - [0.5, 0.5]
fn sample_square() -> (f64, f64) {
    (fastrand::f64() - 0.5, fastrand::f64() - 0.5)
}
