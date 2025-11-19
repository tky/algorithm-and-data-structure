fn main() {
    let steps = vec![2, 9, 4, 5, 1, 6, 10];
    let total_steps = calculate_total_steps(&steps);
    println!("Total steps: {}", total_steps);
}

fn calculate_total_steps(steps: &Vec<i32>) -> i32 {
    let len = steps.len();
    if len == 0 || len == 1 {
        return 0;
    }
    let mut dp = vec![0; len];
    dp[1] = (steps[1] - steps[0]).abs();
    if len == 2 {
        return dp[1];
    }

    for i in 2..len {
        // 飛飛ばして進む場合
        let s1 = dp[i - 2] + (steps[i] - steps[i - 2]).abs();
        // 1歩ずつ進む場合
        let s2 = dp[i - 1] + (steps[i] - steps[i - 1]).abs();
        dp[i] = s1.min(s2);
    }
    dp[len - 1]
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_calculate_total_steps() {
        let steps1 = vec![2, 9, 4, 5, 1, 6, 10];
        assert_eq!(calculate_total_steps(&steps1), 8);
    }
}
