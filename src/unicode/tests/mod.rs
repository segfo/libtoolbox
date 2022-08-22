pub mod utf32_collector;
pub mod utf32_encoder;
pub mod utf8_sequence_collector;

fn redundant_utf8_sequence(code_point: u8) -> Vec<Vec<u8>> {
    let mut default = vec![
        vec![0xC0, 0x80],
        vec![0xE0, 0x80, 0x80],
        vec![0xF0, 0x80, 0x80, 0x80],
    ];
    for i in 0..default.len() {
        default[i][i + 1] |= code_point;
    }
    default
}

#[test]
fn redundant_utf8_sequence_test1() {
    let v = redundant_utf8_sequence(0x2F);
    assert_eq!(vec![0xC0, 0xAF], v[0]);
}
#[test]
fn redundant_utf8_sequence_test2() {
    let v = redundant_utf8_sequence(0x2F);
    assert_eq!(vec![0xE0, 0x80, 0xAF], v[1]);
}
#[test]
fn redundant_utf8_sequence_test3() {
    let v = redundant_utf8_sequence(0x2F);
    assert_eq!(vec![0xF0, 0x80, 0x80, 0xAF], v[2]);
}
