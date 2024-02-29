#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_variables)]

use proconio::input;

// ↓ → ↑ ←
const IDY: [isize; 4] = [1, 0, -1, 0];
const IDX: [isize; 4] = [0, 1, 0, -1];
const IDIR: [&str; 4] = ["D", "R", "U", "L"];

fn main() {
    let (H, W) = (50, 50);
    input! {
        (si, sj): (isize, isize),
        tile: [[usize; W]; H],
        score: [[i32; W]; H],
    }
    let start = Point(si, sj);
    let M = 1 + *tile
        .iter()
        .map(|row| row.iter().max().unwrap())
        .max()
        .unwrap();
    let mut dfs = Dfs::new(tile, score, M);
    let answer = dfs.exec(start);
    println!("{}", answer);
}

struct Point(isize, isize);

struct Dfs {
    H: usize,
    W: usize,
    tile: Vec<Vec<usize>>,
    score: Vec<Vec<i32>>,
    seen: Vec<bool>,
}
impl Dfs {
    fn new(tile: Vec<Vec<usize>>, score: Vec<Vec<i32>>, M: usize) -> Self {
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
        for k in 0..4 {
            let (ny, nx) = (sy + IDY[k], sx + IDX[k]);
            if !self.in_field(ny, nx) {
                continue;
            }
            let (ny, nx) = (ny as usize, nx as usize);
            // let (sy, sx) = (sy as usize, sx as usize);
            if self.seen[self.tile[ny][nx]] {
                continue;
            }
            self.seen[self.tile[ny][nx]] = true;
            answer += IDIR[k];
            score += self.score[ny][nx];
            break;
        }
        eprintln!("score : {}", score);
        answer
    }
    fn in_field(&self, i: isize, j: isize) -> bool {
        0 <= i && i < self.H as isize && 0 <= j && j < self.W as isize
    }
}
