// https://atcoder.jp/contests/arc005/tasks/arc005_3

use std::collections::VecDeque;

// 通路: 0, 壁: 1
type Maze = Vec<Vec<usize>>;

type Point = (usize, usize);

const DIRECTIONS: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

fn resolve(maze: &Maze, start: Point, goal: Point) -> usize {
    let width = maze[0].len();
    let height = maze.len();

    let mut queue: VecDeque<Point> = VecDeque::new();
    // 対象地点までに通る壁の数
    let mut distance = vec![vec![usize::MAX; width]; height];

    queue.push_back(start);
    distance[start.1][start.0] = 0;

    while let Some((x, y)) = queue.pop_front() {
        for (dx, dy) in DIRECTIONS {
            let x = x as isize;
            let y = y as isize;
            let width = width as isize;
            let height = height as isize;
            if x + dx >= 0 && x + dx < width && y + dy >= 0 && y + dy < height {
                let nx = (x + dx) as usize;
                let ny = (y + dy) as usize;
                let cost = maze[ny][nx];
                let next = cost + distance[y as usize][x as usize];
                if next < distance[ny][nx] {
                    distance[ny][nx] = next;
                    if cost == 0 {
                        queue.push_front((nx, ny));
                    } else {
                        queue.push_back((nx, ny));
                    }
                }
            }
        }
    }
    distance[goal.1][goal.0]
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn can_reach(maze: &Maze, start: Point, goal: Point) -> bool {
        resolve(maze, start, goal) <= 2
    }

    #[test]
    fn start_equals_goal() {
        let maze = vec![vec![0]];
        assert_eq!(resolve(&maze, (0, 0), (0, 0)), 0);
        assert!(can_reach(&maze, (0, 0), (0, 0)));
    }

    #[test]
    fn no_wall_needed() {
        let maze = vec![vec![0, 0, 0], vec![0, 0, 0], vec![0, 0, 0]];
        assert_eq!(resolve(&maze, (0, 0), (2, 2)), 0);
        assert!(can_reach(&maze, (0, 0), (2, 2)));
    }

    #[test]
    fn one_wall_needed() {
        let maze = vec![vec![0, 1, 0]];
        assert_eq!(resolve(&maze, (0, 0), (2, 0)), 1);
        assert!(can_reach(&maze, (0, 0), (2, 0)));
    }

    #[test]
    fn two_walls_needed() {
        let maze = vec![vec![0, 1, 1, 0]];
        assert_eq!(resolve(&maze, (0, 0), (3, 0)), 2);
        assert!(can_reach(&maze, (0, 0), (3, 0)));
    }

    #[test]
    fn three_walls_needed() {
        let maze = vec![vec![0, 1, 1, 1, 0]];
        assert_eq!(resolve(&maze, (0, 0), (4, 0)), 3);
        assert!(!can_reach(&maze, (0, 0), (4, 0)));
    }

    #[test]
    fn detour_is_better_than_breaking_walls() {
        // まっすぐ進むと壁を 3 回壊す必要があるが、
        // 下に回り込めば 0 回で到達できる
        let maze = vec![vec![0, 1, 1, 1, 0], vec![0, 0, 0, 0, 0]];
        assert_eq!(resolve(&maze, (0, 0), (4, 0)), 0);
        assert!(can_reach(&maze, (0, 0), (4, 0)));
    }

    #[test]
    fn must_choose_path_with_fewer_breaks_even_if_longer() {
        // 距離は遠回りだが、壁破壊数はそちらの方が少ない
        let maze = vec![vec![0, 1, 1, 0], vec![0, 0, 0, 0], vec![1, 1, 1, 0]];
        assert_eq!(resolve(&maze, (0, 0), (3, 0)), 0);
        assert!(can_reach(&maze, (0, 0), (3, 0)));
    }

    #[test]
    fn goal_is_a_wall_cell_in_internal_representation() {
        // 本番入力では s/g は通常道として扱うが、
        // resolve 単体の性質確認として、goal が壁ならその分コストがかかる
        let maze = vec![vec![0, 1]];
        assert_eq!(resolve(&maze, (0, 0), (1, 0)), 1);
        assert!(can_reach(&maze, (0, 0), (1, 0)));
    }
}
