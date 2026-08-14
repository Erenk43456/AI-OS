pub fn translate_with_shift(
    scancode: u8,
    shift: bool,
) -> Option<char> {
    match scancode {
        // ====================================================
        // NUMBER ROW — TURKISH Q
        // ====================================================

        // 1 !
        0x02 => Some(if shift { '!' } else { '1' }),

        // 2 "
        0x03 => Some(if shift { '"' } else { '2' }),

        // 3 ^
        0x04 => Some(if shift { '^' } else { '3' }),

        // 4 +
        0x05 => Some(if shift { '+' } else { '4' }),

        // 5 %
        0x06 => Some(if shift { '%' } else { '5' }),

        // 6 &
        0x07 => Some(if shift { '&' } else { '6' }),

        // 7 /
        0x08 => Some(if shift { '/' } else { '7' }),

        // 8 (
        0x09 => Some(if shift { '(' } else { '8' }),

        // 9 )
        0x0A => Some(if shift { ')' } else { '9' }),

        // 0 =
        0x0B => Some(if shift { '=' } else { '0' }),

        // * ?
        0x0C => Some(if shift { '?' } else { '*' }),

        // - _
        0x0D => Some(if shift { '_' } else { '-' }),

        // ====================================================
        // Q ROW
        // ====================================================

        0x10 => Some(if shift { 'Q' } else { 'q' }),
        0x11 => Some(if shift { 'W' } else { 'w' }),
        0x12 => Some(if shift { 'E' } else { 'e' }),
        0x13 => Some(if shift { 'R' } else { 'r' }),
        0x14 => Some(if shift { 'T' } else { 't' }),
        0x15 => Some(if shift { 'Y' } else { 'y' }),
        0x16 => Some(if shift { 'U' } else { 'u' }),

        // ı / I
        0x17 => Some(if shift { 'I' } else { 'ı' }),

        0x18 => Some(if shift { 'O' } else { 'o' }),
        0x19 => Some(if shift { 'P' } else { 'p' }),

        // ğ / Ğ
        0x1A => Some(if shift { 'Ğ' } else { 'ğ' }),

        // ü / Ü
        0x1B => Some(if shift { 'Ü' } else { 'ü' }),

        // ====================================================
        // HOME ROW
        // ====================================================

        0x1E => Some(if shift { 'A' } else { 'a' }),
        0x1F => Some(if shift { 'S' } else { 's' }),
        0x20 => Some(if shift { 'D' } else { 'd' }),
        0x21 => Some(if shift { 'F' } else { 'f' }),
        0x22 => Some(if shift { 'G' } else { 'g' }),
        0x23 => Some(if shift { 'H' } else { 'h' }),
        0x24 => Some(if shift { 'J' } else { 'j' }),
        0x25 => Some(if shift { 'K' } else { 'k' }),
        0x26 => Some(if shift { 'L' } else { 'l' }),

        // ş / Ş
        0x27 => Some(if shift { 'Ş' } else { 'ş' }),

        // i / İ
        0x28 => Some(if shift { 'İ' } else { 'i' }),

        // " / é
        0x29 => Some(if shift { 'é' } else { '"' }),

        // ====================================================
        // COMMA
        // ====================================================

        // , / ;
        0x2B => Some(if shift { ';' } else { ',' }),

        // ====================================================
        // BOTTOM ROW
        // ====================================================

        0x2C => Some(if shift { 'Z' } else { 'z' }),
        0x2D => Some(if shift { 'X' } else { 'x' }),
        0x2E => Some(if shift { 'C' } else { 'c' }),
        0x2F => Some(if shift { 'V' } else { 'v' }),
        0x30 => Some(if shift { 'B' } else { 'b' }),
        0x31 => Some(if shift { 'N' } else { 'n' }),
        0x32 => Some(if shift { 'M' } else { 'm' }),

        // ö / Ö
        0x33 => Some(if shift { 'Ö' } else { 'ö' }),

        // ç / Ç
        0x34 => Some(if shift { 'Ç' } else { 'ç' }),

        // . / :
        0x35 => Some(if shift { ':' } else { '.' }),

        // ====================================================
        // SPACE
        // ====================================================

        0x39 => Some(' '),

        // ====================================================
        // UNKNOWN
        // ====================================================

        _ => None,
    }
}