use incodoc::{
    parsing::parse,
    output::doc_out,
    reference_doc::REF_DOC,
    actions::*,
};

const README: &str = "
props{
    (\"language\", \"en\"),
    (\"date\", 9999/12/31),
},
nav{
    nav{
        \"top level\",
        link{\"/home\", \"home\"},
        link{\"/about\", \"about\"},
        link{\"/now\", \"now\"},
        link{\"/blog/index\", \"blog\"},
    },
    nav{
        \"other articles\",
        link{\"./cheese\", \"I like cheese\"},
        link{\"./sosig\", \"Sosig is happiness\"},
    },
},
section {
    head{0, \"very important\"},
    par{
        'One must see this image.',
        link{\"image.png\", \"very important image\", props{(\"bg-text\", 'Extremely important image.')}},
        'Also this one.',
        link{\"website.com/image\", \"another important image\", props{(\"type-hint\", 'image')}},
        'For further questions see ',
        link{\"#questions\", \"questions\"},
        '.',
    },
    section{
        head{1, \"questions\", tags{\"#questions\"}},
        par{
            '
            Why is this important?
            Because it is.
            For even further questions email me at ',
            link{\"email@address.com\", \"email@address.com\"},
            '.',
        },
    },
    par{
        'This is will not compile:',
        code{
            \"rust\",
            \"show\",
            '
                let mut x = 0;
                let y = &mut x;
                let z = &mut x;
                *y = 1;
            ',
        },
        '
        Your viewer may try to run it, only if they wants to.
        This is a piece of code that suggest to be ran and its result inserted into this document.
        Only if you want to.
        ',
        code{
            \"typst-formula\",
            \"replace\",
            '
                x -> y
            ',
        },
    },
    par{
        'A very logical table:',
        table{
            throw{
                par { 'φ' },
                par { '¬φ' },
            },
            trow {
                par { 'T' },
                par { 'F' },
            },
            trow {
                par { 'F' },
                par { 'T' },
            },
        },
    },
},
par{
    props{(\"type\", \"footer\")},
    '
    Copyright (c) 1337 me
    ',
    props { (\"date\", 2000/00/00) },
},
";

const TEST: &str = "
par {
    table {
        throw {
            tags { \"a\" },
        },
        trow {
            tags { \"b\" },
        },
        trow {
            tags { \"c\" },
        },
    },
}
";

fn main() {
    let res = parse(REF_DOC);
    match res {
        Ok(res) => {
            // println!("{:#?}", res);
            let mut output = String::new();
            doc_out(&res, &mut output);
            println!("{output}");
            println!("{:?}", res.get_table_of_contents());
        },
        Err(err) => println!("{err}"),
    }
}
