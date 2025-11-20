fn main() {
    let steps = vec![2, 9, 4, 5, 1, 6, 10];
    let total_steps = calculate_total_steps(&steps);
    println!("Total steps: {}", total_steps);
}

fn chmin<T: PartialOrd + Copy>(a: &mut T, b: T) {
    if *a > b {
        *a = b;
    }
}

fn calculate_total_steps(steps: &Vec<i32>) -> i32 {
    let len = steps.len();
    if len == 0 || len == 1 {
        return 0;
    }
    let mut dp = vec![i32::MAX; len];
    dp[0] = 0;
    dp[1] = (steps[1] - steps[0]).abs();
    if len == 2 {
        return dp[1];
    }

    for i in 2..len {
        // 飛び飛ばして進む合
        let v1 = dp[i - 2] + (steps[i] - steps[i - 2]).abs();
        chmin(&mut dp[i], v1);

        let v2 = dp[i - 1] + (steps[i] - steps[i - 1]).abs();
        chmin(&mut dp[i], v2);
    }
    dp[len - 1]
}

fn calculate_total_steps2(steps: &Vec<i32>) -> i32 {
    let len = steps.len();
    if len == 0 || len == 1 {
        return 0;
    }

    let mut dp = vec![i32::MAX; len];
    dp[0] = 0;
    for i in 0..len {
        if i + 1 < len {
            let cost = dp[i] + (steps[i] - steps[i + 1]).abs();
            chmin(&mut dp[i + 1], cost);
        }
        if i + 2 < len {
            let cost = dp[i] + (steps[i] - steps[i + 2]).abs();
            chmin(&mut dp[i + 2], cost);
        }
    }
    dp[len - 1]
}

fn calculate_total_step3(steps: &Vec<i32>, i: usize) -> i32 {
    if i == 0 {
        return 0;
    }
    if i == 1 {
        return (steps[1] - steps[0]).abs();
    }

    let cost1 = calculate_total_step3(steps, i - 1) + (steps[i] - steps[i - 1]).abs();
    let cost2 = calculate_total_step3(steps, i - 2) + (steps[i] - steps[i - 2]).abs();
    return cost1.min(cost2);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_calculate_total_steps() {
        let steps1 = vec![2, 9, 4, 5, 1, 6, 10];
        assert_eq!(calculate_total_steps(&steps1), 8);
        assert_eq!(calculate_total_steps2(&steps1), 8);
        assert_eq!(calculate_total_step3(&steps1, steps1.len() - 1), 8);
    }
}
