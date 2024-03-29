use crate::get_shift_bits;
use crate::unicode::sequence_data::SequenceData;
use crate::unicode::utf32_encoder::{utf32_to_utf8_len, utf8char_to_utf32char};
use crate::unicode::utf8_sequence_collector::utf8_validate;
pub fn collect_utf32_sequences(byte: &Vec<u32>) -> SequenceData {
    unimplemented!()
}

use crate::unicode::error::*;
pub fn utf32_to_string(utf32_array: &Vec<u32>) -> Result<String, Box<UnicodeParseError>> {
    let mut utf8 = Vec::new();
    const FIRST_BYTES_OR: [u32; 4] = [0x00, 0xC0, 0xE0, 0xF0];
    for utf32 in utf32_array {
        let len = utf32_to_utf8_len(*utf32)?;
        let sh_bits = get_shift_bits!(len, 0);
        utf8.push((FIRST_BYTES_OR[len - 1] | (utf32 >> sh_bits)) as u8);
        for i in 0..len - 1 {
            let sh_bits = get_shift_bits!(len, i + 1);
            utf8.push((0x80 | ((utf32 >> sh_bits) & 0x3F)) as u8);
        }
    }
    Ok(String::from_utf8(utf8).unwrap())
}

pub fn byte_to_utf32(bytes: Vec<u8>) -> Result<Vec<u32>, Box<dyn std::error::Error>> {
    let mut i = 0;
    let mut v = Vec::new();
    while i < bytes.len() {
        let seq_info = utf8_validate(&bytes, i);
        let (len, valid) = seq_info.get_len_valid();
        // utf-8として問題ない場合は、utf8からutf32へ変換してVecへPushする
        // もし、異常なシーケンスを発見した場合は、即時エラーを返す。
        if valid {
            v.push(utf8char_to_utf32char(&bytes[i..i + len]));
        } else {
            return Err(Box::new(seq_info.get_error().unwrap()));
        }
        i += len;
    }
    Ok(v)
}

pub fn string_to_utf32(s: impl Into<String>) -> Result<Vec<u32>, Box<dyn std::error::Error>> {
    let s = s.into();
    let bytes = s.as_bytes().to_vec();
    byte_to_utf32(bytes)
}
