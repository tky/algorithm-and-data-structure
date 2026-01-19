use std::f64::consts::PI;

fn main() {
    println!("Hello, world!");
}

fn resolve(a: usize, b: usize, c: usize) -> f64 {
    let a = a as f64;
    let b = b as f64;
    let c = c as f64;

    let mut left = 0.0_f64;
    let mut right = 100_f64;

    let mut t: f64;

    loop {
        t = (left + right) / 2.0;
        let val = a * t + b * (c * t * PI).sin();

        if val < 100.0 {
            left = t;
        } else {
            right = t;
        }

        if diff(a, b, c, t) < 1e-6 {
            break;
        }
    }
    t
}

fn diff(a: f64, b: f64, c: f64, t: f64) -> f64 {
    let theta = c * t * PI;
    (a * t + b * theta.sin() - 100.0).abs()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_resolve() {
        let result = resolve(1, 1, 1);
        assert!(diff(1.0, 1.0, 1.0, result) < 1e-6);
    }

    #[test]
    fn test_resolve2() {
        let result = resolve(53, 82, 49);
        assert!(diff(53.0, 82.0, 49.0, result) < 1e-6);
    }
}
