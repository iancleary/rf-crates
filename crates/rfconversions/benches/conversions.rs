use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rfconversions::{frequency, noise, p1db, power};

fn bench_power(c: &mut Criterion) {
    let mut group = c.benchmark_group("power");
    group.bench_function("watts_to_dbm", |b| {
        b.iter(|| power::watts_to_dbm(black_box(1.0)))
    });
    group.bench_function("dbm_to_watts", |b| {
        b.iter(|| power::dbm_to_watts(black_box(30.0)))
    });
    group.bench_function("db_to_linear", |b| {
        b.iter(|| power::db_to_linear(black_box(3.0)))
    });
    group.bench_function("linear_to_db", |b| {
        b.iter(|| power::linear_to_db(black_box(2.0)))
    });
    group.bench_function("dbm_to_dbw", |b| {
        b.iter(|| power::dbm_to_dbw(black_box(30.0)))
    });
    group.bench_function("dbw_to_dbm", |b| {
        b.iter(|| power::dbw_to_dbm(black_box(0.0)))
    });
    group.bench_function("milliwatts_to_dbm", |b| {
        b.iter(|| power::milliwatts_to_dbm(black_box(1000.0)))
    });
    group.finish();
}

fn bench_frequency(c: &mut Criterion) {
    let mut group = c.benchmark_group("frequency");
    group.bench_function("frequency_to_wavelength", |b| {
        b.iter(|| frequency::frequency_to_wavelength(black_box(30e9)))
    });
    group.bench_function("wavelength_to_frequency", |b| {
        b.iter(|| frequency::wavelength_to_frequency(black_box(0.01)))
    });
    group.bench_function("ghz_to_hz", |b| {
        b.iter(|| frequency::ghz_to_hz(black_box(30.0)))
    });
    group.bench_function("hz_to_ghz", |b| {
        b.iter(|| frequency::hz_to_ghz(black_box(30e9)))
    });
    group.finish();
}

fn bench_noise(c: &mut Criterion) {
    let mut group = c.benchmark_group("noise");
    group.bench_function("noise_figure_from_noise_temperature", |b| {
        b.iter(|| noise::noise_figure_from_noise_temperature(black_box(290.0)))
    });
    group.bench_function("noise_temperature_from_noise_figure", |b| {
        b.iter(|| noise::noise_temperature_from_noise_figure(black_box(3.0)))
    });
    group.bench_function("noise_power_from_bandwidth", |b| {
        b.iter(|| noise::noise_power_from_bandwidth(black_box(290.0), black_box(1e6)))
    });
    group.finish();
}

fn bench_p1db(c: &mut Criterion) {
    let mut group = c.benchmark_group("p1db");
    group.bench_function("input_to_output_db", |b| {
        b.iter(|| p1db::input_to_output_db(black_box(-10.0), black_box(20.0)))
    });
    group.bench_function("cascade_output_p1db", |b| {
        b.iter(|| p1db::cascade_output_p1db(black_box(34.0), black_box(20.0), black_box(30.0)))
    });
    group.bench_function("cascade_output_p1db_linear", |b| {
        b.iter(|| {
            p1db::cascade_output_p1db_linear(black_box(100.0), black_box(50.0), black_box(2.0))
        })
    });
    group.finish();
}

criterion_group!(
    benches,
    bench_power,
    bench_frequency,
    bench_noise,
    bench_p1db
);
criterion_main!(benches);
