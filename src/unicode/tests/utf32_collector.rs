use crate::unicode::utf32_collector::*;
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
