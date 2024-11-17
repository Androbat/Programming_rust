// The image crate provides functions for writing and reading variety
// of image formats. Includes encoders
use image::png::PNGEncoder;
use image::ColorType;
use std::fs::File;
use mandelbrot::src::parse_commands::parse_pair; // Check import later

fn write_image(
    filename: &str,
    pixels: &[u8],
    bounds: (usize, usize),
) -> Result<(), std::io::Error> {
    let output = File::create(filename)?;
    let encoder = PNGEncoder::new(output);
    encoder.encode(pixels, bounds.0 as u32, bounds.1 as u32, ColorType::Gray(8))?;

    Ok(())
}

/* 
 let output = match File::create(filename){
    Ok(f) => f,
    Err(e) => {
        return Err(e)
    }
 }

 The expression above is equivalent to using "?" => let output = File::create(filename)?;

*/

fn main() {

}