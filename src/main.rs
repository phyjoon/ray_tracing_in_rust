pub mod color;
pub mod vec3;
use color::write_color;
use std::env;
use std::io::{self, Write};
use vec3::Vec3;

fn output_ppm_image<W: Write, E: Write>(
    out: &mut W,
    err: &mut E,
    image_width: usize,
    image_height: usize,
) -> io::Result<()> {
    // Section 2.1 of `RayTracingInOneWeekend`
    // Generate a PPM image with a gradient from black

    writeln!(
        err,
        "Generating PPM image with dimensions {}x{}...",
        image_width, image_height
    )?;

    write!(out, "P3\n")?;
    write!(out, "{} {}\n", image_width, image_height)?;
    write!(out, "255\n")?;

    for j in 0..image_height {
        for i in 0..image_width {
            write!(err, "\rScanlines remaining: {:5}", image_height - j)?;

            let color = Vec3::new(
                (i as f64) / (image_width as f64 - 1.0),
                (j as f64) / (image_height as f64 - 1.0),
                0.0,
            );

            write_color(out, &color)?;
        }
    }

    writeln!(err, "\nPPM image generation complete.")?;

    Ok(())
}

fn main() {
    let args: Vec<usize> = env::args()
        .skip(1)
        .map(|arg| {
            arg.parse()
                .expect("Each argument must be a non-negative integer.")
        })
        .collect();

    let &[image_width, image_height] = args.get(0..2).unwrap_or(&[256, 256]) else {
        panic!("Please provide two arguments for image width and height.");
    };

    let stdout = io::stdout();
    let stderr = io::stderr();
    let mut out = stdout.lock();
    let mut err = stderr.lock();

    output_ppm_image(&mut out, &mut err, image_width, image_height)
        .expect("Failed to generate PPM image.");
}
