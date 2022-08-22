use crate::unicode::error::UnicodeParseErrorKind;
use crate::unicode::sequence_data::*;
use crate::unicode::utf8_sequence_collector::*;
#[allow(dead_code)]
fn dump(byte: &Vec<u8>) {
    for b in byte {
        print!("{:x} ", b);
    }
    println!();
}

#[test]
#[allow(non_snake_case)]
fn バイナリデータからUTF8文字列とバイナリに分離する() {
    let mut bytes = "Helloほげ\x01\x02\x00\x00\x00\x00ふがINVAlid"
        .as_bytes()
        .to_vec();
    let len = bytes.len();
    bytes[14] = 0xff;
    bytes[15] = 0xff;
    bytes[16] = 0xff;
    bytes[len - 7] = 0xf8;
    println!("{:?}", bytes);
    let actual_seq = collect_utf8_sequences(&bytes).get_sequence();
    let expect_seq = [
        DataSequence::Utf8Sequence("Helloほげ\x01\x02\x00".to_owned()),
        DataSequence::ByteSequence(vec![0xff, 0xff, 0xff]),
        DataSequence::Utf8Sequence("ふが".to_owned()),
        DataSequence::ByteSequence(vec![0xf8]),
        DataSequence::Utf8Sequence("NVAlid".to_owned()),
    ];
    println!("{:?}", actual_seq);
    assert_eq!(actual_seq.len(), expect_seq.len());
    for (i, seq) in actual_seq.iter().enumerate() {
        assert_eq!(seq, &expect_seq[i]);
    }
}
#[test]
#[allow(non_snake_case)]
fn 明らかにおかしいUTF8シーケンスがある場合_シーケンスの先頭が不正() {
    let mut bytes = "Helloほげふが".as_bytes().to_vec();

    bytes[5] = 0;
    dump(&bytes);
    let actual_seq = collect_utf8_sequences(&bytes).get_sequence();
    let expect_seq = [
        DataSequence::Utf8Sequence("Hello\0".to_owned()),
        DataSequence::ByteSequence(vec![129, 187]),
        DataSequence::Utf8Sequence("げふが".to_owned()),
    ];
    assert_eq!(actual_seq.len(), expect_seq.len());
    for (i, seq) in actual_seq.iter().enumerate() {
        assert_eq!(seq, &expect_seq[i]);
    }
}

#[test]
#[allow(non_snake_case)]
fn 明らかにおかしいUTF8シーケンスがある場合_シーケンスの途中が不正() {
    let mut bytes = "Helloほげふが".as_bytes().to_vec();

    bytes[6] = 0;
    dump(&bytes);
    let actual_seq = collect_utf8_sequences(&bytes).get_sequence();
    let expect_seq = [
        DataSequence::Utf8Sequence("Hello".to_owned()),
        DataSequence::ByteSequence(vec![0xe3, 0, 187]),
        DataSequence::Utf8Sequence("げふが".to_owned()),
    ];
    assert_eq!(actual_seq.len(), expect_seq.len());
    for (i, seq) in actual_seq.iter().enumerate() {
        assert_eq!(seq, &expect_seq[i]);
    }
}

#[test]
#[allow(non_snake_case)]
fn 明らかにおかしいUTF8シーケンスがある場合() {
    let bytes = [
        224, 130, 144, 224, 130, 173, 229, 149, 132, 230, 174, 148, 230, 139, 148, 224, 129, 145,
        65, 98, 112, 102, 53, 55, 224, 129, 171, 224, 128, 176, 224, 129, 176, 224, 128, 191, 224,
        128, 138, 224, 130, 154, 224, 129, 136, 224, 130, 171, 236, 191, 156,
    ]
    .to_vec();
    dump(&bytes);
    collect_utf8_sequences(&bytes);
}

#[test]
#[allow(non_snake_case)]
fn 明らかにおかしいUTF8シーケンスがある場合2() {
    let bytes = [
        // 235, 137, 155, 235, 137, 166, 238, 158, 143,
        237, 165, 159, 237, 128, 159, 235, 138, 154, 74, 105, 123, 109, 62, 60, 235, 138, 160, 235,
        139, 187, 235, 138, 187, 235, 139, 180, 235, 139, 129, 235, 137, 145, 235, 138, 131, 235,
        137, 160, 231, 180, 151,
    ]
    .to_vec();
    dump(&bytes);
    collect_utf8_sequences(&bytes);
}

