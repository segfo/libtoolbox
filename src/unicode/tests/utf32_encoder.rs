use crate::unicode::utf32_encoder::*;
macro_rules! utf8_char_to_utf32_char_test {
    ($id:ident,$c: expr, $utf32: expr) => {
        #[test]
        fn $id() {
            let utf32 = utf8char_to_utf32char(&$c);
            assert_eq!(utf32, $utf32);
        }
    };
}

utf8_char_to_utf32_char_test!(utf8char_to_utf32char_1バイト_基本1, [0x00], 0x0000_0000);
utf8_char_to_utf32_char_test!(utf8char_to_utf32char_1バイト_基本2, [0x7F], 0x0000_007F);
utf8_char_to_utf32_char_test!(
    utf8char_to_utf32char_2バイト_基本1,
    [0xC2, 0x80],
    0x0000_0080
);
utf8_char_to_utf32_char_test!(
    utf8char_to_utf32char_2バイト_基本2,
    [0xDF, 0xBF],
    0x0000_07FF
);
utf8_char_to_utf32_char_test!(
    utf8char_to_utf32char_3バイト_基本1,
    [0xE0, 0xA0, 0x80],
    0x0000_0800
);
utf8_char_to_utf32_char_test!(
    utf8char_to_utf32char_3バイト_基本2,
    [0xEF, 0xBF, 0xAF],
    0x0000_FFEF
);
utf8_char_to_utf32_char_test!(
    utf8char_to_utf32char_4バイト_基本1,
    [0xF0, 0x90, 0x80, 0x80],
    0x0001_0000
);
utf8_char_to_utf32_char_test!(
    utf8char_to_utf32char_4バイト_基本2,
    [0xF0, 0x9F, 0xA7, 0xBF],
    0x0001_F9FF
);
utf8_char_to_utf32_char_test!(utf8char_to_utf32char_BOM, [0xEF, 0xBB, 0xBF], 0x0000_FEFF);
