fn main() {
    // (weight ,value)
    let items = vec![(2, 3), (1, 2), (3, 6), (2, 1), (1, 3), (5, 8)];
    let capacity = 5;
    let result = napsack(&items, capacity);
    println!("最大価値: {}", result);
}

fn chmax<T: PartialOrd + Copy>(a: &mut T, b: T) {
    if *a < b {
        *a = b;
    }
}

fn napsack(items: &Vec<(usize, usize)>, capacity: usize) -> usize {
    let mut dp: Vec<Vec<usize>> = vec![vec![0; capacity + 1]; items.len() + 1];

    // dp[0][w]は0なので1から始める
    for i in 1..=items.len() {
        for w in 0..=capacity {
            let (weight, value) = items[i - 1];
            // iを選んだ場合
            // 選んだ後にdp[i][w]になるならば、選ぶ前はdp[i-1][w-weight[i])の状態
            // ただしwを選んでcapacityを超える場合は選べない
            if w >= weight {
                let add = dp[i - 1][w - weight] + value;
                chmax(&mut dp[i][w], add);
            }
            // iを選ばなかった場合
            // 選ばなかった後にdp[i][w]になるならば、選ぶ前はdp[i-1][w]の状態
            let not = dp[i - 1][w];
            chmax(&mut dp[i][w], not);
        }
    }
    dp[items.len()][capacity]
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_napsack() {
        let items = vec![(2, 3), (1, 2), (3, 6), (2, 1), (1, 3), (5, 8)];
        let capacity = 5;
        assert_eq!(napsack(&items, capacity), 11);
    }
}
