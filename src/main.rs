use std::env;

fn output_ppm_image(image_width: usize, image_height: usize) -> String {
    // Section 2.1 of `RayTracingInOneWeekend`
    // Generate a PPM image with a gradient from black
    let mut image_data = String::new();
    image_data.push_str("P3\n");
    image_data.push_str(&format!("{} {}\n", image_width, image_height));
    image_data.push_str("255\n");

    for j in 0..image_height {
        for i in 0..image_width {
            let r = (i as f64) / (image_width as f64 - 1.0);
            let g = (j as f64) / (image_height as f64 - 1.0);
            let b = 0.0;

            let [ir, ig, ib] = [r, g, b].map(|c| (255.999 * c).floor() as u8);

            image_data.push_str(&format!("{} {} {}\n", ir, ig, ib));
        }
    }

    image_data
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

    println!("{}", output_ppm_image(image_width, image_height));
}
