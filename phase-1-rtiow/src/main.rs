#![allow(dead_code)]
mod color;
mod vec3;
use color::Color;
use vec3::Vec3;

fn main() {
    eprintln!("Start RT");
    let dummy = ImageData::mock();
    dummy.print_to_ppm();
}

struct ImageData {
    width: u32,
    height: u32,
    pixel_per_byte: u8,
    data: Box<[u8]>,
}

impl ImageData {
    fn mock() -> ImageData {
        const WIDTH: usize = 256;
        const HEIGHT: usize = 256;
        const PIXEL_PER_BYTE: usize = 3;
        const RECT: usize = WIDTH * HEIGHT;
        let mut data = Box::new([0; WIDTH * HEIGHT * PIXEL_PER_BYTE]);

        for (index, fragment) in data.chunks_exact_mut(PIXEL_PER_BYTE).enumerate() {
            let [r, g, b]: &mut [u8; PIXEL_PER_BYTE] = fragment.try_into().unwrap();
            let x = index % WIDTH;
            let y = index / WIDTH;

            *r = (255.999 * (x as f32 / WIDTH as f32)) as u8;
            *g = (255.999 * (y as f32 / HEIGHT as f32)) as u8;
            *b = 0;
        }

        ImageData {
            width: WIDTH as u32,
            height: HEIGHT as u32,
            pixel_per_byte: PIXEL_PER_BYTE as u8,
            data: data,
        }
    }

    fn print_to_ppm(self) {
        println!("P3");
        println!("{} {}", self.width, self.height);
        println!("255");

        for line_fragments in self.data.chunks_exact(self.width as usize) {
            for fragment in line_fragments {
                print!("{} ", fragment);
            }
            print!("\n");
        }
        eprintln!("Finish print ppm data")
    }
}
