pub fn utf8char_to_utf32char(bytes: &[u8]) -> u32 {
    //シフトさせるビット数の算出方法
    // MAX:UTF-8シーケンスの最大値
    // len:今処理しているUTF-8シーケンスの長さ
    // offset:今処理しているシーケンスのオフセット
    // シフトさせるbit数 = SH_BIT[(MAX - (MAX + 1 - len)) - offset]
    const SH_BITS: [u8; 4] = [0, 6, 12, 18];
    const MAX: usize = SH_BITS.len();
    const FIRST_BYTE_MASK: [u8; 4] = [0xff, 0x1F, 0x0f, 0x07]; // 最初のバイト
    const REMAINING_BYTES: u8 = 0x3F; // 残りのバイト
    let len = bytes.len();
    let mut utf32 = 0;
    for offset in 0..len {
        let c = bytes[offset]; // UTF-8のシーケンスを読む

        // シーケンスが先頭か、それ以外かによってMASK値を変化させる
        let mask = if offset == 0 {
            FIRST_BYTE_MASK[len - 1]
        } else {
            REMAINING_BYTES
        };
        // シフトさせるビット数
        let sh_bits = SH_BITS[(MAX - (MAX + 1 - len)) - offset];
        // ビットを分離して合成する
        utf32 |= ((c & mask) as u32) << sh_bits
    }
    utf32
}
