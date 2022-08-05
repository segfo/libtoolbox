use serde_json::to_vec;

use crate::utf8_sequence_collector::utf8_validate;

#[test]
fn utf8char_to_utf32char_1バイト_基本() {
    let utf32 = utf8char_to_utf32char("A".as_bytes());
    assert_eq!(utf32, 0x0000_0041);
}

#[test]
fn utf8char_to_utf32char_1バイト_基本2() {
    let utf32 = utf8char_to_utf32char("B".as_bytes());
    assert_eq!(utf32, 0x0000_0042);
}
#[test]
fn utf8char_to_utf32char_2バイト_基本() {
    let utf32 = utf8char_to_utf32char("©".as_bytes());
    assert_eq!(utf32, 0x0000_00A9);
}

#[test]
fn utf8char_to_utf32char_2バイト_基本2() {
    let utf32 = utf8char_to_utf32char("§".as_bytes());
    assert_eq!(utf32, 0x0000_00A7);
}

#[test]
fn utf8char_to_utf32char_3バイト_基本() {
    let utf32 = utf8char_to_utf32char("ぁ".as_bytes());
    assert_eq!(utf32, 0x0000_3041);
}

#[test]
fn utf8char_to_utf32char_3バイト_基本2() {
    let utf32 = utf8char_to_utf32char("あ".as_bytes());
    assert_eq!(utf32, 0x0000_3042);
}
#[test]
fn utf8char_to_utf32char_4バイト_基本() {
    let utf32 = utf8char_to_utf32char("🍣".as_bytes());
    assert_eq!(utf32, 0x0001_F363);
}

#[test]
fn utf8char_to_utf32char_4バイト_基本2() {
    let utf32 = utf8char_to_utf32char("🍺".as_bytes());
    assert_eq!(utf32, 0x0001_F37A);
}

#[test]
fn utf8char_to_utf32char_BOF() {
    let utf32 = utf8char_to_utf32char(&[0xEF, 0xBB, 0xBF]);
    assert_eq!(utf32, 0x0000_FEFF);
}

#[test]
fn byte_to_utf32_基本() {
    let utf32 = byte_to_utf32("AB©§ぁあ🍣🍺".as_bytes().to_vec());
    assert!(utf32.is_ok());
    assert_eq!(
        utf32.unwrap(),
        vec![
            0x0000_0041,
            0x0000_0042,
            0x0000_00A9,
            0x0000_00A7,
            0x0000_3041,
            0x0000_3042,
            0x0001_F363,
            0x0001_F37A
        ]
    );
}

#[test]
fn byte_to_utf32_バイト列にBOMが入っているのでエラー() {
    let mut test = [0xEF, 0xBB, 0xBF].to_vec();
    test.append(&mut "AB©§ぁあ🍣🍺".as_bytes().to_vec());
    assert!(test.len() > 0);
    let utf32 = byte_to_utf32(test);
    assert!(utf32.is_err());
}

fn utf8char_to_utf32char(bytes: &[u8]) -> u32 {
    //シフトさせるビット数の算出方法
    // MAX:UTF-8シーケンスの最大値
    // len:今処理しているUTF-8シーケンスの長さ
    // offset:今処理しているシーケンスのオフセット
    // シフトさせるbit数 = SH_BIT[(MAX - (MAX + 1 - len)) - offset]
    const SH_BITS: [u8; 4] = [0, 6, 12, 18];
    const MAX: usize = SH_BITS.len();
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
        let sh_bits = SH_BITS[(MAX - (MAX + 1 - len)) - offset];
        utf32 |= ((c & mask) as u32) << sh_bits
    }
    utf32
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

pub fn string_to_utf32(s: String) -> Result<Vec<u32>, Box<dyn std::error::Error>> {
    let bytes = s.as_bytes().to_vec();
    byte_to_utf32(bytes)
}
