use nom::{
    branch::{alt, permutation},
    bytes::{
        complete::{tag, take_till, take_while},
        streaming::take_until,
    },
    character::complete::{alphanumeric1, digit0, digit1, none_of},
    combinator::opt,
    number::complete::{be_u16, be_u32, be_u64, be_u8},
    sequence::{separated_pair, terminated},
    IResult,
};

#[derive(Debug)]
enum Data {
    U64(u64),
    U32(u32),
    U16(u16),
    U8(u8),
}

fn parse_big_endian(raw: &[u8]) -> IResult<&[u8], Data> {
    let r = match raw.len() {
        2..=3 => {
            let (raw, v) = be_u16(raw)?;
            (raw, Data::U16(v))
        }
        1 => {
            let (raw, v) = be_u8(raw)?;
            (raw, Data::U8(v))
        }
        4..=7 => {
            let (raw, v) = be_u32(raw)?;
            (raw, Data::U32(v))
        }
        _ => {
            let (raw, v) = be_u64(raw)?;
            (raw, Data::U64(v))
        }
    };
    Ok(r)
}

#[test]
fn run() {
    use nom::number::complete::le_u32;
    let mut raw = &[
        0xef, 0xbe, 0xad, 0xde, 0xef, 0xbe, 0xad, 0xde, 0xef, 0xbe, 0xad, 0xde, 0x00, 0x55, 0xAA,
    ][..];
    while raw.len() != 0 {
        let (raw0, v) = parse_big_endian(&raw).unwrap();
        raw = raw0;
        println!("{:?},{:?}", v, raw);
    }
}

use super::error::{UnicodeParseError, UnicodeParseErrorKind};
pub struct Utf16SequenceInfo {
    len: usize,
    valid: bool,
    error: Option<UnicodeParseError>,
}
impl Utf16SequenceInfo {
    fn new(len: usize, valid: bool) -> Self {
        Utf16SequenceInfo {
            len: len,
            valid: valid,
            error: None,
        }
    }
    /**
     * 使い方
     * ```
     * use toolbox::unicode::utf16_sequence_collector::{
     *     Utf16SequenceInfo,
     *     utf16_validate
     * };
     * // 4バイトコードの文字
     * let s = "🍺".as_bytes().to_vec();
     * let result = utf16_validate(&s, 0);
     * // ↓↓↓使い方↓↓↓
     * let (len, valid) = result.get_len_valid();
     * assert_eq!(len, 2); // 長さが4バイト
     * assert_eq!(valid, true); // UTF-16シーケンスとして有効かどうか
     * ```
     */
    pub fn get_len_valid(&self) -> (usize, bool) {
        (self.len, self.valid)
    }
    pub fn set_error(&mut self, error: UnicodeParseError) {
        self.error = Some(error)
    }
    pub fn get_error(&self) -> Option<UnicodeParseError> {
        self.error.clone()
    }
}
pub fn utf16_validate(byte_array: &Vec<u8>, offset: usize) -> Utf16SequenceInfo {
    // Vec<u8>をVec<u16>へ変換したい。
    // 条件に合わない場合はバイトオーダーを逆転させるとかの処理をしたい。
    // ライブラリねーかな。あった→byteorder:https://crates.io/crates/byteorder
    let byte = byte_array[offset];
    let i = offset;
    // 不正なUTF-8エンコードかどうかを確認する
    let validate_encoding = |seq: &[u8]| -> Option<UnicodeParseError> {
        let second = seq[1];
        let first = seq[0];
        let error = match (first, second) {
            (0xE0, 0x80..=0x9F) => Some(UnicodeParseErrorKind::RedundantEncoding), // 冗長な符号化
            (0xF0, 0x80..=0x8F) => Some(UnicodeParseErrorKind::RedundantEncoding), // 冗長な符号化
            (0xED, 0xA0..=0xFF) => Some(UnicodeParseErrorKind::IllegalCodePoint), // サロゲートペアの符号位置
            (0xF4, 0x90..=0xFF) => Some(UnicodeParseErrorKind::IllegalRange),     // Unicodeの範囲外
            (_, _) => None,
        };
        if error.is_some() {
            Some(UnicodeParseError::new(error.unwrap()))
        } else {
            None
        }
    };
    unimplemented!()
}
