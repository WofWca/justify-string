#![feature(test)]

extern crate test;

#[cfg(test)]
mod tests {
    use justify_string::justify;
    use test::Bencher;

    fn bench_rust_wikipedia(b: &mut Bencher, line_width: u32) {
        let input = "Rust is an iron oxide, a usually red oxide formed by the redox reaction of iron and oxygen in the presence of water or air moisture. Several forms of rust are distinguishable both visually and by spectroscopy, and form under different circumstances. Rust consists of hydrated iron(III) oxides Fe2O3·nH2O and iron(III) oxide-hydroxide (FeO(OH), Fe(OH)3).\n\nGiven sufficient time, oxygen, and water, any iron mass will eventually convert entirely to rust and disintegrate. Surface rust is flaky and friable, and it provides no protection to the underlying iron, unlike the formation of patina on copper surfaces. Rusting is the common term for corrosion of iron and its alloys, such as steel. Many other metals undergo similar corrosion, but the resulting oxides are not commonly called rust.";

        b.iter(|| justify(
            test::black_box(input),
            line_width,
        ));
    }
    #[bench]
    fn bench_rust_wikipedia_unknown_width(b: &mut Bencher) {
        bench_rust_wikipedia(b, test::black_box(80));
    }
    #[bench]
    fn bench_rust_wikipedia_fixed_width(b: &mut Bencher) {
        bench_rust_wikipedia(b, 80);
    }
}
