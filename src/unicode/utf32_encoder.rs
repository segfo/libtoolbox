use super::error::{UnicodeParseError, UnicodeParseErrorKind};

/**
 * 4バイト文字の場合は以下のような使い方と結果になる。
 * ```
 * use toolbox::get_shift_bits;
 * let seq_len = 4;
 * let mut actual = Vec::new();
 * for i in 0..seq_len{
 *  let shift_bits = get_shift_bits!(seq_len,i);
 *  actual.push(shift_bits);
 * }
 * assert_eq!([18,12,6,0].to_vec(), actual);
 * ```
 * 2バイト文字の場合は以下のような使い方と結果になる。
 * ```
 * use toolbox::get_shift_bits;
 * let seq_len = 2;
 * let mut actual = Vec::new();
 * for i in 0..seq_len{
 *  let shift_bits = get_shift_bits!(seq_len,i);
 *  actual.push(shift_bits);
 * }
 * assert_eq!([6,0].to_vec(), actual);
 * ```
 */
#[macro_export]
macro_rules! get_shift_bits {
    ($len:expr,$offset:expr) => {{
        ($len - 1 - $offset) * 6
    }};
}
pub fn utf32_to_utf8_len(utf32: u32) -> Result<usize, Box<UnicodeParseError>> {
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

pub fn utf8char_to_utf32char(bytes: &[u8]) -> u32 {
    //シフトさせるビット数の算出方法
    // UTF8_MAX_LEN: UTF-8シーケンスの最大長
    // len: 今処理しているUTF-8シーケンスの長さ
    // offset: 今処理しているシーケンスのオフセット
    const FIRST_BYTE_MASK: [u8; 4] = [0xff, 0x1F, 0x0f, 0x07]; // 最初のバイト
    const REMAINING_BYTES: u8 = 0x3F; // 残りのバイト
    let len = bytes.len();
    let mut utf32 = 0;
    for offset in 0..len {
        let c = bytes[offset]; // UTF-8のシーケンスを読む

        // シーケンスが先頭か、それ以外かによってMASK値を変化させる
        let mask = if offset == 0 {
            FIRST_BYTE_MASK[len - 1]
        } else {
            REMAINING_BYTES
        };
        // シフトさせるビット数
        let sh_bits = get_shift_bits!(len, offset);
        // ビットを分離して合成する
        utf32 |= ((c & mask) as u32) << sh_bits
    }
    utf32
}
