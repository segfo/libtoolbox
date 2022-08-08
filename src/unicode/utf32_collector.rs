use crate::get_shift_bits;
use crate::unicode::sequence_data::*;
use crate::unicode::utf32_encoder::utf8char_to_utf32char;
use crate::unicode::utf8_sequence_collector::utf8_validate;
// pub fn collect_utf32_sequences(byte: &Vec<u32>) -> SequenceData {
//     let mut i = 0;
//     let mut seqdata_list = Vec::new();
//     let mut utf8_seq = Vec::new();
//     let mut bin_seq = Vec::new();

//     while i < byte.len() {
//         let info = utf8_validate(byte, i);
//         let (len, valid) = info.get_len_valid();
//         if valid {
//             // 有効なシーケンスがあったら記録していく
//             if bin_seq.len() > 0 {
//                 seqdata_list.push(DataSequence::ByteSequence(bin_seq.clone()));
//                 bin_seq.truncate(0);
//             }
//             for off in 0..len {
//                 utf8_seq.push(byte[i + off]);
//             }
//         } else {
//             // 有効なutf8シーケンスではない
//             // 今までに収集された有効なUTF-8シーケンスがあれば、シーケンスをStringにして保存する。
//             if utf8_seq.len() > 0 {
//                 seqdata_list.push(DataSequence::Utf8Sequence(
//                     String::from_utf8(utf8_seq.clone()).unwrap(),
//                 ));
//                 utf8_seq.truncate(0);
//             }
//             // 有効でないシーケンスもとりあえず保存しておく
//             for off in 0..len {
//                 bin_seq.push(byte[i + off]);
//             }
//         }
//         i += len;
//     }
//     if utf8_seq.len() > 0 {
//         seqdata_list.push(DataSequence::Utf8Sequence(
//             String::from_utf8(utf8_seq.clone()).unwrap(),
//         ));
//     }
//     if bin_seq.len() > 0 {
//         seqdata_list.push(DataSequence::ByteSequence(bin_seq.clone()));
//     }
//     SequenceData::collect_sequence_data(seqdata_list)
// }

fn utf32_to_utf8_len(utf32: u32) -> Result<usize, Box<UnicodeParseError>> {
    match utf32 {
        0..=0x7F => Ok(1),
        0x0000_0080..=0x0000_07FF => Ok(2),
        // 0x0000_D800-0x0000_DFFFはサロゲートペアのコードポイントのため2バイトコードではない
        0x0000_0800..=0x0000_D7FF | 0x0000_E000..=0x0000_FFFF => Ok(3),
        0x0001_0000..=0x0010_FFFF => Ok(4),
        _ => {
            return Err(Box::new(UnicodeParseError::new(
                UnicodeParseErrorKind::IllegalRange,
            )));
        }
    }
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
