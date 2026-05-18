fn main() {
    let str = String::from("aba");
    println!("Is this String: {str} a Palindrome -> {}",check_palindrome(&str));
}

fn check_palindrome(str: &String) -> bool {
    let size: usize = str.len();
    if str.chars().nth(0) != str.chars().nth(size-1) {
        return false;
    } else {
        let mut reverse_str = String::new();
        for c in str.chars().rev() {
            reverse_str.push(c);
        }
        let return_value:bool = if str == &reverse_str { true } else { false };

        return return_value;
    }
}
