mod bench_utils;
use bench_utils::{install_nargo, run_nargo_compile, test_program_dir_path, HALO2_BACKEND};
use criterion::{criterion_group, criterion_main, Criterion};

fn benchmark_tests_compilation(c: &mut Criterion) {
    install_nargo(HALO2_BACKEND);

    // 1_mul
    let path = test_program_dir_path("1_mul");
    c.bench_function("1_mul_compilation", |b| {
        b.iter(|| run_nargo_compile(path.clone()))
    });

    // 2_div
    let path = test_program_dir_path("2_div");
    c.bench_function("2_div_compilation", |b| {
        b.iter(|| run_nargo_compile(path.clone()))
    });

    // 3_add
    let path = test_program_dir_path("3_add");
    c.bench_function("3_add_compilation", |b| {
        b.iter(|| run_nargo_compile(path.clone()))
    });

    // 4_sub
    let path = test_program_dir_path("4_sub");
    c.bench_function("4_sub_compilation", |b| {
        b.iter(|| run_nargo_compile(path.clone()))
    });

    // 5_over
    let path = test_program_dir_path("5_over");
    c.bench_function("5_over_compilation", |b| {
        b.iter(|| run_nargo_compile(path.clone()))
    });

    // 6_array
    let path = test_program_dir_path("6_array");
    c.bench_function("6_array_compilation", |b| {
        b.iter(|| run_nargo_compile(path.clone()))
    });

    // 7_function
    let path = test_program_dir_path("7_function");
    c.bench_function("7_function_compilation", |b| {
        b.iter(|| run_nargo_compile(path.clone()))
    });

    // bit_and
    let path = test_program_dir_path("bit_and");
    c.bench_function("bit_and_compilation", |b| {
        b.iter(|| run_nargo_compile(path.clone()))
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(10);
    targets =  benchmark_tests_compilation
}
criterion_main!(benches);
