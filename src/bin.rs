const TEST: &str = "
    code {
        \"rust\",
        \"auto\",
        '
            let x = 3;
        ',
        meta { (\"source\", \"www.code.com/user/repo\") },
        tags { \"snippet0\", \"snippet-rust\" }
    },
    tags { \"doc tag\", \"nother one\" },
    tags { \"ok sorry\" }
";

fn main() {
    match incodoc::parse(TEST) {
        Ok(res) => println!("{:?}", res),
        Err(err) => println!("{}", err),
    }
}
