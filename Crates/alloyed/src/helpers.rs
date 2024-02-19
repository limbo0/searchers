pub fn function_selector(selector_str: &str) -> [u8; 4] {
    let four_bytes_selector_hex = u32::from_str_radix(&selector_str[2..], 16).unwrap();

    let function_selector: [u8; 4] = [
        (four_bytes_selector_hex >> 24) as u8,
        ((four_bytes_selector_hex >> 16) & 0xFF) as u8,
        ((four_bytes_selector_hex >> 8) & 0xFF) as u8,
        (four_bytes_selector_hex & 0xFF) as u8,
    ];
    function_selector
}
