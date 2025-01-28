const TEST: &str = "
    par{
        code {
            \"rust\",
            \"auto\",
            '
                let x = 3;
            ',
            props { (\"source\", \"www.code.com/user/repo\") },
            tags { \"snippet0\", \"snippet-rust\" }
        },
        tags { \"doc tag\", \"nother one\" },
        tags { \"ok sorry\" },
        'This is a ', em{le, 'light'}, ' emphasis.',
    }
";

fn main() {
    match incodoc::parse(TEST) {
        Ok(res) => println!("{:#?}", res),
        Err(err) => println!("{}", err),
    }
}
