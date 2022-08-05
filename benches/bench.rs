use criterion::*;
use five_letters::*;

static filename : &str = "words_alpha.txt";

pub fn bench_load(c: &mut Criterion) {
    c.bench_function("load words", |b| b.iter(|| load(filename)));
}

pub fn bench_neighborhoods(c: &mut Criterion) {
    let (_,bits) = load(filename);
    c.bench_function("generate neighborhoods", |b| b.iter(|| generate_neighborhoods(black_box(&bits))));
}

pub fn bench_remove_anagrams(c: &mut Criterion) {
    let (words, bits) = load(filename);
    c.bench_function("remove anagrams", |b| b.iter(|| remove_anagrams(black_box((words.clone(), bits.clone())))));
}

pub fn bench_init_dancing_links(c: &mut Criterion) {
    let (_, bits) = remove_anagrams(load(filename));
    c.bench_function("initialize dancing links", |b| b.iter(|| init_dancing_links(&bits)));
}

pub fn bench_solve_dancing_links(c: &mut Criterion) {
    let (_, bits) = remove_anagrams(load(filename));
    let dancing_links = init_dancing_links(&bits);
    c.bench_function("solve dancing links", |b| b.iter(|| dancing_links.clone().solve()));
}

criterion_group!{
    name=benches;
    config=Criterion::default().sample_size(10);
    targets=bench_load, bench_remove_anagrams, bench_init_dancing_links, bench_solve_dancing_links
}
criterion_main!(benches);
