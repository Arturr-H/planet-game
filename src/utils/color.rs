macro_rules! hex {
    ($hex:expr) => {{
        let hex = $hex.trim_start_matches('#');
        let (r, g, b, a) = match hex.len() {
            6 => (
                u8::from_str_radix(&hex[0..2], 16).unwrap() as f32 / 255.,
                u8::from_str_radix(&hex[2..4], 16).unwrap() as f32 / 255.,
                u8::from_str_radix(&hex[4..6], 16).unwrap() as f32 / 255.,
                1., // Default alpha to fully opaque
            ),
            8 => (
                u8::from_str_radix(&hex[0..2], 16).unwrap() as f32 / 255.,
                u8::from_str_radix(&hex[2..4], 16).unwrap() as f32 / 255.,
                u8::from_str_radix(&hex[4..6], 16).unwrap() as f32 / 255.,
                u8::from_str_radix(&hex[6..8], 16).unwrap() as f32 / 255.,
            ),
            _ => panic!("Invalid hex format! Use 6 or 8 characters."),
        };
        Color::linear_rgba(r, g, b, a)
    }};
}

pub(crate) use hex;
