use rand::RngExt;

use crate::vec3::Point3;

use super::noise::Noise;

const POINT_COUNT: usize = 256;
pub struct TrilinearPerlin {
    random_floats: [f32; POINT_COUNT],
    perm_x: [usize; POINT_COUNT],
    perm_y: [usize; POINT_COUNT],
    perm_z: [usize; POINT_COUNT],
}

impl Noise for TrilinearPerlin {
    fn noise(&self, position: Point3, scale: f32) -> f32 {
        let scaled_position = position * scale;
        let u = scaled_position.x - scaled_position.x.floor();
        let v = scaled_position.y - scaled_position.y.floor();
        let w = scaled_position.z - scaled_position.z.floor();

        let i = scaled_position.x.floor() as isize;
        let j = scaled_position.y.floor() as isize;
        let k = scaled_position.z.floor() as isize;

        let mut trilinear_samples = [0.0f32; 8];
        for (index, sample) in trilinear_samples.iter_mut().enumerate() {
            let dk = index as isize / 4 as isize;
            let dj = index as isize / 2 % 2 as isize;
            let di = index as isize % 2 as isize;
            let rand_index = self.perm_x[((i + di) & 255) as usize]
                ^ self.perm_y[((j + dj) & 255) as usize]
                ^ self.perm_z[((k + dk) & 255) as usize];
            *sample = self.random_floats[rand_index];
        }

        let new_value = trilinear_samples
            .iter()
            .enumerate()
            .fold(0.0f32, |acc, (index, value)| {
                let k = (index / 4) as f32;
                let j = (index / 2 % 2) as f32;
                let i = (index % 2) as f32;
                let sample_interpolated = (i * u + (1.0 - i) * (1.0 - u))
                    * (j * v + (1.0 - j) * (1.0 - v))
                    * (k * w + (1.0 - k) * (1.0 - w))
                    * value;
                acc + sample_interpolated
            });
        new_value
    }
}

impl TrilinearPerlin {
    pub const fn point_count() -> usize {
        POINT_COUNT
    }

    fn create_perlin_perm(rng: &mut dyn rand::Rng) -> [usize; POINT_COUNT] {
        let mut new_perm: [usize; POINT_COUNT] = std::array::from_fn(|index| index);
        Self::permute(rng, &mut new_perm, POINT_COUNT);
        new_perm
    }

    fn perlin_generate_perm(rng: &mut dyn rand::Rng, points: &mut [usize]) {
        for (index, point) in points.iter_mut().enumerate() {
            *point = index;
        }

        Self::permute(rng, points, POINT_COUNT);
    }

    fn permute(rng: &mut dyn rand::Rng, points: &mut [usize], max_value: usize) {
        for index in (1..max_value).rev() {
            let target = rng.random_range(0..=index);
            points.swap(index, target);
        }
    }
}

impl TrilinearPerlin {
    pub fn new(rng: &mut dyn rand::Rng) -> Self {
        Self {
            random_floats: std::array::from_fn(|_| rng.random_range(0.0..1.0)),
            perm_x: TrilinearPerlin::create_perlin_perm(rng),
            perm_y: TrilinearPerlin::create_perlin_perm(rng),
            perm_z: TrilinearPerlin::create_perlin_perm(rng),
        }
    }
}
