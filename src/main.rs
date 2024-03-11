#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_variables)]
// #![allow(unused_assignments)]

use proconio::input;

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
    let mut solver = DfsSolver::new(tile, score);
    let start = Point::new(si, sj);
    solver.dfs(start);
    println!("{}", solver.answer());
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
