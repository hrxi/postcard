use criterion::{black_box, criterion_group, criterion_main, Criterion};
use serde_bytes::Bytes;
use serde::Serialize;
use postcard::FixedSizeByteArray;

fn serialize<const N: usize, const B: usize>(c: &mut Criterion)
where
    [u8; N]: Serialize
{
    let own: &_ = &FixedSizeByteArray::from([0; N]);
    let bytes: &_ = Bytes::new(&[0; N]);
    let big_array: &_ = &serde_big_array::Array([0; N]);
    let fixed: &_ = &[0; N];
    let variable: &_ = &[0; N] as &[_];
    let mut buf = [0; B];
    let mut group = c.benchmark_group(format!("serialize{}", N));
    group.bench_function("own", |b| b.iter(|| {
        let _ = black_box(postcard::to_slice(black_box(own), &mut buf).unwrap());
    }));
    group.bench_function("bytes", |b| b.iter(|| {
        let _ = black_box(postcard::to_slice(black_box(bytes), &mut buf).unwrap());
    }));
    group.bench_function("big_array", |b| b.iter(|| {
        let _ = black_box(postcard::to_slice(black_box(big_array), &mut buf).unwrap());
    }));
    group.bench_function("fixed_size", |b| b.iter(|| {
        let _ = black_box(postcard::to_slice(black_box(fixed), &mut buf).unwrap());
    }));
    group.bench_function("variable_size", |b| b.iter(|| {
        let _ = black_box(postcard::to_slice(black_box(variable), &mut buf).unwrap());
    }));
    group.finish();
}

fn serialize0(c: &mut Criterion) { serialize::<0, 64>(c) }
fn serialize1(c: &mut Criterion) { serialize::<1, 64>(c) }
fn serialize2(c: &mut Criterion) { serialize::<2, 64>(c) }
fn serialize4(c: &mut Criterion) { serialize::<4, 64>(c) }
fn serialize8(c: &mut Criterion) { serialize::<8, 64>(c) }
fn serialize16(c: &mut Criterion) { serialize::<16, 64>(c) }
fn serialize32(c: &mut Criterion) { serialize::<32, 64>(c) }

criterion_group!(byte_array,
    serialize0,
    serialize1,
    serialize2,
    serialize4,
    serialize8,
    serialize16,
    serialize32,
);
criterion_main!(byte_array);
