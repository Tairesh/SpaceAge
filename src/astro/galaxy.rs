#![allow(dead_code)]
use crate::geometry::DIR8;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::f32::consts::PI;

#[derive(Debug)]
pub enum GalaxyType {
    Spiral,
    BaredSpiral,
    Elliptical,
    Circular,
    Irregular,
}

pub fn generate(seed: u64, size: usize, typ: GalaxyType) -> Vec<u32> {
    let mut rng = StdRng::seed_from_u64(seed);
    let mut values = match typ {
        GalaxyType::Spiral => fill_spiral(&mut rng, size, true),
        GalaxyType::BaredSpiral => fill_bared_spiral(&mut rng, size),
        GalaxyType::Elliptical => fill_elliptical(&mut rng, size),
        GalaxyType::Circular => fill_circular(&mut rng, size),
        GalaxyType::Irregular => fill_irregular(&mut rng, size),
    };
    smooth(size, &mut values, 2);
    values
}

fn smooth(size: usize, values: &mut Vec<u32>, count: usize) {
    let slice = values.as_mut_slice();
    for _ in 0..count {
        for x in 0..size {
            for y in 0..size {
                let i = x * size + y;
                let val = slice[i] as f32;
                let nvals: f32 = DIR8
                    .iter()
                    .copied()
                    .map(|(dx, dy)| {
                        let j = (x as i32 + dx) * size as i32 + y as i32 + dy;
                        if j < 0 || j >= (size * size) as i32 {
                            0.0
                        } else {
                            slice[j as usize] as f32
                        }
                    })
                    .sum::<f32>()
                    / 8.0;
                let mut val = ((val + nvals) / 2.0).round() as u32;
                if val > CORE_MAX_STARS {
                    val = CORE_MAX_STARS;
                }
                slice[i] = val;
            }
        }
    }
}

// some magic numbers for logarithmic spiral
const TRIES_PER_SIZE: usize = 42;
const SPIRAL_A: f32 = 1.3;
const SPIRAL_B: f32 = 0.15;
const SPIRAL_WINDINGS: f32 = 3.6;
const SPIRAL_T_MAX: f32 = 2.0 * PI * SPIRAL_WINDINGS;
const SPIRAL_DRIFT: f32 = 0.3;

fn fill_spiral<R: Rng + ?Sized>(rng: &mut R, size: usize, with_core: bool) -> Vec<u32> {
    let mut values = vec![0; size * size];
    let slice = values.as_mut_slice();
    let tries = TRIES_PER_SIZE * size;
    let center = size / 2;
    for _ in 0..tries {
        let t = rng.gen_range(0.0..=1.0) * SPIRAL_T_MAX;
        let mut x = SPIRAL_A * (t * SPIRAL_B).exp() * t.cos() * size as f32 / 96.0;
        x += SPIRAL_DRIFT * x * rng.gen_range(0.0..=1.0)
            - SPIRAL_DRIFT * x * rng.gen_range(0.0..=1.0);
        let mut y = SPIRAL_A * (t * SPIRAL_B).exp() * t.sin() * size as f32 / 96.0;
        y += SPIRAL_DRIFT * y * rng.gen_range(0.0..=1.0)
            - SPIRAL_DRIFT * y * rng.gen_range(0.0..=1.0);
        let (x, y) = if rng.gen_bool(0.5) {
            (
                (x + center as f32).round() as i32,
                (y + center as f32).round() as i32,
            )
        } else {
            (
                (-x + center as f32).round() as i32,
                (-y + center as f32).round() as i32,
            )
        };
        if x < 0 || y < 0 || x > size as i32 - 1 || y > size as i32 - 1 {
            continue;
        }
        let i = x as usize * size + y as usize;
        slice[i] += rng.gen_range(1..150_000);
    }
    if with_core {
        fill_core(rng, size, &mut values);
    }
    values
}

const CORE_RADIUS_K: f32 = 0.02;
const CORE_MAX_STARS: u32 = 262_144;
const CORE_MIN_STARS: u32 = 50_000;

fn fill_core<R: Rng + ?Sized>(rng: &mut R, size: usize, values: &mut Vec<u32>) {
    let slice = values.as_mut_slice();
    let core_radius = (size as f32 * CORE_RADIUS_K).round() as usize + 1;
    let center = size / 2;
    for x in center - core_radius..=center + core_radius {
        for y in center - core_radius..=center + core_radius {
            let i = x * size + y;
            let d = f32::hypot(x as f32 - center as f32, y as f32 - center as f32);
            if d < CORE_RADIUS_K && slice[i] < 200_000 {
                slice[i] = (CORE_MAX_STARS as f32 * (1.0 - d * 5.0)).round() as u32
                    + rng.gen_range(0..CORE_MIN_STARS);
            }
        }
    }
}

