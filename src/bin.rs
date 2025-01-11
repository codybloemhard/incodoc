const TEST: &str = "
    code {
        \"rust\",
        \"auto\",
        '
            let x = 3;
        '
    }
";

fn main() {
    match incodoc::parse(TEST) {
        Ok(res) => println!("{:?}", res),
        Err(err) => println!("{}", err),
    }
}
