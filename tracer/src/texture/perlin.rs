use glam::Vec3A;
use rand::Rng;
static POINT_COUNT: usize = 256;

pub struct Perlin {
    pub ran: Vec<Vec3A>,
    pub perm_x: Vec<i32>,
    pub perm_y: Vec<i32>,
    pub perm_z: Vec<i32>,
}

impl Perlin {
    pub fn new() -> Self {
        let mut ran = Vec::with_capacity(POINT_COUNT);
        let mut rng = rand::thread_rng();
        for i in 0..256 {
            ran.push(rng.gen::<Vec3A>().normalize());
        }

        Self {
            ran,
            perm_x: Self::perlin_generate_perm(),
            perm_y: Self::perlin_generate_perm(),
            perm_z: Self::perlin_generate_perm(),
        }
    }

    fn perlin_generate_perm() -> Vec<i32> {
        let mut p = Vec::with_capacity(POINT_COUNT);
        for i in 0..256 {
            p.push(i);
        }
        Self::permute(&mut p);

        p
    }

    fn permute(p: &mut Vec<i32>) {
        for i in (1..POINT_COUNT).rev() {
            let target = (rand::random::<f32>() * (i as f32 + 1.0)) as usize;
            let tmp = p[i];
            p[i] = p[target];
            p[target] = tmp;
        }
    }

    pub fn turb(&self, mut p: Vec3A, depth: i32) -> f32 {
        let mut accum = 0.0;
        let mut weight = 1.0;
        for _ in 0..depth {
            accum += weight * self.noise(p);
            weight *= 0.5;
            p *= 2.0;
        }
        accum.abs()
    }

    pub fn noise(&self, p: Vec3A) -> f32 {
        let u = p.x - p.x.floor();
        let v = p.y - p.y.floor();
        let w = p.z - p.z.floor();

        // hermitian smoothing
        let u = u * u * (3.0 - 2.0 * u);
        let v = v * v * (3.0 - 2.0 * v);
        let w = w * w * (3.0 - 2.0 * w);

        let i = p.x.floor() as i32;
        let j = p.y.floor() as i32;
        let k = p.z.floor() as i32;
        let mut c = [[[Vec3A::ZERO; 2]; 2]; 2];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di as usize][dj as usize][dk as usize] = self.ran[(self.perm_x
                        [((i + di) & 255) as usize]
                        ^ self.perm_y[((j + dj) & 255) as usize]
                        ^ self.perm_z[((k + dk) & 255) as usize])
                        as usize];
                }
            }
        }

        Self::trilinear_interp(c, u, v, w)
    }

    pub fn trilinear_interp(c: [[[Vec3A; 2]; 2]; 2], u: f32, v: f32, w: f32) -> f32 {
        let mut accum = 0.0;
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    accum += (i as f32 * u + (1 - i) as f32 * (1.0 - u))
                        * (j as f32 * v + (1 - j) as f32 * (1.0 - v))
                        * (k as f32 * w + (1 - k) as f32 * (1.0 - w))
                        * Vec3A::dot(
                            c[i][j][k],
                            Vec3A::new(u - i as f32, v - j as f32, w - k as f32),
                        );
                }
            }
        }

        accum
    }
}
