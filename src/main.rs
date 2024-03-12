#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_variables)]
// #![allow(unused_assignments)]

use proconio::input;
use rand::Rng;

// ↓ → ↑ ←
const DIR: [(isize, isize); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
const DIR_STRING: [&str; 4] = ["D", "R", "U", "L"];
const H: usize = 50;
const W: usize = 50;

fn main() {
    input! {
        (si, sj): (usize, usize),
        tile: [[usize; W]; H],
        score: [[i32; W]; H],
    }
    let mut solver = DfsSolver::new(tile.clone(), score.clone());
    let start = Point::new(si, sj);
    solver.dfs(start);

    let seed = 334; // なんでや！
    let mut rng: rand::rngs::StdRng = rand::SeedableRng::seed_from_u64(seed);

    let mut loop_count = 0;
    let mut update_count = 0;

    let time_keeper = TimeKeeper::new(1.8);
    while !time_keeper.isTimeOver() {
        loop_count += 1;
        // 今のパスの中で壊す部分パスの長さ。
        let delete_path_length = rng.gen_range(1..solver.best_path.len() / 10);
        let start_path_id = rng.gen_range(1..solver.best_path.len() - delete_path_length);
        let end_path_id = start_path_id + delete_path_length;
        let remaining_search_cnt = 5 * delete_path_length;

        let mut now_seen = vec![false; H * W];
        let mut now_score = 0;
        for i in 0..solver.best_path.len() {
            let p = solver.best_path[i];
            if start_path_id <= i && i < end_path_id {
                now_score += score[p.0 as usize][p.1 as usize];
            } else {
                now_seen[solver.tile[p.0 as usize][p.1 as usize]] = true;
            }
        }

        let mut part_solver = DfsPartSolver::new(
            tile.clone(),
            score.clone(),
            solver.best_path[start_path_id],
            solver.best_path[end_path_id],
            now_seen,
            remaining_search_cnt,
        );
        part_solver.dfs(solver.best_path[start_path_id]);

        let next_score = part_solver.best_score;
        let diff = next_score - now_score;
        if part_solver.best_path.len() > 0 && diff > 0 {
            update_count += 1;
            solver.seen = part_solver.seen;

            let mut next_path = Vec::new();
            for i in 0..start_path_id {
                next_path.push(solver.best_path[i]);
            }
            for i in 0..part_solver.best_path.len() {
                next_path.push(part_solver.best_path[i]);
            }
            for i in end_path_id..solver.best_path.len() {
                next_path.push(solver.best_path[i]);
            }
            solver.best_path = next_path;
        }
    }
    println!("{}", solver.answer());
    #[cfg(feature = "local")]
    {
        eprintln!("loop  : {}", loop_count);
        eprintln!("update: {}", update_count);
    }
}

#[derive(Debug, Clone)]
struct TimeKeeper {
    start_time: std::time::Instant,
    time_threshold: f64,
}
impl TimeKeeper {
    fn new(time_threshold: f64) -> Self {
        TimeKeeper {
            start_time: std::time::Instant::now(),
            time_threshold,
        }
    }
    #[inline]
    fn isTimeOver(&self) -> bool {
        let elapsed_time = self.start_time.elapsed().as_nanos() as f64 * 1e-9;
        #[cfg(feature = "local")]
        {
            elapsed_time * 0.85 >= self.time_threshold
        }
        #[cfg(not(feature = "local"))]
        {
            elapsed_time >= self.time_threshold
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Point(isize, isize);
impl Point {
    fn new(y: usize, x: usize) -> Self {
        Point(y as isize, x as isize)
    }
}

struct DfsSolver {
    tile: Vec<Vec<usize>>,
    score: Vec<Vec<i32>>,
    seen: Vec<bool>,
    best_path: Vec<Point>,
    best_score: i32,
    now_path: Vec<Point>,
    now_score: i32,
    remaining_search_cnt: i32,
}
impl DfsSolver {
    fn new(tile: Vec<Vec<usize>>, score: Vec<Vec<i32>>) -> Self {
        Self {
            tile,
            score,
            seen: vec![false; H * W],
            best_path: Vec::new(),
            best_score: 0,
            now_path: Vec::new(),
            now_score: 0,
            remaining_search_cnt: 1_000_000,
        }
    }
    fn dfs(&mut self, point: Point) -> () {
        let (y, x) = (point.0, point.1);
        self.now_path.push(point);
        self.now_score += self.score[y as usize][x as usize];
        self.seen[self.tile[y as usize][x as usize]] = true;

        // スコアがよくなればパスを更新
        if self.now_score > self.best_score {
            self.best_score = self.now_score;
            self.best_path = self.now_path.clone();
        }

        // 残り探索回数を減らして終了判定
        self.remaining_search_cnt -= 1;
        if self.remaining_search_cnt == 0 {
            return;
        }

        // 次の場所を探す
        for k in 0..4 {
            let (ny, nx) = (y + DIR[k].0, x + DIR[k].1);
            if !self.in_field(ny, nx) {
                continue;
            }
            let (ny, nx) = (ny as usize, nx as usize);
            if self.seen[self.tile[ny][nx]] {
                continue;
            }
            self.dfs(Point::new(ny, nx));
            if self.remaining_search_cnt == 0 {
                return;
            }
        }

        self.seen[self.tile[y as usize][x as usize]] = false;
        self.now_score -= self.score[y as usize][x as usize];
        self.now_path.pop();
    }
    fn in_field(&self, i: isize, j: isize) -> bool {
        0 <= i && i < H as isize && 0 <= j && j < W as isize
    }
    fn answer(&self) -> String {
        let mut answer = String::new();
        for i in 0..self.best_path.len() - 1 {
            for k in 0..4 {
                if self.best_path[i + 1].0 - self.best_path[i].0 == DIR[k].0
                    && self.best_path[i + 1].1 - self.best_path[i].1 == DIR[k].1
                {
                    answer += DIR_STRING[k];
                }
            }
        }
        answer
    }
}
struct DfsPartSolver {
    tile: Vec<Vec<usize>>,
    score: Vec<Vec<i32>>,
    seen: Vec<bool>,
    best_path: Vec<Point>,
    best_score: i32,
    now_path: Vec<Point>,
    now_score: i32,
    remaining_search_cnt: usize,
    target: Point,
}
impl DfsPartSolver {
    fn new(
        tile: Vec<Vec<usize>>,
        score: Vec<Vec<i32>>,
        start: Point,
        target: Point,
        seen: Vec<bool>,
        remaining_search_cnt: usize,
    ) -> Self {
        Self {
            tile,
            score,
            seen,
            best_path: Vec::new(),
            best_score: 0,
            now_path: Vec::new(),
            now_score: 0,
            remaining_search_cnt,
            target,
        }
    }
    fn dfs(&mut self, point: Point) -> () {
        let (y, x) = (point.0 as usize, point.1 as usize);
        if !self.seen[self.tile[y][x]] {
            self.now_path.push(point);
            self.now_score += self.score[y][x];
            self.seen[self.tile[y][x]] = true;
        }

        self.remaining_search_cnt -= 1;
        if self.remaining_search_cnt == 0 {
            return;
        }
        if point != self.target {
            let (y, x) = (y as isize, x as isize);
            for k in (0..4).rev() {
                let (ny, nx) = (y + DIR[k].0, x + DIR[k].1);
                if !self.in_field(ny, nx) {
                    continue;
                }
                let (ny, nx) = (ny as usize, nx as usize);
                let next_point = Point::new(ny, nx);
                if next_point == self.target {
                    self.best_score = self.now_score;
                    self.best_path = self.now_path.clone();
                    self.remaining_search_cnt = 0;
                    return;
                }
                if self.seen[self.tile[ny][nx]] {
                    continue;
                }
                self.dfs(Point::new(ny, nx));
                if self.remaining_search_cnt == 0 {
                    return;
                }
            }
        }
        self.seen[self.tile[y][x]] = false;
        self.now_score -= self.score[y][x];
        self.now_path.pop();
    }
    fn in_field(&self, i: isize, j: isize) -> bool {
        0 <= i && i < H as isize && 0 <= j && j < W as isize
    }
}
