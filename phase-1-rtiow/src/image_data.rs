use crate::color::Color;
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

    pub fn new(new_width: usize, new_height: usize) -> Self {
        let container = vec![Color::zero(); new_width * new_height];
        Self {
            width: new_width,
            height: new_height,
            data: container.into_boxed_slice(),
        }
    }

    pub fn mock() -> ImageData {
        const WIDTH: f32 = 256.0;
        const HEIGHT: f32 = 256.0;
        let mut image_data = Box::new([Color::zero(); WIDTH as usize * HEIGHT as usize]);

        for (index, fragment) in image_data.iter_mut().enumerate() {
            let float_index = index as f32;
            let x = float_index % WIDTH;
            let y = float_index / WIDTH;

            fragment.x = x / WIDTH;
            fragment.y = y / HEIGHT;
            fragment.z = 0.0;
        }

        ImageData {
            width: WIDTH as usize,
            height: HEIGHT as usize,
            data: image_data,
        }
    }

    pub fn print_to_ppm(self) {
        println!("P3");
        println!("{} {}", self.width, self.height);
        println!("255");

        for line_fragments in self.data.chunks_exact(self.width) {
            for fragment in line_fragments {
                fragment.write_color()
            }
            println!();
        }
        eprintln!("Finish print ppm data")
    }
}
