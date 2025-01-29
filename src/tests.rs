#[cfg(test)]
mod tests {
    use crate::*;

    macro_rules! props {
        ($slice:expr) => {
            HashMap::from($slice)
        }
    }

    macro_rules! hset {
        ($slice:expr) => {
            HashSet::from_iter($slice.iter().map(|s| s.to_string()))
        }
    }

    macro_rules! test {
        ($name:ident, $string:expr, $result:expr) => {
            #[test]
            fn $name() {
                assert_eq!(parse($string), Ok($result));
            }
        }
    }

    test!(
        t_empty_f0,
        "",
        Doc::default()
    );

    test!(
        t_text_c0,
        "'this is text'",
        Doc {
            items: vec![DocItem::Text("this is text".to_string())],
            ..Default::default()
        }
    );

    test!(
        t_text_c1,
        "
        '
        this is text
        '
        ",
        Doc {
            items: vec![DocItem::Text("this is text".to_string())],
            ..Default::default()
        }
    );

    test!(
        t_text_c2,
        "
        '
        this is text
        '
        ",
        Doc {
            items: vec![DocItem::Text("this is text".to_string())],
            ..Default::default()
        }
    );

    test!(
        t_text_c3,
        "
        '
        this is     text
        '
        ",
        Doc {
            items: vec![DocItem::Text("this is text".to_string())],
            ..Default::default()
        }
    );

    test!(
        t_text_c4,
        "
        '
        this
          is     text



        '
        ",
        Doc {
            items: vec![DocItem::Text("this\nis text".to_string())],
            ..Default::default()
        }
    );

    test!(
        t_text_c5,
        "
        ' pre'
        ",
        Doc {
            items: vec![DocItem::Text(" pre".to_string())],
            ..Default::default()
        }
    );

    test!(
        t_text_c6,
        "
        '    pre'
        ",
        Doc {
            items: vec![DocItem::Text(" pre".to_string())],
            ..Default::default()
        }
    );

    test!(
        t_text_c7,
        "
        'post '
        ",
        Doc {
            items: vec![DocItem::Text("post ".to_string())],
            ..Default::default()
        }
    );

    test!(
        t_text_c8,
        "
        'post        '
        ",
        Doc {
            items: vec![DocItem::Text("post ".to_string())],
            ..Default::default()
        }
    );

    test!(
        t_text_c9,
        "
        '   prepost        '
        ",
        Doc {
            items: vec![DocItem::Text(" prepost ".to_string())],
            ..Default::default()
        }
    );

    test!(
        t_text_c10,
        "
        '   pre


        '
        ",
        Doc {
            items: vec![DocItem::Text(" pre".to_string())],
            ..Default::default()
        }
    );

    test!(
        t_props_empty_f0,
        "props{}",
        Doc::default()
    );

    test!(
        t_props_empty_f1,
        "props {}",
        Doc::default()
    );

    test!(
        t_props_empty_f2,
        "props {  }, ",
        Doc::default()
    );

    test!(
        t_props_empty_f3,
        "
        props {

        },
        ",
        Doc::default()
    );

    test!(
        t_props_tuple_string_f0,
        "props{(\"prop\",\"test\")}",
        Doc {
            props: props!([("prop".to_string(), PropVal::String("test".to_string()))]),
            ..Default::default()
        }
    );

    test!(
        t_props_tuple_string_f1,
"
props { (
    \"
    pr
        op\" ,
     \"te
     st
     \"
     ),
},
",
        Doc {
            props: props!([(
                "pr        op".to_string(),
                PropVal::String("te     st     ".to_string())
            )]),
            ..Default::default()
        }
    );

    test!(
        t_props_tuple_int_c0,
        "props { (\"prop\", 5) }",
        Doc {
            props: props!([("prop".to_string(), PropVal::Int(5))]),
            ..Default::default()
        }
    );

    test!(
        t_props_tuple_int_c1,
        "props { (\"prop\", +7) }",
        Doc {
            props: props!([("prop".to_string(), PropVal::Int(7))]),
            ..Default::default()
        }
    );

    test!(
        t_props_tuple_int_c2,
        "props { (\"prop\", -1) }",
        Doc {
            props: props!([("prop".to_string(), PropVal::Int(-1))]),
            ..Default::default()
        }
    );

    test!(
        t_props_tuple_date_c0,
        "props { (\"prop\", 2000/01/01) }",
        Doc {
            props: props!([("prop".to_string(), PropVal::Date(Date::new(2000, 1, 1).unwrap()))]),
            ..Default::default()
        }
    );

