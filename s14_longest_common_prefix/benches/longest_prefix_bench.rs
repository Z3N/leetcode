use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

use rand::distributions::{Alphanumeric, DistString};
use rand::Rng;
use s14_longest_common_prefix::Solution;

fn bench_lcp(c: &mut Criterion) {
    let mut group = c.benchmark_group("Longest common prefix");

    let total_random = (0..10000)
        .map(|_| {
            Alphanumeric.sample_string(
                &mut rand::thread_rng(),
                rand::thread_rng().gen_range(1..100),
            )
        })
        .collect::<Vec<String>>();
    let leading_long = (0..10000)
        .map(|_| "DJkdfjksjIidsfkjsksjjsdfsdfkjwiusdfskjskdklsjdfkjsljflsjfklsjfkljslkjflksjflkjsklfjsljflksjflksjldkfjslkjflkdjsalkfjdsalkfjlsjfkdsjkljdskljfksljflksdjfklsdjdlkfjsalkfjlsdkjalkfdjslkajdlajlkdfjkljdfjkdjkajdkuiudjkjfklsjfsdjlafjldskjlkfadsjlfjslkdjfldsajlkfjdoifuoiqweuoidfjlkgjfdlkgjdiofguildfgjlkfdjskglfjlkgjfkljgiodfuoirpiuiuoigulgjflkdjfklgjsdfl".to_string()
            + Alphanumeric.sample_string(&mut rand::thread_rng(), rand::thread_rng().gen_range(1..100)).as_str())
        .collect::<Vec<String>>();
    let leading_short = (0..10000)
        .map(|_| {
            "fu".to_string()
                + Alphanumeric
                    .sample_string(
                        &mut rand::thread_rng(),
                        rand::thread_rng().gen_range(1..100),
                    )
                    .as_str()
        })
        .collect::<Vec<String>>();

    group.bench_with_input(
        BenchmarkId::new("total_random + iter", "Random 10000 chars string iter"),
        &total_random,
        |b, i| b.iter(|| Solution::longest_common_prefix_iter(i.to_vec())),
    );
    group.bench_with_input(
        BenchmarkId::new("total_random + v1", "Random 10000 chars string v1"),
        &total_random,
        |b, i| b.iter(|| Solution::longest_common_prefix_v1(i.to_vec())),
    );
    group.bench_with_input(
        BenchmarkId::new(
            "total_random + combined",
            "Random 10000 chars string combine",
        ),
        &total_random,
        |b, i| b.iter(|| Solution::longest_common_prefix_combined(i.to_vec())),
    );

    group.bench_with_input(
        BenchmarkId::new("leading_long + iter", "Random 10000 chars string iter"),
        &leading_long,
        |b, i| b.iter(|| Solution::longest_common_prefix_iter(i.to_vec())),
    );
    group.bench_with_input(
        BenchmarkId::new("leading_long + v1", "Random 10000 chars string v1"),
        &leading_long,
        |b, i| b.iter(|| Solution::longest_common_prefix_v1(i.to_vec())),
    );
    group.bench_with_input(
        BenchmarkId::new(
            "leading_long + combined",
            "Random 10000 chars string combine",
        ),
        &leading_long,
        |b, i| b.iter(|| Solution::longest_common_prefix_combined(i.to_vec())),
    );

    group.bench_with_input(
        BenchmarkId::new("leading_short + iter", "Random 10000 chars string iter"),
        &leading_short,
        |b, i| b.iter(|| Solution::longest_common_prefix_iter(i.to_vec())),
    );
    group.bench_with_input(
        BenchmarkId::new("leading_short + v1", "Random 10000 chars string v1"),
        &leading_short,
        |b, i| b.iter(|| Solution::longest_common_prefix_v1(i.to_vec())),
    );
    group.bench_with_input(
        BenchmarkId::new(
            "leading_short + combined",
            "Random 10000 chars string combine",
        ),
        &leading_short,
        |b, i| b.iter(|| Solution::longest_common_prefix_combined(i.to_vec())),
    );

    group.finish();
}

criterion_group!(benches, bench_lcp);
criterion_main!(benches);
