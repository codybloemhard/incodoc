const TEST: &str = "
    meta{(\"test\", 2010/08/06)}
";

fn main() {
    println!("{:?}", incodoc::parse(TEST));
}
