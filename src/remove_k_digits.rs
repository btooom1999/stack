use std::collections::VecDeque;

fn remove_kdigits(num: String,mut k: i32) -> String {
    let mut stack = VecDeque::<u8>::new();
    for byte in num.as_bytes() {
        while k > 0 && stack.back().is_some_and(|v| v > byte) {
            stack.pop_back();
            k -= 1;
        }
        if *byte == b'0' && stack.back().is_none() {
            continue;
        }
        stack.push_back(*byte);
    }

    while k > 0 {
        stack.pop_back();
        k -= 1;
    }

    if stack.is_empty() {
        stack.push_back(b'0');
    }

    String::from_utf8(stack.into()).unwrap()
}

pub fn main() {
    let num = "112".to_string();
    let k = 1;
    println!("{}", remove_kdigits(num, k));
}
