use criterion::{criterion_group, criterion_main, Criterion};

use markdown_table_formatter::format_tables;

fn bench(bencher: &mut Criterion) {
    let start = "| | | | | |\n|-|-|-|-|-|\n";
    let unicode_lines = "|asdf j;agj;asdlkj as;l|dgj a;woietl;,mxc,mngar;lj09238903|4t[ergopgsfdmăܦؤـٷࡀͥ݁̃Ա̱Āضࡗ҆Բ͗ࠡŴΖΆ|ԁĄࠑݓ΅ɓԶѥƘҕ࠹ɇঐ̨ԢظٰΆԁĄࠑ|ݓ΅ɓԶѥƘҕ࠹ɇঐ̨Ԣظٰ|\n".repeat(1_000);
    let bench_text = format!("{}{}", start, unicode_lines);

    bencher.bench_function("table with random Unicode", |b| {
        b.iter(|| format_tables(&bench_text))
    });
}

criterion_group!(benches, bench);
criterion_main!(benches);
