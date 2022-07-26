use proconio::input;

fn main() {
  input! {
    s: String,
  }
  let s: Vec<char> = s.chars().collect();
  let mut cnt = 0;
  for i in 0..s.len() {
    if s[i] == '1' { cnt += 1; }
  }
  println!("{}", cnt);
}
