const TEST: &str = "
    code {
        \"rust\",
        \"auto\",
        '
            let x = 3;
        ',
        meta { (\"source\", \"www.code.com/user/repo\") }
    }
";

fn main() {
    match incodoc::parse(TEST) {
        Ok(res) => println!("{:?}", res),
        Err(err) => println!("{}", err),
    }
}
