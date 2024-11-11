pub fn is_lowercase(s: &str) -> bool {
    s.chars().all(|c| c.is_ascii_lowercase())
}

pub fn is_uppercase(s: &str) -> bool {
    s.chars().all(|c| c.is_ascii_uppercase())
}

pub fn to_binary(s: &str) -> String {
    s.chars()
        .map(|c| format!("{:08b}", c as u8))
        .collect()
}

pub fn chunks(s: &str, size: usize) -> Vec<&str> {
    s.as_bytes()
        .chunks(size)
        .map(|chunk| std::str::from_utf8(chunk).unwrap())
        .collect()
} 