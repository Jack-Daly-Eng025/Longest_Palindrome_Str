pub struct Solution;
impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let s_v: Vec<char> = s.chars().collect();
        let s_l = s_v.len();
        if s_l == 0 {
            return "".to_string();
        }
        let (mut strt, mut end) = (0,0);
        for i in 0..s_l {
            let mut l_s = i;
            let mut r_s= i;
            while r_s+1<s_l && s_v[r_s+1]==s_v[l_s]{
                r_s+=1;
            }
            while r_s+1<s_l && l_s>0  && s_v[r_s+1] == s_v[l_s-1]{
                r_s+=1;
                l_s-=1;
            }
            if r_s-l_s > end -strt{
                end=r_s;
                strt=l_s;

            }
        }
        s_v[strt..=end].iter().collect()
    }
}


fn main(){
    let my_string = "ababababa".to_string();
    let my_other_string = "bababababab".to_string();

    let mut palindrome_longest = Solution::longest_palindrome(my_string);
    println!("{palindrome_longest}");
    palindrome_longest= Solution::longest_palindrome(my_other_string);
    println!("{palindrome_longest}");
}