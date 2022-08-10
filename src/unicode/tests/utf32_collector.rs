use crate::unicode::error::{UnicodeParseError, UnicodeParseErrorKind};
use crate::unicode::utf32_collector::*;
use crate::unicode::utf32_encoder::*;
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
#[allow(non_snake_case)]
fn byte_to_utf32_バイト列にBOMが入っているのでエラー() {
    let mut test = [0xEF, 0xBB, 0xBF].to_vec();
    test.append(&mut "AB©§ぁあ🍣🍺".as_bytes().to_vec());
    assert!(test.len() > 0);
    let utf32 = byte_to_utf32(test);
    assert!(utf32.is_err());
}

macro_rules! utf32_to_string_test {
    ($id:ident,$utf32_array:expr,$proc:expr) => {
        #[test]
        fn $id() {
            let s = utf32_to_string($utf32_array);
            $proc(s);
        }
    };
}

macro_rules! utf32_to_string_Err_test {
    ($id:ident,$utf32_array:expr) => {
        utf32_to_string_test!($id, $utf32_array, |s: Result<
            String,
            Box<UnicodeParseError>,
        >| {
            assert!(s.is_err());
            assert_eq!(
                s.unwrap_err().get_error(),
                UnicodeParseErrorKind::IllegalRange
            );
        });
    };
}
macro_rules! utf32_to_string_Ok_test {
    ($id:ident,$utf32_array:expr,$s:expr) => {
        utf32_to_string_test!($id, $utf32_array, |s: Result<
            String,
            Box<UnicodeParseError>,
        >| {
            assert!(s.is_ok());
            assert_eq!($s, &s.unwrap());
        });
    };
}

macro_rules! utf32_to_utf8_len_test {
    ($id:ident,$utf32:expr,$proc:expr) => {
        #[test]
        fn $id() {
            let s = utf32_to_utf8_len($utf32);
            $proc(s);
        }
    };
}
macro_rules! utf32_to_utf8_len_Ok_test {
    ($id:ident,$utf32:expr,$s:expr) => {
        utf32_to_utf8_len_test!($id, $utf32, |s: Result<usize, Box<UnicodeParseError>>| {
            assert!(s.is_ok());
            assert_eq!($s, s.unwrap());
        });
    };
}
macro_rules! utf32_to_utf8_len_Err_test {
    ($id:ident,$utf32:expr) => {
        utf32_to_utf8_len_test!($id, $utf32, |s: Result<usize, Box<UnicodeParseError>>| {
            assert!(s.is_err());
            assert_eq!(
                s.unwrap_err().get_error(),
                UnicodeParseErrorKind::IllegalRange
            );
        });
    };
}

utf32_to_utf8_len_Ok_test!(utf32_to_utf8_len_1バイト文字1, 0x0000_0000, 1);
utf32_to_utf8_len_Ok_test!(utf32_to_utf8_len_1バイト文字2, 0x0000_007F, 1);
utf32_to_utf8_len_Ok_test!(utf32_to_utf8_len_2バイト文字1, 0x0000_0080, 2);
utf32_to_utf8_len_Ok_test!(utf32_to_utf8_len_2バイト文字2, 0x0000_07FF, 2);
utf32_to_utf8_len_Ok_test!(utf32_to_utf8_len_3バイト文字1, 0x0000_0800, 3);
utf32_to_utf8_len_Ok_test!(utf32_to_utf8_len_3バイト文字2, 0x0000_FFFF, 3);
utf32_to_utf8_len_Ok_test!(utf32_to_utf8_len_4バイト文字1, 0x0001_0000, 4);
utf32_to_utf8_len_Ok_test!(utf32_to_utf8_len_4バイト文字2, 0x0010_FFFF, 4);
/////////////
utf32_to_utf8_len_Err_test!(
    utf32_to_utf8_len_ハイサロゲートコードポイント開始,
    0x0000_D800
);
utf32_to_utf8_len_Err_test!(
    utf32_to_utf8_len_ハイサロゲートコードポイント終了,
    0x0000_DBFF
);
utf32_to_utf8_len_Err_test!(
    utf32_to_utf8_len_ローサロゲートコードポイント開始,
    0x0000_DC00
);
utf32_to_utf8_len_Err_test!(
    utf32_to_utf8_len_ローサロゲートコードポイント終了,
    0x0000_DFFF
);
utf32_to_utf8_len_Err_test!(utf32_to_utf8_len_範囲外開始, 0x0011_0000);
utf32_to_utf8_len_Err_test!(utf32_to_utf8_len_範囲外終了, 0xFFFF_FFFF);

utf32_to_string_Ok_test!(utf32_to_string_test_基本1バイト文字1, &[0x41].to_vec(), "A");
utf32_to_string_Ok_test!(
    utf32_to_string_test_基本2バイト文字1,
    &[0x0000_0080].to_vec(),
    &String::from_utf8([0xC2, 0x80].to_vec()).unwrap()
);
utf32_to_string_Ok_test!(
    utf32_to_string_test_基本2バイト文字2,
    &[0x0000_07FF].to_vec(),
    &String::from_utf8([0xDF, 0xBF].to_vec()).unwrap()
);
utf32_to_string_Err_test!(
    utf32_to_string_test_ハイサロゲートコードポイント開始,
    &[0x0000_D800].to_vec()
);
utf32_to_string_Err_test!(
    utf32_to_string_test_ハイサロゲートコードポイント終了,
    &[0x0000_DBFF].to_vec()
);
utf32_to_string_Err_test!(
    utf32_to_string_test_ローサロゲートコードポイント開始,
    &[0x0000_DC00].to_vec()
);
utf32_to_string_Err_test!(
    utf32_to_string_test_ローサロゲートコードポイント終了,
    &[0x0000_DFFF].to_vec()
);
utf32_to_string_Ok_test!(
    utf32_to_string_test_基本3バイト文字1,
    &[0x3042].to_vec(),
    "あ"
);
utf32_to_string_Ok_test!(
    utf32_to_string_test_基本4バイト文字1,
    &[0x0001_F363].to_vec(),
    "🍣"
);
