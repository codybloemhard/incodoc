const TEST: &str = "
    meta{ (
        \"test\",
        '

            hello
            '
    ) },
";

fn main() {
    match incodoc::parse(TEST) {
        Ok(res) => println!("{:?}", res),
        Err(err) => println!("{}", err),
    }
}
