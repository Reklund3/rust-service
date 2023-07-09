use criterion::{black_box, criterion_group, criterion_main, Criterion};
use jni::sys::jbyte;
use rand::Rng;
use std::ffi::c_char;
use temp::util::testing;

fn criterion_benchmark(c: &mut Criterion) {
    let source: [c_char; 6] = [
        'h' as c_char,
        'e' as c_char,
        'l' as c_char,
        'l' as c_char,
        'o' as c_char,
        '\0' as c_char, // Null-terminator
    ];

    const ARRAY_SIZE: usize = 1000000;
    let mut rng = rand::thread_rng();
    let mut test_array: [c_char; ARRAY_SIZE] = [0; ARRAY_SIZE];
    for i in 0..ARRAY_SIZE {
        let value = rng.gen_range(65..=122) as c_char;
        test_array[i] = value;
    }

    // Fixed string bench
    c.bench_function("fixed string copy with saved pointer", |b| {
        b.iter(|| unsafe {
            testing::loop_char(black_box(source.as_ptr()), black_box(&mut [0 as jbyte; 5]))
        })
    });
    c.bench_function("fixed string copy using offset", |b| {
        b.iter(|| unsafe {
            testing::working(black_box(source.as_ptr()), black_box(&mut [0 as jbyte; 5]))
        })
    });

    // ARRAY_SIZE char string bench
    c.bench_function("rand ARRAY_SIZE copy with saved pointer", |b| {
        b.iter(|| unsafe {
            testing::loop_char(
                black_box(test_array.as_ptr()),
                black_box(&mut [0 as jbyte; ARRAY_SIZE]),
            )
        })
    });
    c.bench_function("rand ARRAY_SIZE copy using offset", |b| {
        b.iter(|| unsafe {
            testing::working(
                black_box(test_array.as_ptr()),
                black_box(&mut [0 as jbyte; ARRAY_SIZE]),
            )
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
