use crate::unicode::utf32_encoder::utf8char_to_utf32char;
use crate::unicode::utf8_sequence_collector::utf8_validate;

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
