use case_insensitive_string::CaseInsensitiveString;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::collections::hash_map::DefaultHasher;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};

fn hash_one(s: &CaseInsensitiveString) -> u64 {
    let mut h = DefaultHasher::new();
    s.hash(&mut h);
    h.finish()
}

fn bench_hash(c: &mut Criterion) {
    let short = CaseInsensitiveString::new("https://Example.COM");
    let medium = CaseInsensitiveString::new(
        "https://www.Example.COM/Blog/Some-Article-Title?page=2&ref=HOME",
    );
    let long = CaseInsensitiveString::new(&"https://cdn.Example.COM/assets/images/".repeat(10));

    c.bench_function("hash_short_19b", |b| b.iter(|| hash_one(black_box(&short))));
    c.bench_function("hash_medium_66b", |b| {
        b.iter(|| hash_one(black_box(&medium)))
    });
    c.bench_function("hash_long_390b", |b| b.iter(|| hash_one(black_box(&long))));
}

fn bench_eq(c: &mut Criterion) {
    let a = CaseInsensitiveString::new("https://Example.COM/Path");
    let b = CaseInsensitiveString::new("https://example.com/path");

    c.bench_function("eq_24b", |b2| b2.iter(|| black_box(&a) == black_box(&b)));
}

fn bench_cmp(c: &mut Criterion) {
    let a = CaseInsensitiveString::new("https://Alpha.COM/zzz");
    let b = CaseInsensitiveString::new("https://beta.com/aaa");

    c.bench_function("cmp_21b", |bench| {
        bench.iter(|| black_box(&a).cmp(black_box(&b)))
    });
}

fn bench_hashset_insert(c: &mut Criterion) {
    let urls: Vec<CaseInsensitiveString> = (0..1000)
        .map(|i| CaseInsensitiveString::new(&format!("https://example.com/page/{}", i)))
        .collect();

    c.bench_function("hashset_insert_1000", |b| {
        b.iter(|| {
            let mut set = HashSet::with_capacity(1000);
            for u in &urls {
                set.insert(black_box(u));
            }
            set
        })
    });
}

fn bench_hashset_contains(c: &mut Criterion) {
    let urls: Vec<CaseInsensitiveString> = (0..1000)
        .map(|i| CaseInsensitiveString::new(&format!("https://example.com/page/{}", i)))
        .collect();
    let set: HashSet<&CaseInsensitiveString> = urls.iter().collect();

    c.bench_function("hashset_contains_1000", |b| {
        b.iter(|| {
            for u in &urls {
                black_box(set.contains(black_box(u)));
            }
        })
    });
}

criterion_group!(
    benches,
    bench_hash,
    bench_eq,
    bench_cmp,
    bench_hashset_insert,
    bench_hashset_contains,
);
criterion_main!(benches);
