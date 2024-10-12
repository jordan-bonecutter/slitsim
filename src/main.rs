use uom::si::f64::Length;
use uom::si::length::nanometer;
use image::{RgbImage, Rgb};

fn main() {
    let num_slits = 10;
    let wavelength = Length::new::<nanometer>(300.);
    let slit_distance = wavelength * 3.;
    let sim_width = wavelength * 1000.;
    let sim_height = wavelength * 500.;
    let length_per_pix = wavelength / 3.;
    let width_pix = (sim_width / length_per_pix).value as usize;
    let height_pix = (sim_height / length_per_pix).value as usize;
    let mut grid = vec![0.0; width_pix * height_pix];

    let mut min = None;
    let mut max = None;
    for idx in 0..grid.len() {
        let x: Length = (((idx % width_pix) as f64)*length_per_pix) - ((width_pix as f64) / 2.)*length_per_pix;
        let y: Length = ((height_pix - 1) as f64*length_per_pix) - (((idx / width_pix) as f64)*length_per_pix);

        let value = (0..num_slits).fold(0f64, |acc, slit_idx| {
            let slit_y = Length::new::<nanometer>(0.);
            let slit_x = (slit_idx as f64 - (num_slits as f64 / 2.)) * slit_distance;
            let dist_y = (y - slit_y)*(y - slit_y);
            let dist_x = (x - slit_x)*(x - slit_x);
            let dist = (dist_y + dist_x).sqrt();
            let phase = std::f64::consts::TAU * (dist / wavelength).value;
            return acc + phase.cos();
        });
        grid[idx] = value;

        match min {
            None => min = Some(value),
            Some(v) => {
                if v > value {
                    min = Some(value);
                }
            }
        }
        match max {
            None => max = Some(value),
            Some(v) => {
                if v < value {
                    max = Some(value);
                }
            }
        }
    }

    let scale = min.unwrap().abs().max(max.unwrap());
    let mut img = RgbImage::new(width_pix as u32, height_pix as u32);
    for y in 0..height_pix {
        for x in 0..width_pix {
            let grid_idx = x + (y*width_pix);
            let v = grid[grid_idx];
            img.put_pixel(x as u32, y as u32, Rgb(if v > 0. {
                [((v / scale)*255.) as u8, 0, 0]
            } else {
                [0, 0, ((-v / scale)*255.) as u8]
            }));
        }
    }

    img.save("out.png").unwrap();
}
