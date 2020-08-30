use std;

fn next_permutation<T: copy>(v: [T]) -> option<[T]> {
  let n = vec::len(v) as int;
  let nv = vec::to_mut(v);
  if n < 2 {
    none
  }

  let i = n - 1;
  while i > 0 {
    i -= 1;
    if nv[i] < nv[i + 1] {
      let j = n - 1;
      while nv[i] >= nv[j] {
        j -= 1;
      }
      vec::swap(nv, i as uint, j as uint);
      // i+1~nまでswapする
      let low = i + 1;
      let high = n - 1;
      while low < high {
        vec::swap(nv, low as uint, high as uint);
        low += 1;
        high -= 1;
      }

      some(vec::from_mut(nv))
    }
  }

  none
}

fn permutation<T: copy>(v: [T], f: fn([T])) {
  f(v);
  let nv = next_permutation(v);
  while !option::is_none(nv) {
    let mv = option::get(nv);
    f(mv);
    nv = next_permutation(mv);
  }
}
