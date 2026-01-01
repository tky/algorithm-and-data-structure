fn main() {
    println!("Hello, world!");
}

/*
a := a_i
m := m_i
dp[0] = 0
dp[1..W] = -1

for s = 0..W:
  // iを使わなくてもすでにsを作れるのでsは制限回数まで使って良い
  if dp[s] >= 0:
      dp[s] = m
  // dp[s] < 0なのでi-1まででsを作ることはできていない
  // iを使ってsを作ることができるか？
  // d[s - a]であればiをつかってsを作ることができる

  // s < a => 今回のaを足してもsに届かないので作ることができない
  else if s < a
      dp[s] = -1

  // s - aは到達不能（作れない)のでaを足しても作れない
  else if dp[s-a] < 0
      dp[s] = -1

  // s - aは到達可能だが、aを使える回数が0なので作れない
  else if dp[s - a] = 0
      dp[s] = -1

  // このelseは
  // dp[s-a] > 0
  // つまり s - aに到達可能で、aの使用回数も残っている
  else:
      dp[s] = dp[s - a] - 1
 */
fn resolve(vs: &[usize], ms: &[usize], w: usize) -> bool {
    // dp[i] := iが作れるかどうか
    let mut dp = vec![-1isize; w + 1];

    // 0は必ず作れる
    dp[0] = 0;

    for (&v, &m) in vs.iter().zip(ms) {
        // println!("v: {}, m: {}", v, m);
        // このアイテムに切り替える前に、到達できているsはvを残りm回使える状態にする
        for s in 0..=w {
            if dp[s] >= 0 {
                dp[s] = m as isize;
            }
        }
        // println!("dp after reset: {:?}", dp);
        // sを小さい方から見ていく
        for s in 0..=w {
            // すでに作れる場合はスキップ
            if dp[s] >= 0 {
                continue;
            }
            // vでは作れない場合はスキップ
            if s < v {
                continue;
            }
            // s - vが作れるかつ、まだvを使える場合
            if dp[s - v] > 0 {
                // sを作って、vを1回使う
                dp[s] = dp[s - v] - 1;
            }
        }
        // println!("dp after v = {}: {:?}", v, dp);
    }

    dp[w] >= 0
}

// この方法だと制限を守れていない
fn resolve_ng(vs: &Vec<usize>, ms: &Vec<usize>, w: usize) -> bool {
    // dp[i] := iが作れるかどうか
    let mut dp = vec![false; w + 1];

    // 0は必ず作れる
    dp[0] = true;
    for i in 1..=w {
        for (&v, &m) in vs.iter().zip(ms) {
            if dp[i] {
                continue;
            }
            // vをm回使ってiが作れるかどうか
            dp[i] = (1..=m).any(|k| i >= (k * v) && dp[i - (k * v)]);
        }
    }
    dp[w]
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_resolve() {
        let vs = vec![5, 11];
        let ms = vec![2, 1];
        assert_eq!(resolve(&vs, &ms, 9), false);
        assert_eq!(resolve(&vs, &ms, 11), true);
        assert_eq!(resolve(&vs, &ms, 21), true);
        assert_eq!(resolve(&vs, &ms, 22), false);
        assert_eq!(resolve(&vs, &ms, 23), false);
    }
    #[test]
    fn test_break_early_is_wrong() {
        // 100を1回、3を2回
        let vs = vec![100, 3];
        let ms = vec![1, 2];

        assert_eq!(resolve(&vs, &ms, 103), true);
    }

    #[test]
    fn test_resolve_user_cases() {
        let vs = vec![5, 11];
        let ms = vec![2, 1];
        assert_eq!(resolve(&vs, &ms, 9), false);
        assert_eq!(resolve(&vs, &ms, 21), true);
        assert_eq!(resolve(&vs, &ms, 22), false);
        assert_eq!(resolve(&vs, &ms, 23), false);
    }
}
