const TEST: &str = "
    meta{ (
        \"test\",
        \"
        hello
        \"
    ) },
";

fn main() {
    println!("{:?}", incodoc::parse(TEST));
}
