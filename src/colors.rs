/**
 * Copyright 2023 Thomas Hügel.
 * This file is part of Cargo Graphmod.
 * SPDX-License-Identifier: GPL-3.0-only
 */

pub fn make_gray(level: usize) -> String {
    let l = if level > 16 {
        0
    } else {
        15 - level
    } as u32;
    let c = String::from(char::from_digit(l, 16).unwrap_or('f')).repeat(2);
    String::from("#") + &c.repeat(3)
}

pub fn make_random_color(dirname: &str) -> String {
    let n: u32 = dirname.chars()
        .filter_map(|c| c.to_digit(36))
        .sum();
    let red = (255 - n * 71 % 128) as u8;
    let green = (255 - n * 131 % 128) as u8;
    let blue = (255 - n * 29 % 128) as u8;
    let number = u32::from_be_bytes([0, red, green, blue]);
    let hexadecimal = format!("{:#08x}", number);
    String::from("#") + &hexadecimal.chars().skip(2).collect::<String>()
}

#[cfg(test)]
mod tests {
    use crate::colors::{make_gray, make_random_color};

    #[test]
    fn it_makes_gray() {
        assert_eq!(String::from("#dddddd"), make_gray(2))
    }

    #[test]
    fn it_makes_a_random_color() {
        assert_eq!(String::from("#b6b2ec"), make_random_color("::foo::bar"))
    }
}