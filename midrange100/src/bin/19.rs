use proconio::input;
use std::cmp::min;

fn main() {
    input! {
        d: usize,
        n: usize,
        q: usize,
        given: [usize; n-1],
        queries: [usize; q],
    }
    let mut shop = vec![];
    shop.push(0);
    for i in 0..n - 1 {
        shop.push(given[i]);
    }
    shop.push(d);
    shop.sort();

    let mut res = 0;
    for query in queries {
        let mut lo = 0;
        let mut hi = shop.len();
        let mid = (hi + lo) / 2;
        while hi - lo > 1 {
            if shop[mid] > query {
                hi = mid;
            } else {
                lo = mid;
            }
        }

        res += min(query - shop[lo], shop[hi] - query);
    }

    println!("{}", res);
}
