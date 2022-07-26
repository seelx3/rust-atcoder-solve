use proconio::input;

fn main() {
  input! {
    n: u32,
    a: [u32; n],
  }
  let mut ans = 0xFFFFFFFF;
  for n in a {
    ans = std::cmp::min(ans, n.trailing_zeros());
  }
  println!("{}", ans);
}
