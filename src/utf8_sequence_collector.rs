#[test]
fn バイナリデータからUTF8文字列とバイナリに分離する() {
    let mut bytes = "Helloほげ\x01\x02\x00\x00\x00\x00ふがINVAlid"
        .as_bytes()
        .to_vec();

    let len = bytes.len();
    bytes[14] = 0xff;
    bytes[15] = 0xff;
    bytes[16] = 0xff;
    bytes[len - 7] = 0xf8;
    let actual_seq = collect_utf8_sequences(&bytes);
    let expect_seq = [
        DataSequence::Utf8("Helloほげ\x01\x02\x00".to_owned()),
        DataSequence::BinaryArray(vec![0xff, 0xff, 0xff]),
        DataSequence::Utf8("ふが".to_owned()),
        DataSequence::BinaryArray(vec![0xf8]),
        DataSequence::Utf8("NVAlid".to_owned()),
    ];
    for (i, seq) in actual_seq.iter().enumerate() {
        assert_eq!(seq, &expect_seq[i]);
    }
}
#[test]
fn 明らかにおかしいUTF8シーケンスがある場合_シーケンスの先頭が不正() {
    let mut bytes = "Helloほげふが".as_bytes().to_vec();

    let len = bytes.len();
    bytes[5] = 0;
    dump(&bytes);
    let actual_seq = collect_utf8_sequences(&bytes);
    let expect_seq = [
        DataSequence::Utf8("Hello\0".to_owned()),
        DataSequence::BinaryArray(vec![129, 187]),
        DataSequence::Utf8("げふが".to_owned()),
    ];
    assert_eq!(actual_seq.len(), expect_seq.len());
    for (i, seq) in actual_seq.iter().enumerate() {
        assert_eq!(seq, &expect_seq[i]);
    }
}

#[test]
fn 明らかにおかしいUTF8シーケンスがある場合_シーケンスの途中が不正() {
    let mut bytes = "Helloほげふが".as_bytes().to_vec();

    let len = bytes.len();
    bytes[6] = 0;
    dump(&bytes);
    let actual_seq = collect_utf8_sequences(&bytes);
    let expect_seq = [
        DataSequence::Utf8("Hello".to_owned()),
        DataSequence::BinaryArray(vec![0xe3, 0, 187]),
        DataSequence::Utf8("げふが".to_owned()),
    ];
    assert_eq!(actual_seq.len(), expect_seq.len());
    for (i, seq) in actual_seq.iter().enumerate() {
        assert_eq!(seq, &expect_seq[i]);
    }
}
fn dump(byte: &Vec<u8>) {
    for b in byte {
        print!("{:?} ", b);
    }
    println!();
}

#[derive(Debug, PartialEq, PartialOrd)]
pub enum DataSequence {
    Utf8(String),
    BinaryArray(Vec<u8>),
}

pub fn collect_utf8_sequences(byte: &Vec<u8>) -> Vec<DataSequence> {
    let mut i = 0;
    let mut seq_list = Vec::new();
    let mut seq = Vec::new();
    let mut bin = Vec::new();
    while i < byte.len() {
        let (len, valid) = utf8_len(byte, i);
        if valid {
            // 有効なシーケンスがあったら記録していく
            if bin.len() > 0 {
                seq_list.push(DataSequence::BinaryArray(bin.clone()));
                bin.truncate(0);
            }
            for off in 0..len {
                seq.push(byte[i + off]);
            }
        } else {
            // 有効なutf8シーケンスではない
            // 今までに収集された有効なUTF-8シーケンスがあれば、シーケンスをStringにして保存する。
            if seq.len() > 0 {
                seq_list.push(DataSequence::Utf8(String::from_utf8(seq.clone()).unwrap()));
                seq.truncate(0);
            }
            // 有効でないシーケンスもとりあえず保存しておく
            for off in 0..len {
                bin.push(byte[i + off]);
            }
        }
        i += len;
    }
    if seq.len() > 0 {
        seq_list.push(DataSequence::Utf8(String::from_utf8(seq.clone()).unwrap()));
    }
    seq_list
}

fn utf8_len(byte_array: &Vec<u8>, index: usize) -> (usize, bool) {
    let byte = byte_array[index];
    let i = index;
    let len = byte_array.len();
    let valid = |seq_len| -> (usize, bool) {
        if (len - i) >= seq_len {
            for off in 0..seq_len {
                if byte_array[off + i] & 0x80 != 0x80 {
                    return (seq_len, false);
                }
            }
            (seq_len, true)
        } else {
            (seq_len, false)
        }
    };
    if byte & 0xC0 == 0x80 {
        // いきなり80が来たら異常なので異常なシーケンスを返す
        (1, false)
    } else if byte & 0xF8 == 0xF8 {
        // F8は来ないはずなので異常なシーケンスを返す
        (1, false)
    } else if byte & 0xF8 == 0xF0 {
        valid(4)
    } else if byte & 0xF0 == 0xE0 {
        valid(3)
    } else if byte & 0xE0 == 0xC0 {
        valid(2)
    } else if byte & 0xff < 0x7f {
        (1, true)
    } else {
        // 上記以外であれば異常なシーケンスを返す
        (1, false)
    }
}
