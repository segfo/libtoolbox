#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum DataSequence {
    Utf8Sequence(String),
    Utf32Sequence(Vec<u32>),
    ByteSequence(Vec<u8>),
}

pub struct SequenceData {
    sequence: Vec<DataSequence>,
    byte_sequence_total_len: usize,
    utf32_sequence_total_len: usize,
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
    pub fn get_utf32_sequence_bytes(&self) -> usize {
        self.utf32_sequence_total_len
    }
    pub fn get_total_string_length(&self) -> usize {
        self.string_total_len
    }

    pub fn collect_sequence_data(sequence: Vec<DataSequence>) -> Self {
        let mut byte_sequence_total_len = 0;
        let mut string_total_len = 0;
        let mut utf8_sequence_total_len = 0;
        let mut utf32_sequence_total_len = 0;
        for ds in &sequence {
            match ds {
                DataSequence::Utf8Sequence(s) => {
                    utf8_sequence_total_len += s.bytes().len();
                    string_total_len += s.len();
                }
                DataSequence::Utf32Sequence(utf32) => {
                    utf32_sequence_total_len += utf32.len();
                }
                DataSequence::ByteSequence(bytes) => byte_sequence_total_len += bytes.len(),
            }
        }
        SequenceData {
            sequence: sequence,
            byte_sequence_total_len: byte_sequence_total_len,
            utf8_sequence_total_len: utf8_sequence_total_len,
            utf32_sequence_total_len: utf32_sequence_total_len,
            string_total_len: string_total_len,
        }
    }
}