#[test]
#[allow(non_snake_case)]
fn 明らかにおかしいUTF8シーケンスがある場合3() {
    let bytes = [0xF3, 0x91, 0x83, 0xF3].to_vec();
    dump(&bytes);
    collect_utf8_sequences(&bytes);
    let actual_seq = collect_utf8_sequences(&bytes).get_sequence();
    let expect_seq = [DataSequence::ByteSequence(vec![0xF3, 0x91, 0x83, 0xF3])];
    assert_eq!(actual_seq.len(), expect_seq.len());
    for (i, seq) in actual_seq.iter().enumerate() {
        assert_eq!(seq, &expect_seq[i]);
    }
}

#[test]
#[allow(non_snake_case)]
fn 明らかにおかしいUTF8シーケンスがある場合4() {
    let bytes = [
        251, 153, 139, 251, 153, 182, 254, 142, 159, 253, 181, 143, 253, 144, 143, 251, 154, 138,
        90, 121, 107, 125, 46, 44, 251, 154, 176, 251, 155, 171, 251, 154, 171, 251, 155, 164, 251,
        155, 145, 251, 153, 129, 251, 154, 147, 251, 153, 176, 247, 164, 135,
    ]
    .to_vec();
    dump(&bytes);
    collect_utf8_sequences(&bytes);
}

#[test]
#[allow(non_snake_case)]
fn 明らかにおかしいUTF8シーケンスがある場合5() {
    let bytes = [0xC1, 0xA3].to_vec();
    dump(&bytes);
    let actual_seq = collect_utf8_sequences(&bytes).get_sequence();
    let expect_seq = [DataSequence::ByteSequence(vec![0xC1, 0xA3])];
    assert_eq!(actual_seq.len(), expect_seq.len());
    for (i, seq) in actual_seq.iter().enumerate() {
        assert_eq!(seq, &expect_seq[i]);
    }
}

#[allow(non_snake_case)]
#[test]
fn 冗長なAscii符号化() {
    use crate::unicode::tests::redundant_utf8_sequence;
    for c in 0x00..=0x7F {
        let expect = redundant_utf8_sequence(c);
        let actual_seq = collect_utf8_sequences(
            &[
                vec![c],
                expect[0].clone(),
                expect[1].clone(),
                expect[2].clone(),
            ]
            .concat(),
        )
        .get_sequence();
        let expect_seq = [
            DataSequence::Utf8Sequence(String::from_utf8(vec![c]).unwrap().to_owned()),
            DataSequence::ByteSequence(
                [expect[0].clone(), expect[1].clone(), expect[2].clone()].concat(),
            ),
        ];
        assert_eq!(actual_seq.len(), expect_seq.len());
        for (i, seq) in actual_seq.iter().enumerate() {
            assert_eq!(seq, &expect_seq[i]);
        }
    }
}

#[test]
#[allow(non_snake_case)]
fn 符号化されたBOM() {
    let bytes = [0xEF, 0xBB, 0xBF].to_vec();
    dump(&bytes);
    let actual_seq = collect_utf8_sequences(&bytes).get_sequence();
    let expect_seq = [DataSequence::ByteSequence(vec![0xEF, 0xBB, 0xBF])];
    assert_eq!(actual_seq.len(), expect_seq.len());
    for (i, seq) in actual_seq.iter().enumerate() {
        assert_eq!(seq, &expect_seq[i]);
    }
}

#[test]
#[allow(non_snake_case)]
fn utf8_validate_符号化されたBOM() {
    let bytes = [0xEF, 0xBB, 0xBF].to_vec();
    dump(&bytes);
    let actual_info = utf8_validate(&bytes, 0);
    let (len, valid) = actual_info.get_len_valid();
    assert_eq!(valid, false);
    assert_eq!(len, 3);
    assert_eq!(
        actual_info.get_error().unwrap().get_error(),
        UnicodeParseErrorKind::IllegalRange
    );
}

