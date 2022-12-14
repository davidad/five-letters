use core::time::Duration;
use criterion::*;
use five_letters::*;

static FILENAME : &str = "words_alpha.txt";

pub fn bench_load(c: &mut Criterion) {
    let mut g = c.benchmark_group("loading");
    g.bench_function("load words", |b| b.iter(|| load(FILENAME)));
}

pub fn bench_neighborhoods(c: &mut Criterion) {
    let (_,bits) = load(FILENAME);
    let mut g = c.benchmark_group("clique search");
    g.sampling_mode(SamplingMode::Flat);
    g.sample_size(10);
    g.bench_function("generate neighborhoods", |b| b.iter(|| generate_neighborhoods(black_box(&bits))));
    g.finish();
}

pub fn bench_remove_anagrams(c: &mut Criterion) {
    let (words, bits) = load(FILENAME);
    let mut g = c.benchmark_group("loading");
    g.sample_size(50);
    g.bench_function("remove anagrams", |b| b.iter(|| remove_anagrams(black_box((words.clone(), bits.clone())))));
    g.finish();
}

pub fn bench_init_dancing_links(c: &mut Criterion) {
    let (_, bits) = remove_anagrams(load(FILENAME));
    let mut g = c.benchmark_group("dancing links");
    g.bench_function("initialize", |b| b.iter(|| init_dancing_links(&bits)));
    g.finish();
}

pub fn bench_solve_dancing_links(c: &mut Criterion) {
    let (_, bits) = remove_anagrams(load(FILENAME));
    let dancing_links = init_dancing_links(&bits);
    let mut g = c.benchmark_group("dancing links");
    g.sampling_mode(SamplingMode::Flat);
    g.sample_size(10);
    g.measurement_time(Duration::from_secs(20));
    g.bench_function("solve", |b| b.iter(|| dancing_links.clone().solve()));
    g.finish();
}

pub fn bench_dancing_links(c: &mut Criterion) {
    let mut g = c.benchmark_group("dancing links");
    g.sampling_mode(SamplingMode::Flat);
    g.sample_size(10);
    g.measurement_time(Duration::from_secs(25));
    g.bench_function("end-to-end solution", |b| b.iter(|| {
        let (words, bits) = remove_anagrams(load(FILENAME));
        let mut dancing_links = init_dancing_links(&bits);
        let solutions = dancing_links.solve();
        fmt_solutions(&words, solutions)
    }));
    g.finish();
}

pub fn bench_clique_search(c: &mut Criterion) {
    let mut g = c.benchmark_group("clique search");
    g.sampling_mode(SamplingMode::Flat);
    g.sample_size(10);
    g.measurement_time(Duration::from_secs(25));
    g.bench_function("end-to-end solution", |b| b.iter(|| {
        let (words, bits) = remove_anagrams(load(FILENAME));
        let neighborhoods = generate_neighborhoods(&bits);
        let solutions = find_cliques(neighborhoods);
        fmt_solutions(&words, solutions)
    }));
    g.finish();
}

criterion_group!(
    unit_benches,
    bench_load, bench_remove_anagrams, bench_init_dancing_links, bench_solve_dancing_links
);
criterion_group!(
    integration_bench,
    bench_dancing_links
);
criterion_group!(
    comparison_bench,
    bench_clique_search
);
criterion_main!(unit_benches, integration_bench, comparison_bench);
