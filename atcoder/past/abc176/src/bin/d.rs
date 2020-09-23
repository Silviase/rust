use num::abs;
use proconio::input;
use proconio::marker::{Chars, Isize1};
use std::collections::VecDeque;

fn bfs(
    h: isize,
    w: isize,
    field: Vec<Vec<char>>,
    si: isize,
    sj: isize,
    ti: isize,
    tj: isize,
) -> isize {
    let mut q = VecDeque::<(isize, isize, isize)>::new();
    q.push_back((0, si, sj));
    let mut visited = vec![vec![false; w as usize]; h as usize];
    let mut queued = vec![vec![false; w as usize]; h as usize];
    queued[si as usize][sj as usize] = true;

    while !q.is_empty() {
        let (cost, i, j) = q.pop_front().unwrap();
        if i == ti && j == tj {
            return cost;
        };
        if visited[i as usize][j as usize] {
            continue;
        };
        visited[i as usize][j as usize] = true;
        for ni in i as isize - 2..=i + 2 {
            if ni < 0 || h <= ni {
                continue;
            }
            for nj in j as isize - 2..=j + 2 {
                if nj < 0
                    || w <= nj
                    || field[ni as usize][nj as usize] == '#'
                    || queued[ni as usize][nj as usize]
                {
                    continue;
                }
                match abs(ni - i) + abs(nj - j) {
                    0 => (),
                    1 => {
                        q.push_front((cost, ni, nj));
                        queued[ni as usize][nj as usize] = true;
                    }
                    _ => q.push_back((cost + 1, ni, nj)),
                }
            }
        }
    }

    -1
}

fn main() {
    input! {
        h: isize,
        w: isize,
        ch: Isize1,
        cw: Isize1,
        dh: Isize1,
        dw: Isize1,
        field: [Chars; h],
    }
    let ans = bfs(h, w, field, ch, cw, dh, dw);
    println!("{}", ans);
}
