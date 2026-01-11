fn main() {
    println!("Hello, world!");
}

// どの2要素も互いに異なる整数列
// i= 0, 1,,,N-1に対して、ajが全体の中で何番目に小さい値であるかを求める
// たとえばa = 12, 43, 7, 15, 9の時、答えは[2, 4, 0 , 3, 1]となる

// vsをsortして元ののインデックスを返す
fn resolve(vs: &[usize]) -> Vec<usize> {
    // sort: O(N log N)
    let mut sorted = vec![0; vs.len()];
    sorted.clone_from_slice(vs);
    sorted.sort();

    // これだとsortでO(N log N)、positionでO(N)かかるのでO(N^2)になってしまう
    /*
       vs.iter()
           .map(|&v| sorted.iter().position(|&s| s == v).unwrap())
           .collect()
    */
    // binary_search O(N log N)
    vs.iter()
        .map(|&v| sorted.binary_search(&v).unwrap())
        .collect()

    // 全体で
    // O(N log N) + O(N log N) = O(N log N)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_resolve() {
        let vs = vec![12, 43, 7, 15, 9];
        let result = resolve(&vs);
        assert_eq!(result, vec![2, 4, 0, 3, 1]);
    }
}
