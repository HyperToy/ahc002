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
        tile: [[i32; W]; H],
        score: [[i32; W]; H],
    }
    let start = Point(si, sj);
    let mut answer = String::new();
    for k in 0..4 {
        let (ny, nx) = (start.0 + IDY[k], start.1 + IDX[k]);
        if ny < 0 || H <= ny || nx < 0 || W <= nx {
            continue;
        }
        let (ny, nx) = (ny as usize, nx as usize);
        if tile[ny][nx] == tile[start.0 as usize][start.1 as usize] {
            continue;
        }
        answer += IDIR[k];
        break;
    }
    println!("{}", answer);
}

struct Point(isize, isize);
