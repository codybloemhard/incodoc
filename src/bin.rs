const TEST: &str = "
    nav {
        snav { \"A\", link { \"link\", \"a\" }, link { \"link\", \"b\" } },
        snav { \"B\", link { \"link\", \"c\" }, snav { \"C\", link { \"link\", \"d\" } } }
    },
    par { list { dl, 'item' }, link { \"mailto:e@mail.com\", \"link\" } },
    head { 0, 'heading with an '{ tags { \"tag\" } }, em{ le, \"emphasis\" } },
    list {
        il,
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
        'This is a ', em{le, \"light\"}, ' emphasis.',
        list { dl, 'item', 'item', 'item' },
        link { \"file\", \"link\" },
        tags { \"doc tag\", \"nother one\" },
        props { (\"prop\", 0) }
    },
    section {
        head { 0, 'heading' },
        par { 'paragraph' },
        section {
            head { 1, 'heading' },
            par { 'paragraph' }
        },
        section {
            head { 1, 'heading' },
            par { 'paragraph' },
            section {
                head { 2, 'heading' },
                par { 'paragraph' }
            }
        }
    },
    link { \"www\", \"show\", em { le, \"em\" }, \"yay\", props { (\"prop\", 0) } }
";

fn main() {
    match incodoc::parse(TEST) {
        Ok(res) => println!("{:#?}", res),
        Err(err) => println!("{}", err),
    }
}
