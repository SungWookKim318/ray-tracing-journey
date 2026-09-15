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
    fn noise(&self, position: Point3) -> f32 {
        const MAX_INDEX: i32 = POINT_COUNT as i32 - 1;
        let i = ((4.0 * position.x) as i32 & MAX_INDEX) as usize;
        let j = ((4.0 * position.y) as i32 & MAX_INDEX) as usize;
        let k = ((4.0 * position.z) as i32 & MAX_INDEX) as usize;

        self.random_floats[self.perm_x[i] ^ self.perm_y[j] ^ self.perm_z[k]]
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
