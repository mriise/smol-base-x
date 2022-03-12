use criterion::{
    black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput::Bytes,
};

use rand::Rng;
use smol_base_x::{
    util::{decoded_size, encoded_size},
    Base,
};

#[derive(Debug, Default)]
pub struct Base58Match {}

impl Base<58> for Base58Match {
    const ALPHABET: [u8; 58] =
        *b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";

    fn lookup_ascii(ch: u8) -> Option<usize> {
        smol_base_x::gen_ascii_match!(
            ch,
            b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz"
        )
    }
}

#[derive(Debug, Default)]
pub struct Base58LUT {}

impl Base<58> for Base58LUT {
    const ALPHABET: [u8; 58] =
        *b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";

    fn lookup_ascii(ch: u8) -> Option<usize> {
        const LUT: [i8; 256] = smol_base_x::util::gen_lut::<58>(
            b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz",
        );

        let ch = LUT[ch as usize];
        match ch {
            -1 => None,
            i => Some(i as usize),
        }
    }
}

pub fn lut_vs_match(c: &mut Criterion) {
    let mut group = c.benchmark_group("lookup_ascii_lut");
    for char in Base58LUT::ALPHABET.iter() {
        group.throughput(Bytes(1));
        group.bench_with_input(
            BenchmarkId::from_parameter(*char as char),
            char,
            |b, &ch| {
                b.iter(|| Base58LUT::lookup_ascii(black_box(ch)));
            },
        );
    }
    group.finish();

    let mut group = c.benchmark_group("lookup_ascii_match");
    for char in Base58Match::ALPHABET.iter() {
        group.throughput(Bytes(1));
        group.bench_with_input(
            BenchmarkId::from_parameter(*char as char),
            char,
            |b, &ch| {
                b.iter(|| Base58Match::lookup_ascii(black_box(ch)));
            },
        );
    }
    group.finish();
}

pub fn random_lut_vs_match(c: &mut Criterion) {
    let mut rng = rand::thread_rng();

    // 256 bits, typical hash lengths
    let mut inputs = Vec::<[u8; 32]>::with_capacity(10);
    for _ in 0..10 {
        inputs.push(rng.gen());
    }

    let mut encoded = Vec::<String>::with_capacity(10);

    let enc_size = encoded_size(58, 32);
    for hash in inputs.iter() {
        let mut buf = vec![0; enc_size];
        let size = Base58Match::encode_mut(hash, &mut buf).unwrap();
        encoded.push(String::from_utf8_lossy(&buf[..size]).to_string());
    }

    let mut buf = [0u8; 32]; // the whole benchmark uses only this buffer as it will fill it with zeroes on its own
    let buf = &mut buf[..decoded_size(58, encoded_size(58, 32))];

    let mut group = c.benchmark_group("random_ascii_lut");
    for (i, str) in encoded.iter().enumerate() {
        // group.throughput(Bytes(32));
        group.bench_with_input(BenchmarkId::from_parameter(i), str, |b, str| {
            b.iter(|| {
                Base58LUT::decode_mut(black_box(str), black_box(buf)).unwrap();
            });
        });
    }
    group.finish();

    let mut group = c.benchmark_group("random_ascii_match");
    for (i, str) in encoded.iter().enumerate() {
        // group.throughput(Bytes(32));
        group.bench_with_input(BenchmarkId::from_parameter(i), str, |b, str| {
            b.iter(|| {
                Base58Match::decode_mut(black_box(str), black_box(buf)).unwrap();
            });
        });
    }
    group.finish();
}

criterion_group!(benches, random_lut_vs_match);
criterion_main!(benches);
