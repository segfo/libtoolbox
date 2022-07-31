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
        let len = utf8_len(byte[i]);
        if len > 0 {
            if bin.len() > 0 {
                seq_list.push(DataSequence::BinaryArray(bin.clone()));
                bin.truncate(0);
            }
            // 有効なutf8シーケンスだったので、保存する。
            for n in 0..len {
                seq.push(byte[i + n]);
            }
            i += len;
        } else {
            // 有効なutf8シーケンスではない
            // 今までに収集されたシーケンスがあれば、シーケンスをStringにして保存する。
            if seq.len() > 0 {
                seq_list.push(DataSequence::Utf8(String::from_utf8(seq.clone()).unwrap()));
                seq.truncate(0);
            }
            // 有効でないシーケンスもとりあえず保存しておく
            bin.push(byte[i]);
            i += 1;
        }
    }
    if seq.len() > 0 {
        seq_list.push(DataSequence::Utf8(String::from_utf8(seq.clone()).unwrap()));
    }
    seq_list
}

fn utf8_len(byte: u8) -> usize {
    if byte & 0xF8 == 0xF8 {
        0
    } else if byte & 0xF8 == 0xF0 {
        4
    } else if byte & 0xF0 == 0xE0 {
        3
    } else if byte & 0xE0 == 0xC0 {
        2
    } else if byte & 0xff < 0x7f {
        1
    } else {
        0
    }
}
