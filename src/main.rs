#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_variables)]

use proconio::input;

// ↓ → ↑ ←
const DIR: [(isize, isize); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
const DIR_STRING: [&str; 4] = ["D", "R", "U", "L"];

fn main() {
    let (H, W) = (50, 50);
    input! {
        (si, sj): (isize, isize),
        tile: [[usize; W]; H],
        score: [[i32; W]; H],
    }
    let mut dfs = Dfs::new(tile, score);
    let start = Point(si, sj);
    let answer = dfs.exec(start);
    println!("{}", answer);
}

#[derive(PartialEq, Eq)]
struct Point(isize, isize);

struct Dfs {
    H: usize,
    W: usize,
    tile: Vec<Vec<usize>>,
    score: Vec<Vec<i32>>,
    seen: Vec<bool>,
}
impl Dfs {
    fn new(tile: Vec<Vec<usize>>, score: Vec<Vec<i32>>) -> Self {
        let M = 1 + *tile
            .iter()
            .map(|row| row.iter().max().unwrap())
            .max()
            .unwrap();
        Self {
            H: tile.len(),
            W: tile[0].len(),
            tile,
            score,
            seen: vec![false; M],
        }
    }
    fn exec(&mut self, start: Point) -> String {
        let (sy, sx) = (start.0, start.1);
        self.seen[self.tile[sy as usize][sx as usize]] = true;
        let mut answer = String::new();
        let mut score = self.score[sy as usize][sx as usize];

        let mut best_dir = 0;
        let mut best_score = 0;
        for k in 0..4 {
            let (ny, nx) = (sy + DIR[k].0, sx + DIR[k].1);
            if !self.in_field(ny, nx) {
                continue;
            }
            let (ny, nx) = (ny as usize, nx as usize);
            if self.seen[self.tile[ny][nx]] {
                continue;
            }
            if self.score[ny][nx] > best_score {
                best_score = self.score[ny][nx];
                best_dir = k;
            }
        }
        let (ny, nx) = (
            (sy + DIR[best_dir].0) as usize,
            (sx + DIR[best_dir].1) as usize,
        );
        self.seen[self.tile[ny][nx]] = true;
        answer += DIR_STRING[best_dir];
        score += self.score[ny][nx];

        eprintln!("score : {}", score);
        answer
    }
    fn in_field(&self, i: isize, j: isize) -> bool {
        0 <= i && i < self.H as isize && 0 <= j && j < self.W as isize
    }
}
