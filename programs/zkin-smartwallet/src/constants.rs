pub const COLON_ASCII: u8 = 58;
pub const QUOTE_ASCII: u8 = 34;
pub const COMMA_ASCII: u8 = 44;

pub fn is_special_char(c: u8) -> bool {
  c == COLON_ASCII || c == QUOTE_ASCII || c == COMMA_ASCII
}