#[test]
#[allow(non_snake_case)]
fn utf8_validate_符号化されたBOM_バッファ尻切れトンボ() {
    let bytes = [0xEF, 0xBB].to_vec();
    dump(&bytes);
    let actual_info = utf8_validate(&bytes, 0);
    let (len, valid) = actual_info.get_len_valid();
    assert_eq!(valid, false);
    assert_eq!(len, bytes.len());
    assert_eq!(
        actual_info.get_error().unwrap().get_error(),
        UnicodeParseErrorKind::IllegalByteSequence
    );
}
#[test]
fn utf8_validate_1バイト_基本() {
    let bytes = "A".as_bytes().to_vec();
    dump(&bytes);
    let actual_info = utf8_validate(&bytes, 0);
    let (len, valid) = actual_info.get_len_valid();
    assert_eq!(valid, true);
    assert_eq!(len, bytes.len());
    assert!(actual_info.get_error().is_none());
}
#[test]
fn utf8_validate_2バイト_基本() {
    let bytes = "§".as_bytes().to_vec();
    dump(&bytes);
    let actual_info = utf8_validate(&bytes, 0);
    let (len, valid) = actual_info.get_len_valid();
    assert_eq!(valid, true);
    assert_eq!(len, bytes.len());
    assert!(actual_info.get_error().is_none());
}
#[test]
fn utf8_validate_2バイト_バッファ尻切れトンボ() {
    let mut bytes = "§".as_bytes().to_vec();
    bytes.truncate(bytes.len() - 1);
    dump(&bytes);
    let actual_info = utf8_validate(&bytes, 0);
    let (len, valid) = actual_info.get_len_valid();
    assert_eq!(valid, false);
    assert_eq!(len, bytes.len());
    assert_eq!(
        actual_info.get_error().unwrap().get_error(),
        UnicodeParseErrorKind::IllegalByteSequence
    );
}
#[test]
fn utf8_validate_3バイト_基本() {
    let bytes = "あ".as_bytes().to_vec();
    dump(&bytes);
    let actual_info = utf8_validate(&bytes, 0);
    let (len, valid) = actual_info.get_len_valid();
    assert_eq!(valid, true);
    assert_eq!(len, bytes.len());
    assert!(actual_info.get_error().is_none());
}
#[test]
fn utf8_validate_3バイト_バッファ尻切れトンボ() {
    let mut bytes = "あ".as_bytes().to_vec();
    bytes.truncate(bytes.len() - 1);
    dump(&bytes);
    let actual_info = utf8_validate(&bytes, 0);
    let (len, valid) = actual_info.get_len_valid();
    assert_eq!(valid, false);
    assert_eq!(len, bytes.len());
    assert_eq!(
        actual_info.get_error().unwrap().get_error(),
        UnicodeParseErrorKind::IllegalByteSequence
    );
}
#[test]
fn utf8_validate_4バイト_基本() {
    let bytes = "🍺".as_bytes().to_vec();
    dump(&bytes);
    let actual_info = utf8_validate(&bytes, 0);
    let (len, valid) = actual_info.get_len_valid();
    assert_eq!(valid, true);
    assert_eq!(len, bytes.len());
    assert!(actual_info.get_error().is_none());
}
#[test]
fn utf8_validate_4バイト_バッファ尻切れトンボ() {
    let mut bytes = "🍺".as_bytes().to_vec();
    bytes.truncate(bytes.len() - 1);
    dump(&bytes);
    let actual_info = utf8_validate(&bytes, 0);
    let (len, valid) = actual_info.get_len_valid();
    assert_eq!(valid, false);
    assert_eq!(len, bytes.len());
    assert_eq!(
        actual_info.get_error().unwrap().get_error(),
        UnicodeParseErrorKind::IllegalByteSequence
    );
}

macro_rules! test_utf8_validate_invalid_byte {
    ($id:ident,$v:expr) => {
        #[test]
        fn $id() {
            let bytes = $v.to_vec();
            dump(&bytes);
            let actual_info = utf8_validate(&bytes, 0);
            let (len, valid) = actual_info.get_len_valid();
            assert_eq!(valid, false);
            assert_eq!(len, bytes.len());
            assert_eq!(
                actual_info.get_error().unwrap().get_error(),
                UnicodeParseErrorKind::IllegalRange
            );
        }
    };
    ($id:ident,$v:expr,$vv:expr) => {
        test_utf8_validate_invalid_byte!(
            $id,
            || -> Vec<u8> {
                let mut v = $v.to_owned().as_bytes().to_vec();
                let len = v.len();
                v[len - 1] = $vv;
                v
            }()
        );
    };
}
test_utf8_validate_invalid_byte!(utf8_validate_1バイト_不正なバイト1, [0x80]);
test_utf8_validate_invalid_byte!(utf8_validate_1バイト_不正なバイト2, [0xFF]);
test_utf8_validate_invalid_byte!(utf8_validate_2バイト_不正なバイト1, "§", 0x7F);
test_utf8_validate_invalid_byte!(utf8_validate_2バイト_不正なバイト2, "§", 0xC0);
test_utf8_validate_invalid_byte!(utf8_validate_3バイト_不正なバイト1, "あ", 0x7F);
test_utf8_validate_invalid_byte!(utf8_validate_3バイト_不正なバイト2, "あ", 0xC0);
test_utf8_validate_invalid_byte!(utf8_validate_4バイト_不正なバイト1, "🍺", 0x7F);
test_utf8_validate_invalid_byte!(utf8_validate_4バイト_不正なバイト2, "🍺", 0xC0);
