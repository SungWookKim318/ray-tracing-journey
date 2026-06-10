use crate::Color;
pub(crate) struct ImageData {
    width: usize,
    height: usize,
    pub data: Box<[Color]>,
}

impl ImageData {
    pub fn width(&self) -> usize {
        self.width
    }
    pub fn height(&self) -> usize {
        self.height
    }

    pub fn new(width: usize, height: usize) -> Self {
        let container = vec![Color::zero(); width * height];
        return Self {
            width: width,
            height: height,
            data: container.into_boxed_slice(),
        };
    }

    pub fn mock() -> ImageData {
        const WIDTH: f32 = 256.0;
        const HEIGHT: f32 = 256.0;
        let mut data = Box::new([Color::zero(); WIDTH as usize * HEIGHT as usize]);

        for (index, fragment) in data.iter_mut().enumerate() {
            let float_index = index as f32;
            let x = float_index % WIDTH;
            let y = float_index / WIDTH;

            fragment.x = 255.999 * (x / WIDTH);
            fragment.y = 255.999 * (y / HEIGHT);
            fragment.z = 0.0;
        }

        ImageData {
            width: WIDTH as usize,
            height: HEIGHT as usize,
            data: data,
        }
    }

    pub fn print_to_ppm(self) {
        println!("P3");
        println!("{} {}", self.width, self.height);
        println!("255");

        for line_fragments in self.data.chunks_exact(self.width) {
            for fragment in line_fragments {
                print!(
                    "{} {} {} ",
                    fragment.x as i32, fragment.y as i32, fragment.z as i32,
                );
            }
            print!("\n");
        }
        eprintln!("Finish print ppm data")
    }
}
