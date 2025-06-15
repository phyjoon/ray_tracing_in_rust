pub mod color;
pub mod ray;
pub mod vec3;
use color::write_color;
use ray::Ray;
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

fn ray_output_ppm_image<W: Write, E: Write>(
    out: &mut W,
    err: &mut E,
    pixel_width: usize,
    pixel_height: usize,
) -> io::Result<()> {
    // Setting up the camera parameters and world / pixel coordinate systems.
    // The x coordinate increases from left to right, and the y coordinate increases from bottom to top.
    // The u coordinate increases from left to right, and the v coordinate increases from top to bottom.
    let camera_origin = Vec3(0.0, 0.0, 0.0);
    let focal_length = 1.0;

    // The physical height of the image plane.
    let viewport_height = 2.0;
    // The physical width of the image plane.
    let viewport_width = viewport_height * (pixel_width as f64 / pixel_height as f64);

    // The 3D vector pointing 'right' along U.
    let viewport_horizontal = Vec3(viewport_width, 0.0, 0.0);
    // The 3D vector pointing 'down' along V.
    let viewport_vertical = Vec3(0.0, -viewport_height, 0.0);

    // World-space width covered by one pixel.
    let per_pixel_horizontal = viewport_horizontal / pixel_width as f64;
    // World-space height covered by one pixel.
    let per_pixel_vertical = viewport_vertical / pixel_height as f64;

    // World coordiantes of important viewport locations.
    let viewport_center = camera_origin - Vec3(0.0, 0.0, focal_length);
    let viewport_upper_left_corner =
        viewport_center - viewport_horizontal / 2.0 - viewport_vertical / 2.0;
    let viewport_first_pixel: Vec3 =
        viewport_upper_left_corner + per_pixel_vertical / 2.0 + per_pixel_horizontal / 2.0;

    writeln!(
        err,
        "Generating PPM image with dimensions {}x{}...",
        pixel_width, pixel_height
    )?;

    write!(out, "P3\n")?;
    write!(out, "{} {}\n", pixel_width, pixel_height)?;
    write!(out, "255\n")?;

    for j in 0..pixel_height {
        for i in 0..pixel_width {
            // write!(err, "\rScanlines remaining: {:5}", pixel_height - j)?;

            let viewport_pixel =
                viewport_first_pixel + per_pixel_horizontal * i as f64 + per_pixel_vertical * j as f64;
            let ray = Ray::new(camera_origin, viewport_pixel - camera_origin);

            let unit_direction = ray.direction().unit_vector();
            let param = unit_direction.1 * 0.5 + 0.5;
            let color = Vec3(1.0, 1.0, 1.0) * (1.0 - param) + Vec3(0.5, 0.7, 1.0) * param;

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

    let image_width = image_width as usize;

    let stdout = io::stdout();
    let stderr = io::stderr();
    let mut out = stdout.lock();
    let mut err = stderr.lock();

    ray_output_ppm_image(&mut out, &mut err, image_width, image_height)
        .expect("Failed to generate PPM image.");
}
