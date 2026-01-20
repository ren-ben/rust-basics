pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

pub fn sum(nums: &[i32]) -> i32 {
    nums.iter().sum()
}

pub fn flip(b: bool) -> bool {
    !b
}

// ownership & borrowing
pub fn take_ownership(s: String) -> usize {
    s.len()
}

pub fn borrow_first_char(s: &str) -> Option<char> {
    s.chars().next()
}

pub fn push_exclamation(s: &mut String) {
    s.push('!');
}
