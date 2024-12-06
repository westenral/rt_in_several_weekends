mod color;
mod vec3;

use vec3::*;

fn main() {
    // image output
    let width = 255;
    let height = 255;
    println!("P3\n{width} {height}\n255");

    for y in 0..height {
        eprint!("\rLines remaining: {:>6}", height - y);
        for x in 0..width {
            println!(
                "{} {} {}",
                0x00,
                (((y as f64 / height as f64) * 255.0) as usize),
                (((x as f64 / width as f64) * 255.0) as usize)
            )
        }
    }
    eprintln!();
}
