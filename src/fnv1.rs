//! FNV1アルゴリズムの実装です。
//! 32ビットから128ビットのハッシュに対応しています。
//! ```
//! use toolbox::fnv1::{
//!     FNV1, // FNV1ハッシュ計算メソッドを使用するために必須のトレイトです。
//!     FNV1_32 // FNV1の実装です。単体ではnewメソッド（インスタンス生成）以外使用することが出来ません。
//! };
//! let mut fnv1 = FNV1_32::new();
//! fnv1.hash(123);
//! fnv1.hash_a(100);
//! fnv1.finalize(); // hashメソッドで計算したハッシュ値を取得します
//! fnv1.finalize_a(); // hash_aメソッドで計算したハッシュ値を取得します
//! ```
//! ```
//! use toolbox::fnv1::{
//!     FNV1, // FNV1ハッシュ計算メソッドを使用するために必須のトレイトです。
//!     FNV1_64 // FNV1の実装です。単体ではnewメソッド（インスタンス生成）以外使用することが出来ません。
//! };
//! let mut fnv1 = FNV1_64::new();
//! fnv1.hash(123);
//! fnv1.hash_a(100);
//! fnv1.finalize(); // hashメソッドで計算したハッシュ値を取得します
//! fnv1.finalize_a(); // hash_aメソッドで計算したハッシュ値を取得します
//! ```
//! ```
//! use toolbox::fnv1::{
//!     FNV1, // FNV1ハッシュ計算メソッドを使用するために必須のトレイトです。
//!     FNV1_128 // FNV1の実装です。単体ではnewメソッド（インスタンス生成）以外使用することが出来ません。
//! };
//! let mut fnv1 = FNV1_128::new();
//! fnv1.hash(123);
//! fnv1.hash_a(100);
//! fnv1.finalize(); // hashメソッドで計算したハッシュ値を取得します
//! fnv1.finalize_a(); // hash_aメソッドで計算したハッシュ値を取得します
//! ```
use std::num::Wrapping;

pub trait FNV1 {
    type Item;
    /// ハッシュ値を計算します。
    /// ハッシュ化する情報が32ビット以上あればこちらを使用します。（推奨）
    fn hash(&mut self, value: u8);
    /// ハッシュ値を計算します。
    /// hashメソッドとの違いは、ハッシュ化する情報が32ビット未満の場合において、分散率が良くなります。
    fn hash_a(&mut self, value: u8);
    /// hashメソッドに対応したハッシュ値を取得します
    fn finalize(&self) -> Self::Item;
    /// hash_aメソッドに対応したハッシュ値を取得します
    fn finalize_a(&self) -> Self::Item;
}

/// $struct_name: 生成したい構造体名
/// $type: ハッシュの返却値型
/// $prime: FNV1で定義されているハッシュのビット数に対応した素数
/// $offset_basis: FNV1で定義されているハッシュのビット数に対応した基底値
macro_rules! fnv1_impl {
    ($struct_name:ident,$type:ty,$prime:expr,$offset_basis:expr) => {
        #[derive(Debug, Copy, Clone)]
        pub struct $struct_name {
            hash_a: $type,
            hash: $type,
        }
        impl $struct_name {
            pub fn new() -> Self {
                $struct_name {
                    hash: $offset_basis,
                    hash_a: $offset_basis,
                }
            }
        }
        impl FNV1 for $struct_name {
            type Item = $type;
            fn hash_a(&mut self, value: u8) {
                let mut hash_a = self.hash_a;
                hash_a = (Wrapping($prime) * Wrapping(hash_a ^ value as Self::Item)).0;
                self.hash_a = hash_a;
            }
            fn hash(&mut self, value: u8) {
                let mut hash = self.hash;
                hash = (Wrapping($prime) * Wrapping(hash)).0 ^ value as Self::Item;
                self.hash = hash;
            }
            fn finalize(&self) -> Self::Item {
                self.hash
            }
            fn finalize_a(&self) -> Self::Item {
                self.hash_a
            }
        }
    };
}

/// 各ビット数に対して、ボイラープレートからロジックを定義します。
const OFFSET_BASIS_32: u32 = 2166136261;
const FNV_PRIME_32: u32 = 16777619;
fnv1_impl!(FNV1_32, u32, FNV_PRIME_32, OFFSET_BASIS_32);

const OFFSET_BASIS_64: u64 = 14695981039346656037;
const FNV_PRIME_64: u64 = 1099511628211;
fnv1_impl!(FNV1_64, u64, FNV_PRIME_64, OFFSET_BASIS_64);

const OFFSET_BASIS_128: u128 = 144066263297769815596495629667062367629;
const FNV_PRIME_128: u128 = 309485009821345068724781371;
fnv1_impl!(FNV1_128, u128, FNV_PRIME_128, OFFSET_BASIS_128);
