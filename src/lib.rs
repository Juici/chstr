#![no_std]

/// Converts a `char` array into a constant `&str`.
///
/// # Examples
///
/// Basic usage:
/// ```
/// # use chstr::chstr;
/// const ABC: &str = chstr!['a', 'b', 'c'];
///
/// assert_eq!(ABC, "abc");
/// ```
///
/// Directory separator:
/// ```
/// # use chstr::chstr;
/// const SEPARATOR_CHAR: char = std::path::MAIN_SEPARATOR;
/// const SEPARATOR: &str = chstr![SEPARATOR_CHAR];
///
/// let mut chars = SEPARATOR.chars();
/// assert_eq!(chars.next(), Some(SEPARATOR_CHAR));
/// assert_eq!(chars.next(), None);
/// ```
#[macro_export]
macro_rules! chstr {
    [$($c:expr),* $(,)?] => {
        {
            const CHARS: &[char] = &[$($c),*];
            const N: usize = CHARS.len();

            const LEN: usize = {
                let mut len = 0;

                let mut i = 0;
                while i < N {
                    let c = CHARS[i];
                    len += c.len_utf8();
                    i += 1;
                }

                len
            };

            const BUF: [u8; LEN] = {
                // UTF-8 ranges and tags for encoding characters.
                const TAG_CONT: u8 = 0b1000_0000;
                const TAG_TWO_B: u8 = 0b1100_0000;
                const TAG_THREE_B: u8 = 0b1110_0000;
                const TAG_FOUR_B: u8 = 0b1111_0000;

                let mut buf = [0; LEN];
                let mut offset = 0;

                let mut i = 0;
                while i < N {
                    let c = CHARS[i];
                    let code = c as u32;
                    let len = c.len_utf8();

                    match len {
                        1 => {
                            buf[offset + 0] = code as u8;
                        }
                        2 => {
                            buf[offset + 0] = (code >> 6 & 0x1F) as u8 | TAG_TWO_B;
                            buf[offset + 1] = (code & 0x3F) as u8 | TAG_CONT;
                        }
                        3 => {
                            buf[offset + 0] = (code >> 12 & 0x0F) as u8 | TAG_THREE_B;
                            buf[offset + 1] = (code >> 6 & 0x3F) as u8 | TAG_CONT;
                            buf[offset + 2] = (code & 0x3F) as u8 | TAG_CONT;
                        }
                        4 => {
                            buf[offset + 0] = (code >> 18 & 0x07) as u8 | TAG_FOUR_B;
                            buf[offset + 1] = (code >> 12 & 0x3F) as u8 | TAG_CONT;
                            buf[offset + 2] = (code >> 6 & 0x3F) as u8 | TAG_CONT;
                            buf[offset + 3] = (code & 0x3F) as u8 | TAG_CONT;
                        }
                        _ => ::core::unreachable!(),
                    }
                    offset += len;

                    i += 1;
                }

                buf
            };

            unsafe { ::core::str::from_utf8_unchecked(&BUF) }
        }
    };
}