    test!(
        t_props_tuple_date_c1,
        "props { (\"prop\", -3434/01/01) }",
        Doc {
            props: props!([("prop".to_string(), PropVal::Date(Date::new(-3434, 1, 1).unwrap()))]),
            ..Default::default()
        }
    );

    test!(
        t_props_tuple_date_c2,
        "props { (\"prop\", 2000/13/01) }",
        Doc {
            props: props!([
                ("prop".to_string(), PropVal::Error(PropValError::Date(DateError::MonthRange(13))))
            ]),
            ..Default::default()
        }
    );

    test!(
        t_props_tuple_date_c3,
        "props { (\"prop\", 2000/01/32) }",
        Doc {
            props: props!([
                ("prop".to_string(), PropVal::Error(PropValError::Date(DateError::DayRange(32))))
            ]),
            ..Default::default()
        }
    );

    test!(
        t_props_tuple_text_c0,
        "props { (\"prop\", 'this is text') }",
        Doc {
            props: props!([("prop".to_string(), PropVal::Text("this is text".to_string()))]),
            ..Default::default()
        }
    );

    test!(
        t_props_tuple_text_c1,
        // looks incorrect because of the escape characters for "
        "
        props { (\"prop\", '
                        this is text
        ') }",
        Doc {
            props: props!([("prop".to_string(), PropVal::Text("this is text".to_string()))]),
            ..Default::default()
        }
    );

    test!(
        t_props_absorb_c0,
        "props { (\"a\", 0), (\"b\", 0) }, props { (\"c\", 1) }",
        Doc {
            props: props!([
                ("a".to_string(), PropVal::Int(0)),
                ("b".to_string(), PropVal::Int(0)),
                ("c".to_string(), PropVal::Int(1)),
            ]),
            ..Default::default()
        }
    );

    test!(
        t_props_absorb_c1,
        "props { (\"a\", 0), (\"b\", 0) }, props { (\"b\", 1) }",
        Doc {
            props: props!([
                ("a".to_string(), PropVal::Int(0)),
                ("b".to_string(), PropVal::Int(1)),
            ]),
            ..Default::default()
        }
    );

    test!(
        t_props_absorb_c2,
        "props { (\"a\", 0), (\"b\", 2000/13/01) }, props { (\"b\", 1) }",
        Doc {
            props: props!([
                ("a".to_string(), PropVal::Int(0)),
                ("b".to_string(), PropVal::Int(1)),
            ]),
            ..Default::default()
        }
    );

    test!(
        t_props_absorb_c3,
        "
        props { (\"a\", 0), (\"b\", 0), (\"a\", 1) },
        props { (\"b\", 2000/13/01) }
        ",
        Doc {
            props: props!([
                ("a".to_string(), PropVal::Int(1)),
                ("b".to_string(), PropVal::Int(0)),
            ]),
            ..Default::default()
        }
    );

    test!(
        t_props_absorb_c4,
        "
        props { (\"a\", 0), (\"b\", 2000/13/01), (\"a\", 1) },
        props { (\"b\", 2000/14/01) }
        ",
        Doc {
            props: props!([
                ("a".to_string(), PropVal::Int(1)),
                ("b".to_string(), PropVal::Error(PropValError::Date(DateError::MonthRange(14)))),
            ]),
            ..Default::default()
        }
    );

    test!(
        t_props_overwrite_c0,
        "
        props { (\"a\", 0), (\"b\", 2000/13/01), (\"a\", 1), (\"b\", 2000/14/01) },
        ",
        Doc {
            props: props!([
                ("a".to_string(), PropVal::Int(1)),
                ("b".to_string(), PropVal::Error(PropValError::Date(DateError::MonthRange(14)))),
            ]),
            ..Default::default()
        }
    );

    test!(
        t_props_overwrite_c1,
        "
        props { (\"a\", 0), (\"b\", 2000/13/01), (\"a\", 1), (\"b\", 2) },
        ",
        Doc {
            props: props!([
                ("a".to_string(), PropVal::Int(1)),
                ("b".to_string(), PropVal::Int(2)),
            ]),
            ..Default::default()
        }
    );

    test!(
        t_props_overwrite_c2,
        "
        props { (\"a\", 0), (\"b\", 0), (\"a\", 1), (\"b\", 2000/15/01) },
        ",
        Doc {
            props: props!([
                ("a".to_string(), PropVal::Int(1)),
                ("b".to_string(), PropVal::Int(0)),
            ]),
            ..Default::default()
        }
    );

    test!(
        t_tags_c0_f0,
        "tags{\"tag0\", \"tag1\"}",
        Doc {
            tags: hset!(["tag0", "tag1"]),
            ..Default::default()
        }
    );

    test!(
        t_tags_c0_f1,
        "tags {
            \"tag0\",
            \"tag1\"
        },",
        Doc {
            tags: hset!(["tag0", "tag1"]),
            ..Default::default()
        }
    );

    test!(
        t_tags_c1,
        "tags{}",
        Doc {
            ..Default::default()
        }
    );

    test!(
        t_tags_c2,
        "
        tags{\"tag0\"},
        tags{\"tag1\"},
        ",
        Doc {
            tags: hset!(["tag0", "tag1"]),
            ..Default::default()
        }
    );

    test!(
        t_tags_c3,
        "
        tags{\"tag0\"},
        tags{\"tag0\"},
        ",
        Doc {
            tags: hset!(["tag0"]),
            ..Default::default()
        }
    );

    test!(
        t_emphasis_c0_f0,
        "em{le, 'light emphasis'}",
        Doc {
            items: vec![DocItem::Emphasis(Emphasis{
                strength: EmStrength::Light,
                etype: EmType::Emphasis,
                text: "light emphasis".to_string(),
            })],
            ..Default::default()
        }

    );

    test!(
        t_emphasis_c0_f1,
        " em{
            le   ,
            'light emphasis'
        }",
        Doc {
            items: vec![DocItem::Emphasis(Emphasis{
                strength: EmStrength::Light,
                etype: EmType::Emphasis,
                text: "light emphasis".to_string(),
            })],
            ..Default::default()
        }

    );

    test!(
        t_emphasis_c1,
        "em{me, 'medium emphasis'}",
        Doc {
            items: vec![DocItem::Emphasis(Emphasis{
                strength: EmStrength::Medium,
                etype: EmType::Emphasis,
                text: "medium emphasis".to_string(),
            })],
            ..Default::default()
        }

    );

    test!(
        t_emphasis_c2,
        "em{se, 'strong emphasis'}",
        Doc {
            items: vec![DocItem::Emphasis(Emphasis{
                strength: EmStrength::Strong,
                etype: EmType::Emphasis,
                text: "strong emphasis".to_string(),
            })],
            ..Default::default()
        }

    );

    test!(
        t_emphasis_c3,
        "em{ld, 'light deemphasis'}",
        Doc {
            items: vec![DocItem::Emphasis(Emphasis{
                strength: EmStrength::Light,
                etype: EmType::Deemphasis,
                text: "light deemphasis".to_string(),
            })],
            ..Default::default()
        }

    );

    test!(
        t_emphasis_c4,
        "em{md, 'medium deemphasis'}",
        Doc {
            items: vec![DocItem::Emphasis(Emphasis{
                strength: EmStrength::Medium,
                etype: EmType::Deemphasis,
                text: "medium deemphasis".to_string(),
            })],
            ..Default::default()
        }

    );

    test!(
        t_emphasis_c5,
        "em{sd, 'strong deemphasis'}",
        Doc {
            items: vec![DocItem::Emphasis(Emphasis{
                strength: EmStrength::Strong,
                etype: EmType::Deemphasis,
                text: "strong deemphasis".to_string(),
            })],
            ..Default::default()
        }

    );

    test!(
        t_emphasis_c6,
        "
        'This is a ',
        em{le, 'light'},
        ' emphasis.',
        ",
        Doc {
            items: vec![
                DocItem::Text("This is a ".to_string()),
                DocItem::Emphasis(Emphasis{
                    strength: EmStrength::Light,
                    etype: EmType::Emphasis,
                    text: "light".to_string(),
                }),
                DocItem::Text(" emphasis.".to_string()),
            ],
            ..Default::default()
        }

    );

    test!(
        t_code_c0_f0,
        "code { \"plain\", \"show\", '' }",
        Doc {
            items: vec![DocItem::Code(CodeBlock{
                language: "plain".to_string(),
                ..Default::default()
            })],
            ..Default::default()
        }
    );

    test!(
        t_code_c0_f1,
        "code
        {
    \"plain\",
            \"show\",
                ''}",
        Doc {
            items: vec![DocItem::Code(CodeBlock{
                language: "plain".to_string(),
                ..Default::default()
            })],
            ..Default::default()
        }
    );

    test!(
        t_code_c1,
        "code { \"plain\", \"choice\", '' }",
        Doc {
            items: vec![DocItem::Code(CodeBlock{
                language: "plain".to_string(),
                mode: CodeModeHint::Choice,
                ..Default::default()
            })],
            ..Default::default()
        }
    );

    test!(
        t_code_c2,
        "code { \"plain\", \"auto\", '' }",
        Doc {
            items: vec![DocItem::Code(CodeBlock{
                language: "plain".to_string(),
                mode: CodeModeHint::Auto,
                ..Default::default()
            })],
            ..Default::default()
        }
    );

    test!(
        t_code_c3,
        "code { \"plain\", \"replace\", '' }",
        Doc {
            items: vec![DocItem::Code(CodeBlock{
                language: "plain".to_string(),
                mode: CodeModeHint::Replace,
                ..Default::default()
            })],
            ..Default::default()
        }
    );

    test!(
        t_code_c4,
        "code { \"plain\", \"not a mode!\", '' }",
        Doc {
            items: vec![DocItem::Code(CodeBlock{
                language: "plain".to_string(),
                ..Default::default()
            })],
            ..Default::default()
        }
    );

    test!(
        t_code_c5,
        "code {
            \"plain\",
            \"show\",
            '
            this is code
            '
        }",
        Doc {
            items: vec![DocItem::Code(CodeBlock{
                language: "plain".to_string(),
                code: "this is code".to_string(),
                ..Default::default()
            })],
            ..Default::default()
        }
    );

    test!(
        t_code_c6,
        "code {
            \"plain\",
            \"show\",
            'this is code
            '
        }",
        Doc {
            items: vec![DocItem::Code(CodeBlock{
                language: "plain".to_string(),
                code: "this is code".to_string(),
                ..Default::default()
            })],
            ..Default::default()
        }
    );

    test!(
        t_code_c7,
        "code {
            \"plain\",
            \"show\",
            '
                this is code
            '
        }",
        Doc {
            items: vec![DocItem::Code(CodeBlock{
                language: "plain".to_string(),
                code: "    this is code".to_string(),
                ..Default::default()
            })],
            ..Default::default()
        }
    );

    test!(
        t_code_c8,
        "code {
            \"plain\",
            \"show\",
            '

                this is code
            '
        }",
        Doc {
            items: vec![DocItem::Code(CodeBlock{
                language: "plain".to_string(),
                code: "\n    this is code".to_string(),
                ..Default::default()
            })],
            ..Default::default()
        }
    );

    test!(
        t_code_c9,
        "code {
            \"plain\",
            \"show\",
            '
            this is code

            '
        }",
        Doc {
            items: vec![DocItem::Code(CodeBlock{
                language: "plain".to_string(),
                code: "this is code\n".to_string(),
                ..Default::default()
            })],
            ..Default::default()
        }
    );

    test!(
        t_code_c10,
        "code {
            \"plain\",
            \"show\",
            '

                this is code
                 more code

            '
        }",
        Doc {
            items: vec![DocItem::Code(CodeBlock{
                language: "plain".to_string(),
                code: "\n    this is code\n     more code\n".to_string(),
                ..Default::default()
            })],
            ..Default::default()
        }
    );

    test!(
        t_code_c11,
        "code {
            \"plain\",
            \"show\",
            '
           this is code
            '
        }",
        Doc {
            errors: vec![DocError::Code(CodeError::Ident(CodeIdentError))],
            ..Default::default()
        }
    );

    test!(
        t_code_c12,
        "code {
            \"plain\",
            \"show\",
            '
            this is code
            ',
            props { (\"test tuple\", 'yay') }
        }",
        Doc {
            items: vec![DocItem::Code(CodeBlock{
                language: "plain".to_string(),
                code: "this is code".to_string(),
                props: props!([("test tuple".to_string(), PropVal::Text("yay".to_string()))]),
                ..Default::default()
            })],
            ..Default::default()
        }
    );

    test!(
        t_code_c13,
        "code {
            \"plain\",
            \"show\",
            '
            this is code
            ',
            props { (\"test tuple\", 'yay') },
            tags { \"tag\", \"nother tag\" }
        }",
        Doc {
            items: vec![DocItem::Code(CodeBlock{
                language: "plain".to_string(),
                code: "this is code".to_string(),
                props: props!([("test tuple".to_string(), PropVal::Text("yay".to_string()))]),
                tags: hset!(["tag", "nother tag"]),
                ..Default::default()
            })],
            ..Default::default()
        }
    );
}
