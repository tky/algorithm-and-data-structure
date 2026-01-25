use superslice::*;

fn main() {
    println!("Hello, world!");
}

fn resolve(xs: &mut [usize], ys: &mut [usize]) -> usize {
    xs.sort_unstable();
    ys.sort_unstable();

    let mut ys = ys;
    let mut result = 0;

    for x in xs {
        let idx = ys.upper_bound(x);
        if idx == ys.len() {
            break;
        }
        result += 1;
        ys = &mut ys[idx + 1..];
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve() {
        let mut xs = vec![1, 2, 3];
        let mut ys = vec![2, 2, 4];
        assert_eq!(resolve(&mut xs, &mut ys), 2);
    }
}
