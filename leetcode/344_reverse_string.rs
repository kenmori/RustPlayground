// https://leetcode.com/problems/reverse-string/
// https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=%0Afn+swap%28s%3A+%26mut+Vec%3Cchar%3E%2C+l%3A+usize%2C+r%3A+usize%29+%7B%0A++++if+l+%3E%3D+r+%7B%0A++++++++return%3B%0A++++%7D%0A++++s.swap%28l%2C+r%29%3B%0A++++swap%28s%2C+l+%2B+1%2C+r+-+1%29%3B%0A%7D%0A%0Afn+main%28%29+%7B%0A++++let+mut+str+%3D+vec%21%5B%27h%27%2C+%27e%27%2C+%27l%27%2C+%27l%27%2C+%27o%27%5D%3B%0A%0A++++match+str.len%28%29+%7B%0A++++++++0+%3D%3E+%28%29%2C%0A++++++++l+%3D%3E+swap%28%26mut+str%2C+0%2C+l+-+1%29%2C%0A++++%7D%0A++++println%21%28%22%7B%3A%3F%7D%22%2C+str%29%3B%0A%7D%0A

impl Solution {
    fn swap(s: &mut Vec<char>, l: usize, r: usize) {
        if l >= r {
            return;
        }
        s.swap(l, r);
        Solution::swap(s, l + 1, r - 1);
    }
}

fn main() {
    let mute str = vec!['h', 'e', 'l', 'l', 'o'];

    match str.len() {
        0 => (),
        l => Solution::swap(&str, 0, l - 1),
    }
}
