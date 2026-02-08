fn main() {
    println!("Hello, world!");
}

// (所要時間, 締切)
type Pair = (i32, i32);

// 締切の早い順にソートして仕事が終終わる判定する
fn resolve(ns: &mut [Pair]) -> bool {
    ns.sort_unstable_by_key(|x| x.1);

    let mut acc = 0;
    for &(a, b) in ns.iter() {
        // 締切までに終わらない場合はearly return
        acc += a;
        if acc > b {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve() {
        let mut ns1 = vec![(2, 4), (1, 9), (1, 8), (4, 9), (3, 12)];
        assert_eq!(resolve(&mut ns1), true);

        let mut ns2 = vec![(334, 1000), (334, 1000), (334, 1000)];
        assert_eq!(resolve(&mut ns2), false);
    }
}
