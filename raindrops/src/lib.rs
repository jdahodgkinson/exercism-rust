pub fn raindrops(n: u32) -> String {
    let mut noise = String::new();
    if n % 3 == 0 {
        noise.push_str("Pling");
    }
    if n % 5 == 0 {
        noise.push_str("Plang");
    }
    if n % 7 == 0 {
        noise.push_str("Plong")
    }
    if noise.is_empty() {
        noise.push_str(&n.to_string())
    }
    noise
}