fn fill_bared_spiral<R: Rng + ?Sized>(rng: &mut R, size: usize) -> Vec<u32> {
    let mut values = fill_spiral(rng, size, false);
    let slice = values.as_mut_slice();
    let core_radius = match size {
        64 => 5,
        128 => 8,
        256 => 15,
        _ => ((size as f32) * 0.05).round() as usize,
    };
    let center = size / 2;
    for x in size - center..=size + center {
        for y in size - center..=size + center {
            let d = f32::hypot(x as f32 - center as f32, y as f32 - center as f32);
            let dx = (x as f32 - y as f32).abs();
            if d + dx * 0.5 > core_radius as f32 {
                continue;
            }
            let i = x * size + y;
            let decr = (CORE_MAX_STARS as f32
                * (2.0 - (d + dx) / core_radius as f32)
                * rng.gen_range(0.0..=0.1))
            .round() as u32;
            if decr < slice[i] {
                slice[i] -= decr;
            } else {
                slice[i] = 0;
            }
            let d = d / core_radius as f32;
            if d > 1.0 {
                continue;
            }
            let dx = (x as f32 - center as f32).abs() / core_radius as f32;
            let mut val = 100_000.0 * (1.0 - dx) * rng.gen_range(0.0..=0.5);
            if dx < 0.3 {
                val += CORE_MAX_STARS as f32 * (0.3 - dx) * rng.gen_range(0.0..=0.1);
            }
            if d < 0.3 {
                val += CORE_MAX_STARS as f32 * (0.3 - d) * rng.gen_range(0.0..=0.1);
            }
            let val = if val < 0.0 { 0 } else { val.round() as u32 };
            slice[i] += val;
        }
    }
    values
}

const MAX_STARS_ELLIPTICAL: u32 = 120_000;

fn fill_elliptical<R: Rng + ?Sized>(rng: &mut R, size: usize) -> Vec<u32> {
    let mut values = vec![0; size * size];
    let slice = values.as_mut_slice();
    let center = size / 2;
    for x in 0..size {
        for y in 0..size {
            let d = f32::hypot(x as f32 - center as f32, y as f32 - center as f32) / size as f32;
            if d > 0.6 {
                continue;
            }
            let val =
                ((d * 2.0 * PI).cos() + rng.gen_range(0.0..=0.5)) * MAX_STARS_ELLIPTICAL as f32;
            let val = if val < 0.0 { 0 } else { val.round() as u32 };
            let i = x * size + y;
            slice[i] = val;
        }
    }
    values
}

const CIRCULAR_RADIUS: f32 = 0.2;
const MAX_STARS_CIRCULAR: u32 = 220_000;

fn fill_circular<R: Rng + ?Sized>(rng: &mut R, size: usize) -> Vec<u32> {
    let mut values = vec![0; size * size];
    let slice = values.as_mut_slice();
    let center = size / 2;
    for x in 0..size {
        for y in 0..size {
            let d = f32::hypot(x as f32 - center as f32, y as f32 - center as f32);
            let d_to_circle = (d - CIRCULAR_RADIUS).abs();
            let val = ((CIRCULAR_RADIUS / 4.0 - d_to_circle) * 10.0 + rng.gen_range(0.0..=0.5))
                * MAX_STARS_CIRCULAR as f32;
            let val = if val < 0.0 { 0 } else { val.round() as u32 };
            let i = x * size + y;
            slice[i] = val;
        }
    }
    values
}

const MAX_STARS_IRREGULAR: u32 = 120_000;

fn fill_irregular<R: Rng + ?Sized>(rng: &mut R, size: usize) -> Vec<u32> {
    let mut values = vec![0; size * size];
    let slice = values.as_mut_slice();
    let centers_count: usize = rng.gen_range(2..=4);
    let mut centers = Vec::with_capacity(centers_count);
    let clamps = ((size as f32 * 0.3) as usize, (size as f32 * 0.7) as usize);
    for _ in 0..centers_count {
        centers.push((
            rng.gen_range(clamps.0..=clamps.1),
            rng.gen_range(clamps.0..=clamps.1),
        ));
    }
    for (center_x, center_y) in centers {
        for x in 0..size {
            for y in 0..size {
                let d = f32::hypot(x as f32 - center_x as f32, y as f32 - center_y as f32)
                    / size as f32;
                if d > 0.3 {
                    continue;
                }
                let val = (0.3 - d) * 5.0 - rng.gen_range(0.0..=1.0) * MAX_STARS_IRREGULAR as f32;
                let val = if val < 0.0 { 0 } else { val.round() as u32 };
                let i = x * size + y;
                slice[i] = val;
            }
        }
    }
    let center = size / 2;
    for x in 0..size {
        for y in 0..size {
            let i = x * size + y;
            let d = f32::hypot(x as f32 - center as f32, y as f32 - center as f32);
            if d > 0.5 {
                slice[i] = 0
            } else {
                slice[i] = (slice[i] as f32 * (0.5 - d) * 2.0).round() as u32
            }
        }
    }
    values
}

#[cfg(test)]
mod tests {
    use crate::astro::galaxy::{generate, GalaxyType};

    #[test]
    fn test_spiral() {
        let spiral = generate(42, 128, GalaxyType::Spiral);
        assert_eq!(spiral.as_slice()[0], 0);
        dbg!(spiral.as_slice()[64 * 128 + 64]);
    }

    #[test]
    fn test_bared_spiral() {
        let bared = generate(42, 128, GalaxyType::BaredSpiral);
        assert_eq!(bared.as_slice()[0], 0);
        dbg!(bared.as_slice()[64 * 128 + 64]);
    }

    #[test]
    fn test_elliptic() {
        let elliptic = generate(42, 128, GalaxyType::Elliptical);
        assert_eq!(elliptic.as_slice()[0], 0);
        dbg!(elliptic.as_slice()[64 * 128 + 64]);
    }

    #[test]
    fn test_circular() {
        let circular = generate(42, 128, GalaxyType::Circular);
        assert_eq!(circular.as_slice()[0], 0);
        dbg!(circular.as_slice()[64 * 128 + 64]);
    }

    #[test]
    fn test_irregular() {
        let irregular = generate(42, 128, GalaxyType::Irregular);
        assert_eq!(irregular.as_slice()[0], 0);
        dbg!(irregular.as_slice()[64 * 128 + 64]);
    }
}