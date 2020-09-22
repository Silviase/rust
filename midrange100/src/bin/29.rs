use proconio::{fastout, input, marker::Chars, marker::Usize1};
use std::collections::VecDeque;

#[fastout]
fn main() {
    input! {
        h:usize,
        w:usize,
        s:(Usize1, Usize1),
        g:(Usize1, Usize1),
        b:[Chars; h],
    }

    let dx: [i32; 4] = [-1, 1, 0, 0];
    let dy: [i32; 4] = [0, 0, -1, 1];
    let mut cnt = vec![vec![-1; w]; h];
    let mut q = VecDeque::new();
    q.push_back(s);
    cnt[s.0][s.1] = 0;
    while !q.is_empty() {
        let now = q.pop_front().unwrap();
        for i in 0..4 {
            let nxt = ((now.0 as i32 + dx[i]), now.1 as i32 + dy[i]);
            if nxt.0 < 0 || nxt.0 >= h as i32 || nxt.1 < 0 || nxt.1 >= w as i32 {
                continue;
            }
            if cnt[nxt.0 as usize][nxt.1 as usize] == -1 && b[nxt.0 as usize][nxt.1 as usize] == '.'
            {
                cnt[nxt.0 as usize][nxt.1 as usize] = cnt[now.0][now.1] + 1;
                q.push_back((nxt.0 as usize, nxt.1 as usize));
            }
        }
    }

    println!("{}", cnt[g.0][g.1]);
}
