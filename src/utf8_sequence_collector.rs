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
fn 明らかにおかしいUTF8シーケンスがある場合() {
    let mut bytes = [
        224, 130, 144, 224, 130, 173, 229, 149, 132, 230, 174, 148, 230, 139, 148, 224, 129, 145,
        65, 98, 112, 102, 53, 55, 224, 129, 171, 224, 128, 176, 224, 129, 176, 224, 128, 191, 224,
        128, 138, 224, 130, 154, 224, 129, 136, 224, 130, 171, 236, 191, 156,
    ]
    .to_vec();
    dump(&bytes);
    collect_utf8_sequences(&bytes);
}

#[test]
fn 明らかにおかしいUTF8シーケンスがある場合2() {
    let mut bytes = [
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
fn 明らかにおかしいUTF8シーケンスがある場合3() {
    let mut bytes = [0xF3, 0x91, 0x83, 0xF3].to_vec();
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
fn 明らかにおかしいUTF8シーケンスがある場合4() {
    let mut bytes = [
        251, 153, 139, 251, 153, 182, 254, 142, 159, 253, 181, 143, 253, 144, 143, 251, 154, 138,
        90, 121, 107, 125, 46, 44, 251, 154, 176, 251, 155, 171, 251, 154, 171, 251, 155, 164, 251,
        155, 145, 251, 153, 129, 251, 154, 147, 251, 153, 176, 247, 164, 135,
    ]
    .to_vec();
    dump(&bytes);
    collect_utf8_sequences(&bytes);
}

#[test]
fn 明らかにおかしいUTF8シーケンスがある場合5() {
    let mut bytes = [0xC1, 0xA3].to_vec();
    dump(&bytes);
    let actual_seq = collect_utf8_sequences(&bytes).get_sequence();
    let expect_seq = [DataSequence::ByteSequence(vec![0xC1, 0xA3])];
    assert_eq!(actual_seq.len(), expect_seq.len());
    for (i, seq) in actual_seq.iter().enumerate() {
        assert_eq!(seq, &expect_seq[i]);
    }
}

#[test]
fn 冗長な符号化() {
    let mut bytes = [0x2F, 0xC0, 0xAF, 0xE0, 0x80, 0xAF, 0xF0, 0x80, 0x80, 0xAF].to_vec();
    dump(&bytes);
    let actual_seq = collect_utf8_sequences(&bytes).sequence;
    let expect_seq = [
        DataSequence::Utf8Sequence("/".to_owned()),
        DataSequence::ByteSequence(vec![0xC0, 0xAF, 0xE0, 0x80, 0xAF, 0xF0, 0x80, 0x80, 0xAF]),
    ];
    assert_eq!(actual_seq.len(), expect_seq.len());
    for (i, seq) in actual_seq.iter().enumerate() {
        assert_eq!(seq, &expect_seq[i]);
    }
}

#[test]
fn 符号化されたBOM() {
    let mut bytes = [0xEF, 0xBB, 0xBF].to_vec();
    dump(&bytes);
    let actual_seq = collect_utf8_sequences(&bytes).sequence;
    let expect_seq = [DataSequence::ByteSequence(vec![0xEF, 0xBB, 0xBF])];
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

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum DataSequence {
    Utf8Sequence(String),
    ByteSequence(Vec<u8>),
}

pub struct SequenceData {
    sequence: Vec<DataSequence>,
    byte_sequence_total_len: usize,
    utf8_sequence_total_len: usize,
    string_total_len: usize,
}

impl SequenceData {
    pub fn get_sequence(&self) -> Vec<DataSequence> {
        self.sequence.clone()
    }
    pub fn get_total_bytes(&self) -> usize {
        self.byte_sequence_total_len + self.utf8_sequence_total_len
    }
    pub fn get_byte_sequence_bytes(&self) -> usize {
        self.byte_sequence_total_len
    }
    pub fn get_utf8_sequence_bytes(&self) -> usize {
        self.utf8_sequence_total_len
    }
    pub fn get_total_string_length(&self) -> usize {
        self.string_total_len
    }

    pub fn collect_sequence_data(sequence: Vec<DataSequence>) -> Self {
        let mut byte_sequence_total_len = 0;
        let mut string_total_len = 0;
        let mut utf8_sequence_total_len = 0;
        for ds in &sequence {
            match ds {
                DataSequence::Utf8Sequence(s) => {
                    utf8_sequence_total_len += s.bytes().len();
                    string_total_len += s.len();
                }
                DataSequence::ByteSequence(bytes) => byte_sequence_total_len += bytes.len(),
            }
        }
        SequenceData {
            sequence: sequence,
            byte_sequence_total_len: byte_sequence_total_len,
            utf8_sequence_total_len: utf8_sequence_total_len,
            string_total_len: string_total_len,
        }
    }
}

pub fn collect_utf8_sequences(byte: &Vec<u8>) -> SequenceData {
    let mut i = 0;
    let mut seqdata_list = Vec::new();
    let mut utf8_seq = Vec::new();
    let mut bin_seq = Vec::new();

    while i < byte.len() {
        let (len, valid) = utf8_len(byte, i);
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
fn utf8_len(byte_array: &Vec<u8>, index: usize) -> (usize, bool) {
    let byte = byte_array[index];
    let i = index;
    let len = byte_array.len();
    // 不正なUTF-8エンコードかどうかを確認する
    let is_invalid_encode = |off| -> bool {
        let second = byte_array[off + 1];
        let first = byte_array[off + 0];
        if first == 0xE0 && 0x80 <= second && second <= 0x9F // 冗長な符号化
                || first == 0xF0 && 0x80 <= second && second <= 0x8F // 冗長な符号化
                || first == 0xED && 0xA0 <= second // サロゲートペアの符号位置
                || first == 0xF4 && 0x90 <= second
        // Unicodeの範囲外
        {
            true
        } else {
            false
        }
    };
    // バリデーションを担う
    let validation = |seq_len| -> (usize, bool) {
        if (len - i) >= seq_len {
            for off in 1..seq_len {
                if !(0x80 <= byte_array[off + i] && byte_array[off + i] < 0xBF)
                    || is_invalid_encode(i)
                {
                    return (seq_len, false);
                }
            }
            (seq_len, true)
        } else if len > i {
            (len - i, false)
        } else {
            // あり得ないが、一応処理を入れておく。
            panic!("illegal index");
        }
    };
    let condition = |start: u8, end: u8| -> bool {
        let first = byte;
        start <= first && first <= end
    };

    if condition(0xF0, 0xF4) {
        // 4バイト文字
        validation(4)
    } else if condition(0xE0, 0xEF) {
        // 3バイト文字
        validation(3)
    } else if condition(0xC2, 0xDF) {
        // 2バイト文字
        validation(2)
    } else if byte & 0xff < 0x7f {
        // Ascii文字
        (1, true)
    } else {
        // UTF-8では表現不能なバイト
        (1, false)
    }
}
