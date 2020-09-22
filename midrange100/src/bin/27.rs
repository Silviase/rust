use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        m: usize,
        n: usize,
        grid: [[usize; m];n]
    }
    let dp = [(1, 0), (0, 1), (-1, 0), (0, -1)];

    let mut max_depth = 0;
    for y in 0..n {
        for x in 0..m {
            if grid[y][x] == 0 {
                continue;
            }
            let mut stack = vec![];
            stack.push(((y as isize, x as isize), vec![]));
            while let Some((pos, mut path)) = stack.pop() {
                path.push(pos);

                let mut is_exist_next = false;
                for &(dy, dx) in &dp {
                    let (ny, nx) = (pos.0 + dy, pos.1 + dx);
                    if nx >= 0
                        && nx < m as isize
                        && ny >= 0
                        && ny < n as isize
                        && grid[ny as usize][nx as usize] == 1
                    {
                        if !path.iter().any(|&(yy, xx)| yy == ny && xx == nx) {
                            is_exist_next = true;
                            stack.push(((ny, nx), path.clone()));
                        }
                    }
                }

                if !is_exist_next {
                    max_depth = max_depth.max(path.len());
                }
            }
        }
    }
    println!("{}", max_depth);
}
