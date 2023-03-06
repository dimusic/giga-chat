pub fn hex_to_rgb(hex: &str) -> (u32, u32, u32) {
    let mut c: Vec<char> = hex.chars().collect();
    if c.len() == 4 {
        c = vec![c[1], c[1], c[2], c[2], c[3], c[3]];
    }
    let c: String = c.into_iter().collect();
    let c = u32::from_str_radix(&c, 16).unwrap();

    ((c >> 16) & 255, (c >> 8) & 255, c & 255)
}
