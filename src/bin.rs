const TEST: &str = "
    meta{ (
        \"test\",
        '
        this



        is text



        '
    ) },
";

fn main() {
    match incodoc::parse(TEST) {
        Ok(res) => println!("{:?}", res),
        Err(err) => println!("{}", err),
    }
}
