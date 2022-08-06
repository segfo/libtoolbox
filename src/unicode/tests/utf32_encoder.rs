use crate::unicode::utf32_encoder::*;
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
