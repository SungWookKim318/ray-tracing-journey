use rand::RngExt;

use crate::vec3::{Point3, Vec3};

use super::noise::Noise;

const POINT_COUNT: usize = 256;
pub struct TurbulencePerlin {
    perm_x: [usize; POINT_COUNT],
    perm_y: [usize; POINT_COUNT],
    perm_z: [usize; POINT_COUNT],
    random_vectors: [Vec3; POINT_COUNT],
    depth: i32,
}

impl Noise for TurbulencePerlin {
    fn noise(&self, position: Point3, scale: f32) -> f32 {
        let mut accum = 0.0;
        let mut temp_p = position;
        let mut weight = 1.0;

        (0..self.depth).for_each(|_| {
            accum += weight * self.get_noise(temp_p);
            weight *= 0.5;
            temp_p *= 2.0;
        });

        let turb = accum.abs(); // noise.turb(p,7)
        let phase_turb = scale * position.z + 10.0 * turb;
        //std::sin(scale * p.z() + 10 * noise.turb(p, 7))
        (1.0 + phase_turb.sin()) * 0.5
    }
}

impl TurbulencePerlin {
    pub const fn point_count() -> usize {
        POINT_COUNT
    }

    fn get_noise(&self, position: Point3) -> f32 {
        let u = position.x - position.x.floor();
        let v = position.y - position.y.floor();
        let w = position.z - position.z.floor();

        let i = position.x.floor() as isize;
        let j = position.y.floor() as isize;
        let k = position.z.floor() as isize;

        let vector_samples: [Vec3; 8] = std::array::from_fn(|index| {
            let dk = index as isize / 4 as isize;
            let dj = index as isize / 2 % 2 as isize;
            let di = index as isize % 2 as isize;
            let rand_index = self.perm_x[((i + di) & 255) as usize]
                ^ self.perm_y[((j + dj) & 255) as usize]
                ^ self.perm_z[((k + dk) & 255) as usize];
            self.random_vectors[rand_index]
        });

        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);
        let noise_value = vector_samples
            .iter()
            .enumerate()
            .fold(0.0f32, |acc, (index, value)| {
                let k = (index / 4) as f32;
                let j = (index / 2 % 2) as f32;
                let i = (index % 2) as f32;
                let weight_vector = Vec3::new(u - i, v - j, w - k);

                let sample_interpolated = (i * uu + (1.0 - i) * (1.0 - uu))
                    * (j * vv + (1.0 - j) * (1.0 - vv))
                    * (k * ww + (1.0 - k) * (1.0 - ww))
                    * value.dot(weight_vector);
                acc + sample_interpolated
            });
        noise_value
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

impl TurbulencePerlin {
    pub fn new(rng: &mut dyn rand::Rng) -> Self {
        Self {
            perm_x: TurbulencePerlin::create_perlin_perm(rng),
            perm_y: TurbulencePerlin::create_perlin_perm(rng),
            perm_z: TurbulencePerlin::create_perlin_perm(rng),
            random_vectors: std::array::from_fn(|_| Vec3::random_range(rng, -1.0, 1.0)),
            depth: 7,
        }
    }
    pub fn new_with_phase(rng: &mut dyn rand::Rng, depth: i32) -> Self {
        Self {
            perm_x: TurbulencePerlin::create_perlin_perm(rng),
            perm_y: TurbulencePerlin::create_perlin_perm(rng),
            perm_z: TurbulencePerlin::create_perlin_perm(rng),
            random_vectors: std::array::from_fn(|_| Vec3::random_range(rng, -1.0, 1.0)),
            depth,
        }
    }
}
