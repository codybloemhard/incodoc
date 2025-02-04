const TEST: &str = "
    'test text' {
        tags { \"test tag\" },
        props { (\"test prop\", 0) }
    },
    par{
        code {
            \"rust\",
            \"auto\",
            '
                let x = 3;
            ',
            tags { \"snippet0\", \"snippet-rust\" },
            props { (\"source\", \"www.code.com/user/repo\") }
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
