macro_rules! bench {
    ($group:expr, $group_name:ident / $name:literal, | $b:ident | $body:expr) => {{
        const_format::assertcp!(
            $group_name.len() + 1 + $name.len() <= 23,
            "`{}` is too long, max {}ch",
            $name,
            23 - $group_name.len() - 1
        );
        $group.bench_function($name, |$b| $body);
    }};
}
