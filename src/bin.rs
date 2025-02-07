const TEST: &str = "
    head { 0, 'heading with an '{ tags { \"tag\" } }, em{ le, 'emphasis' } },
    list { il,
    'test text' {
        tags { \"test tag\" },
        props { (\"test prop\", 0) }
    },
    code {
        \"rust\",
        \"auto\",
        '
            let x = 3;
        ',
        tags { \"snippet0\", \"snippet-rust\" },
        props { (\"source\", \"www.code.com/user/repo\") }
    },
    'This is a ', em{le, 'light'}, ' emphasis.',
    list { dl, 'item', 'item', 'item' },
    tags { \"doc tag\", \"nother one\" },
    props { (\"prop\", 0) }
    }
";

fn main() {
    match incodoc::parse(TEST) {
        Ok(res) => println!("{:#?}", res),
        Err(err) => println!("{}", err),
    }
}
