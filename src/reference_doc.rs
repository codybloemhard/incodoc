/// An incodoc reference document.
/// Contains all features of incodoc.
/// A quick overview of the syntax as well as quick test material.
pub const REF_DOC: &str = "
    tags { \"tag-a\", \"tag-b\" },
    props {
        (\"prop-string\", \"hi\"),
        (\"prop-text\", 'text'),
        (\"prop-int\", 26),
        (\"prop-date\", 2000/01/11),
    },
    par {
        'text',
        'text with meta' {
            tags { \"tag\" },
            props { (\"prop\", 0) },
        },
        em { le, \"light emphasis\", tags { \"tag\" }, props { (\"prop\", 0) } },
        em { me, \"medium emphasis\", tags { \"tag\" }, props { (\"prop\", 0) } },
        em { se, \"strong emphasis\", tags { \"tag\" }, props { (\"prop\", 0) } },
        em { ld, \"light deemphasis\", tags { \"tag\" }, props { (\"prop\", 0) } },
        em { md, \"medium deemphasis\", tags { \"tag\" }, props { (\"prop\", 0) } },
        em { sd, \"strong deemphasis\", tags { \"tag\" }, props { (\"prop\", 0) } },
        code { \"rust\", \"show\", 'let x = 0;', tags { \"tag\" }, props { (\"prop\", 0) } },
        code { \"rust\", \"choice\", 'let x = 0;', tags { \"tag\" }, props { (\"prop\", 0) } },
        code { \"rust\", \"auto\", 'let x = 0;', tags { \"tag\" }, props { (\"prop\", 0) } },
        code { \"rust\", \"replace\", 'let x = 0;', tags { \"tag\" }, props { (\"prop\", 0) } },
        link { \"url\", \"link string\", tags { \"tag\" }, props { (\"prop\", 0) } },
        list {
            il,
            par {
                'text',
                em { le, \"light emphasis\" },
                code { \"rust\", \"show\", 'let x = 0' },
                link { \"url\", \"link\" },
                list { il, par { 'item 0' }, par { 'item 1' } },
                tags { \"tag\" },
                props { (\"prop\", 0) },
            },
            tags { \"tag\" },
            props { (\"prop\", 0) },
        },
    },
    section {
        head {
            0,
            \"heading with an \",
            em { le, \"emphasised\", tags { \"tag\" }, props { (\"prop\", 0) } },
            \" part\",
        },
        par { 'paragraph' },
        section {
            head { 1, \"heading\" },
            par { 'paragraph' }
        },
        section {
            head { 1, \"heading\" },
            par { 'paragraph' },
            section {
                head { 2, \"heading\" },
                par { 'paragraph' }
            }
        }
    },
    nav {
        snav {
            \"description A\",
            link { \"url-a\", \"link string a\" },
            link { \"url-b\", \"link string b\" },
        },
        snav {
            \"description B\",
            link { \"url-c\", \"link string c\" },
            snav {
                \"description C\",
                link { \"url-d\", \"link string d\" },
            },
        },
    },
";

