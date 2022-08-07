use crate::unicode::error::{UnicodeParseError, UnicodeParseErrorKind};
use crate::unicode::sequence_data::*;
pub struct Utf8SequenceInfo {
    len: usize,
    valid: bool,
    error: Option<UnicodeParseError>,
}
impl Utf8SequenceInfo {
    fn new(len: usize, valid: bool) -> Self {
        Utf8SequenceInfo {
            len: len,
            valid: valid,
            error: None,
        }
    }
    /**
     * 使い方
     * ```
     * use toolbox::unicode::utf8_sequence_collector::{
     *     Utf8SequenceInfo,
     *     utf8_validate
     * };
     * // 4バイトコードの文字
     * let s = "🍺".as_bytes().to_vec();
     * let result = utf8_validate(&s, 0);
     * // ↓↓↓使い方↓↓↓
     * let (len, valid) = result.get_len_valid();
     * assert_eq!(len, 4); // 長さが4バイト
     * assert_eq!(valid, true); // UTF-8シーケンスとして有効かどうか
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

pub fn collect_utf8_sequences(byte: &Vec<u8>) -> SequenceData {
    let mut i = 0;
    let mut seqdata_list = Vec::new();
    let mut utf8_seq = Vec::new();
    let mut bin_seq = Vec::new();

    while i < byte.len() {
        let info = utf8_validate(byte, i);
        let (len, valid) = (info.len, info.valid);
        if valid {
            // 有効なシーケンスがあったら記録していく
            if bin_seq.len() > 0 {
                seqdata_list.push(DataSequence::ByteSequence(bin_seq.clone()));
                bin_seq.truncate(0);
            }
            for off in 0..len {
                utf8_seq.push(byte[i + off]);
            }
        } else {
            // 有効なutf8シーケンスではない
            // 今までに収集された有効なUTF-8シーケンスがあれば、シーケンスをStringにして保存する。
            if utf8_seq.len() > 0 {
                seqdata_list.push(DataSequence::Utf8Sequence(
                    String::from_utf8(utf8_seq.clone()).unwrap(),
                ));
                utf8_seq.truncate(0);
            }
            // 有効でないシーケンスもとりあえず保存しておく
            for off in 0..len {
                bin_seq.push(byte[i + off]);
            }
        }
        i += len;
    }
    if utf8_seq.len() > 0 {
        seqdata_list.push(DataSequence::Utf8Sequence(
            String::from_utf8(utf8_seq.clone()).unwrap(),
        ));
    }
    if bin_seq.len() > 0 {
        seqdata_list.push(DataSequence::ByteSequence(bin_seq.clone()));
    }
    SequenceData::collect_sequence_data(seqdata_list)
}

// indexの位置にあるバイトデータをUTF-8の1文字目と仮定して扱う。
// 明らかに1文字目でもないしUTF-8のルールに違反している場合はバイナリデータとして扱うように(n,false)を返す。
// なお、n>=1とする。
pub fn utf8_validate(byte_array: &Vec<u8>, offset: usize) -> Utf8SequenceInfo {
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
    // バリデーションを担う
    let validation = |seq_len, byte: &Vec<u8>, index| -> Utf8SequenceInfo {
        let len = byte.len();
        let i = index;
        match seq_len {
            0 => {
                let mut info = Utf8SequenceInfo::new(1, false);
                info.set_error(UnicodeParseError::new(UnicodeParseErrorKind::IllegalRange));
                return info;
            } // utf-8でも無くAsciiでもない
            1 => return Utf8SequenceInfo::new(1, true), // Asciiだった
            _ => { /* 後続の処理を行うのでここには書かない */ }
        }
        if (len - i) >= seq_len {
            // 2バイト以降の値が範囲外でないかを検証する。
            for off in 1..seq_len {
                if !(0x80 <= byte_array[off + i] && byte_array[off + i] < 0xBF) {
                    let mut r = Utf8SequenceInfo::new(seq_len, false);
                    r.set_error(UnicodeParseError::new(UnicodeParseErrorKind::IllegalRange));
                    return r;
                }
            }
            // 一応2バイト以降がutf-8シーケンスっぽかったので
            // 今度は1バイト目も含めて、2バイト目以降が正しくエンコードされていそうか検証する。
            let error = validate_encoding(&byte[i..i + seq_len]);
            if error.is_some() {
                let mut r = Utf8SequenceInfo::new(seq_len, false);
                r.set_error(error.unwrap());
                r
            } else {
                Utf8SequenceInfo::new(seq_len, true)
            }
        } else if len > i {
            // 残りのデータ配列全体の長さが、指定されたシーケンスの長さよりも短い
            let mut r = Utf8SequenceInfo::new(len - i, false);
            r.set_error(UnicodeParseError::new(
                UnicodeParseErrorKind::IllegalByteSequence,
            ));
            r
        } else {
            // あり得ないが、一応処理を入れておく。
            panic!("illegal index");
        }
    };
    let condition = |first, start: u8, end: u8| -> bool { start <= first && first <= end };

    struct NumberRange {
        start: u8,
        end: u8,
    }
    let range = [
        NumberRange {
            start: 0xF0,
            end: 0xF4,
        },
        NumberRange {
            start: 0xE0,
            end: 0xEF,
        },
        NumberRange {
            start: 0xC2,
            end: 0xDF,
        },
        NumberRange {
            start: 0x00,
            end: 0x7f,
        },
    ];
    for seq_len in 0..=range.len() - 1 {
        if condition(byte, range[seq_len].start, range[seq_len].end) {
            // シーケンス長 = range.len() - seq_len
            return validation(range.len() - seq_len, byte_array, i);
        }
    }
    // バイナリでしか表現できない場合は、以下が実行される
    validation(0, byte_array, i)
}
