#[cfg(test)]
mod tests {
    use crate::*;
    use crate::parsing::*;
    use crate::output::*;

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
                let doc_a_raw = parse($string);
                assert_eq!(doc_a_raw, Ok($result));
                if let Ok(mut doc_a) = doc_a_raw {
                    let mut output = String::new();
                    doc_out(&doc_a, &mut output);
                    let doc_b = parse(&output).expect("test_out: could not parse doc b");
                    doc_a.remove_errors();
                    assert_eq!(doc_a, doc_b);
                }
            }
        }
    }

    test!(
        t_empty,
        "",
        Doc::default()
    );

    test!(
        t_empty_comment0,
        "// comment",
        Doc::default()
    );

    test!(
        t_empty_comment1,
        "
        /* comment */
        ",
        Doc::default()
    );

    test!(
        t_empty_comment2,
        "
        //
        ",
        Doc::default()
    );

    test!(
        t_empty_comment3,
        "
        /**/
        ",
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
        t_text_meta0,
        "
        'text'{ tags { } }
        ",
        Doc {
            items: vec![DocItem::Text("text".to_string())],
            ..Default::default()
        }
    );

    test!(
        t_text_meta1,
        "
        'text'{ props { } }
        ",
        Doc {
            items: vec![DocItem::Text("text".to_string())],
            ..Default::default()
        }
    );

    test!(
        t_text_meta2,
        "
        'text'{ tags { }, props { } }
        ",
        Doc {
            items: vec![DocItem::Text("text".to_string())],
            ..Default::default()
        }
    );

    test!(
        t_text_meta3,
        "
        'text'{ tags { \"a\", \"b\" } }
        ",
        Doc {
            items: vec![DocItem::MText(TextWithMeta{
                text: "text".to_string(),
                tags: hset!(["a", "b"]),
                props: Props::default(),
            })],
            ..Default::default()
        }
    );

    test!(
        t_text_meta4,
        "
        'text'{ props { (\"a\", 0), (\"b\", 1), (\"a\", 2) } }
        ",
        Doc {
            items: vec![DocItem::MText(TextWithMeta{
                text: "text".to_string(),
                tags: Tags::default(),
                props: props!([
                    ("a".to_string(), PropVal::Int(2)),
                    ("b".to_string(), PropVal::Int(1))]
                ),
            })],
            ..Default::default()
        }
    );

    test!(
        t_text_meta5,
        "
        'text'{ tags { \"a\", \"b\" }, props { (\"a\", 0), (\"b\", 1), (\"a\", 2) } }
        ",
        Doc {
            items: vec![DocItem::MText(TextWithMeta{
                text: "text".to_string(),
                tags: hset!(["a", "b"]),
                props: props!([
                    ("a".to_string(), PropVal::Int(2)),
                    ("b".to_string(), PropVal::Int(1))]
                ),
            })],
            ..Default::default()
        }
    );

    test!(
        t_text_comment0,
        "
        // comment
        'text' // comment
        // comment
        ",
        Doc {
            items: vec![DocItem::Text("text".to_string())],
            ..Default::default()
        }
    );

    test!(
        t_text_comment1,
        "
        /* comment */
        /* comment */ 'text' /* comment */
        /* comment */
        ",
        Doc {
            items: vec![DocItem::Text("text".to_string())],
            ..Default::default()
        }
    );

    test!(
        t_text_comment2,
        "
        // comment
        'text' {
            // comment
            tags { /* comment */\"a\", \"b\" }, // comment
            /* comment */
            props {
                /*comment*/(/*comment*/\"a\"/*comment*/,/*comment*/0/*comment*/),
                (\"b\", 1),
            /* comment */}// comment
            // comment
        /* comment */}
        // comment
        ",
        Doc {
            items: vec![DocItem::MText(TextWithMeta{
                text: "text".to_string(),
                tags: hset!(["a", "b"]),
                props: props!([
                    ("a".to_string(), PropVal::Int(0)),
                    ("b".to_string(), PropVal::Int(1))]
                ),
            })],
            ..Default::default()
        }
    );

    test!(
        t_text_comment3,
        "
        /* comment */
        /* comment */ '//comment' /* comment */,
        /* comment */ '/*comment*/' /* comment */
        /* comment */
        ",
        Doc {
            items: vec![
                DocItem::Text("//comment".to_string()),
                DocItem::Text("/*comment*/".to_string()),
            ],
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
                "    pr        op".to_string(),
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
        t_props_tuple_date_c4,
        "props { (\"prop\", 1/1/1) }",
        Doc {
            props: props!([
                ("prop".to_string(), PropVal::Date(Date::new(1, 1, 1).unwrap())),
            ]),
            ..Default::default()
        }
    );

    test!(
        t_props_tuple_date_c5,
        "props { (\"prop\", 0/0/0) }",
        Doc {
            props: props!([
                ("prop".to_string(), PropVal::Error(PropValError::Date(DateError::MonthRange(0))))
            ]),
            ..Default::default()
        }
    );

    test!(
        t_props_tuple_date_c6,
        "props { (\"prop\", 0/1/0) }",
        Doc {
            props: props!([
                ("prop".to_string(), PropVal::Error(PropValError::Date(DateError::DayRange(0))))
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
        t_props_trailing_comma,
        "props{ (\"a\", 0), }",
        Doc {
            props: props!([("a".to_string(), PropVal::Int(0))]),
            ..Default::default()
        }
    );

    test!(
        t_props_comment,
        "
        // comment
        /*comment*/props/*comment*/{/*comment*/(/*comment*/\"//comment\"/*comment*/,/*comment*/0/*coment*/)/*comment*/}//comment
        /*comment*/
        ",
        Doc {
            props: props!([("//comment".to_string(), PropVal::Int(0))]),
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
        t_tags_c0_f2,
        "tags {
            \"tag0\",
            \"tag1\",
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
        t_tags_comment,
        "
        /* comment */
        /*comment*/tags/*comment*/{/*comment*/\"a\"/*comment*/,/*comment*/\"b\"/*comment*/}/*comment*/,//comment
        // comment
        ",
        Doc {
            tags: hset!(["a", "b"]),
            ..Default::default()
        }
    );

    test!(
        t_emphasis_c0_f0,
        "em{le, \"light emphasis\"}",
        Doc {
            items: vec![DocItem::Emphasis(Emphasis{
                strength: EmStrength::Light,
                etype: EmType::Emphasis,
                text: "light emphasis".to_string(),
                ..Default::default()
            })],
            ..Default::default()
        }

    );

    test!(
        t_emphasis_c0_f1,
        " em{
            le   ,
            \"light emphasis\"
        }",
        Doc {
            items: vec![DocItem::Emphasis(Emphasis{
                strength: EmStrength::Light,
                etype: EmType::Emphasis,
                text: "light emphasis".to_string(),
                ..Default::default()
            })],
            ..Default::default()
        }

    );

    test!(
        t_emphasis_c1,
        "em{me, \"medium emphasis\"}",
        Doc {
            items: vec![DocItem::Emphasis(Emphasis{
                strength: EmStrength::Medium,
                etype: EmType::Emphasis,
                text: "medium emphasis".to_string(),
                ..Default::default()
            })],
            ..Default::default()
        }

    );

    test!(
        t_emphasis_c2,
        "em{se, \"strong emphasis\"}",
        Doc {
            items: vec![DocItem::Emphasis(Emphasis{
                strength: EmStrength::Strong,
                etype: EmType::Emphasis,
                text: "strong emphasis".to_string(),
                ..Default::default()
            })],
            ..Default::default()
        }

    );

    test!(
        t_emphasis_c3,
        "em{ld, \"light deemphasis\"}",
        Doc {
            items: vec![DocItem::Emphasis(Emphasis{
                strength: EmStrength::Light,
                etype: EmType::Deemphasis,
                text: "light deemphasis".to_string(),
                ..Default::default()
            })],
            ..Default::default()
        }

    );

    test!(
        t_emphasis_c4,
        "em{md, \"medium deemphasis\"}",
        Doc {
            items: vec![DocItem::Emphasis(Emphasis{
                strength: EmStrength::Medium,
                etype: EmType::Deemphasis,
                text: "medium deemphasis".to_string(),
                ..Default::default()
            })],
            ..Default::default()
        }

    );

    test!(
        t_emphasis_c5,
        "em{sd, \"strong deemphasis\"}",
        Doc {
            items: vec![DocItem::Emphasis(Emphasis{
                strength: EmStrength::Strong,
                etype: EmType::Deemphasis,
                text: "strong deemphasis".to_string(),
                ..Default::default()
            })],
            ..Default::default()
        }

    );

    test!(
        t_emphasis_c6,
        "
        'This is a ',
        em{le, \"light\"},
        ' emphasis.',
        ",
        Doc {
            items: vec![
                DocItem::Text("This is a ".to_string()),
                DocItem::Emphasis(Emphasis{
                    strength: EmStrength::Light,
                    etype: EmType::Emphasis,
                    text: "light".to_string(),
                    ..Default::default()
                }),
                DocItem::Text(" emphasis.".to_string()),
            ],
            ..Default::default()
        }

    );

    test!(
        t_emphasis_c7,
        "
        em{
            le,
            \"light\",
            tags { \"a\", \"b\" },
        }
        ",
        Doc {
            items: vec![
                DocItem::Emphasis(Emphasis{
                    strength: EmStrength::Light,
                    etype: EmType::Emphasis,
                    text: "light".to_string(),
                    tags: hset!(["a", "b"]),
                    ..Default::default()
                }),
            ],
            ..Default::default()
        }

    );

    test!(
        t_emphasis_c8,
        "
        em{
            le,
            \"light\",
            props { (\"a\", 0), (\"b\", 1) },
        }
        ",
        Doc {
            items: vec![
                DocItem::Emphasis(Emphasis{
                    strength: EmStrength::Light,
                    etype: EmType::Emphasis,
                    text: "light".to_string(),
                    props: props!([
                        ("a".to_string(), PropVal::Int(0)),
                        ("b".to_string(), PropVal::Int(1)),
                    ]),
                    ..Default::default()
                }),
            ],
            ..Default::default()
        }

    );

    test!(
        t_emphasis_c9,
        "
        em{
            le,
            \"light\",
            tags { \"a\", \"b\" },
            props { (\"a\", 0), (\"b\", 1) },
        }
        ",
        Doc {
            items: vec![
                DocItem::Emphasis(Emphasis{
                    strength: EmStrength::Light,
                    etype: EmType::Emphasis,
                    text: "light".to_string(),
                    tags: hset!(["a", "b"]),
                    props: props!([
                        ("a".to_string(), PropVal::Int(0)),
                        ("b".to_string(), PropVal::Int(1)),
                    ]),
                }),
            ],
            ..Default::default()
        }

    );

    test!(
        t_emphasis_comment,
        "/*c*/em/*c*/{/*c*/le/*c*/,/*c*/\"le\"/*c*/,/*c*/tags{}/*c*/,/*c*/props{}/*c*/}//c",
        Doc {
            items: vec![DocItem::Emphasis(Emphasis{
                strength: EmStrength::Light,
                etype: EmType::Emphasis,
                text: "le".to_string(),
                ..Default::default()
            })],
            ..Default::default()
        }

    );

    test!(
        t_code_c0_f0,
        "code { \"plain\", \"show\", '' }",
        Doc {
            items: vec![DocItem::Code(Ok(CodeBlock{
                language: "plain".to_string(),
                ..Default::default()
            }))],
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
            items: vec![DocItem::Code(Ok(CodeBlock{
                language: "plain".to_string(),
                ..Default::default()
            }))],
            ..Default::default()
        }
    );

    test!(
        t_code_c1,
        "code { \"plain\", \"runnable\", '' }",
        Doc {
            items: vec![DocItem::Code(Ok(CodeBlock{
                language: "plain".to_string(),
                mode: CodeModeHint::Runnable,
                ..Default::default()
            }))],
            ..Default::default()
        }
    );

    test!(
        t_code_c2,
        "code { \"plain\", \"run\", '' }",
        Doc {
            items: vec![DocItem::Code(Ok(CodeBlock{
                language: "plain".to_string(),
                mode: CodeModeHint::Run,
                ..Default::default()
            }))],
            ..Default::default()
        }
    );

    test!(
        t_code_c3,
        "code { \"plain\", \"replace\", '' }",
        Doc {
            items: vec![DocItem::Code(Ok(CodeBlock{
                language: "plain".to_string(),
                mode: CodeModeHint::Replace,
                ..Default::default()
            }))],
            ..Default::default()
        }
    );

    test!(
        t_code_c4,
        "code { \"plain\", \"not a mode!\", '' }",
        Doc {
            items: vec![DocItem::Code(Ok(CodeBlock{
                language: "plain".to_string(),
                ..Default::default()
            }))],
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
            items: vec![DocItem::Code(Ok(CodeBlock{
                language: "plain".to_string(),
                code: "this is code".to_string(),
                ..Default::default()
            }))],
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
            items: vec![DocItem::Code(Ok(CodeBlock{
                language: "plain".to_string(),
                code: "this is code".to_string(),
                ..Default::default()
            }))],
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
            items: vec![DocItem::Code(Ok(CodeBlock{
                language: "plain".to_string(),
                code: "    this is code".to_string(),
                ..Default::default()
            }))],
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
            items: vec![DocItem::Code(Ok(CodeBlock{
                language: "plain".to_string(),
                code: "\n    this is code".to_string(),
                ..Default::default()
            }))],
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
            items: vec![DocItem::Code(Ok(CodeBlock{
                language: "plain".to_string(),
                code: "this is code\n".to_string(),
                ..Default::default()
            }))],
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
            items: vec![DocItem::Code(Ok(CodeBlock{
                language: "plain".to_string(),
                code: "\n    this is code\n     more code\n".to_string(),
                ..Default::default()
            }))],
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
            items: vec![DocItem::Code(Err(CodeIdentError))],
            ..Default::default()
        }
    );

    test!(
        t_code_meta0,
        "
        code {
            \"plain\",
            \"show\",
            'this is code',
            tags { \"a\", \"b\" }
        }
        ",
        Doc {
            items: vec![DocItem::Code(Ok(CodeBlock{
                language: "plain".to_string(),
                code: "this is code".to_string(),
                tags: hset!(["a", "b"]),
                ..Default::default()
            }))],
            ..Default::default()
        }
    );

    test!(
        t_code_meta1,
        "
        code {
            \"plain\",
            \"show\",
            'this is code',
            props { (\"a\", 0), (\"b\", 1), (\"a\", 2) }
        }
        ",
        Doc {
            items: vec![DocItem::Code(Ok(CodeBlock{
                language: "plain".to_string(),
                code: "this is code".to_string(),
                props: props!([
                    ("a".to_string(), PropVal::Int(2)),
                    ("b".to_string(), PropVal::Int(1))]
                ),
                ..Default::default()
            }))],
            ..Default::default()
        }
    );

    test!(
        t_code_meta2,
        "
        code {
            \"plain\",
            \"show\",
            'this is code',
            tags { \"a\", \"b\" },
            props { (\"a\", 0), (\"b\", 1), (\"a\", 2) }
        }
        ",
        Doc {
            items: vec![DocItem::Code(Ok(CodeBlock{
                language: "plain".to_string(),
                code: "this is code".to_string(),
                tags: hset!(["a", "b"]),
                props: props!([
                    ("a".to_string(), PropVal::Int(2)),
                    ("b".to_string(), PropVal::Int(1))]
                ),
                ..Default::default()
            }))],
            ..Default::default()
        }
    );

    test!(
        t_code_trailing_comma0,
        "
        code {
            \"plain\",
            \"show\",
            'this is code',
        }
        ",
        Doc {
            items: vec![DocItem::Code(Ok(CodeBlock{
                language: "plain".to_string(),
                code: "this is code".to_string(),
                ..Default::default()
            }))],
            ..Default::default()
        }
    );

    test!(
        t_code_trailing_comma1,
        "
        code {
            \"plain\",
            \"show\",
            'this is code',
            tags { \"a\", \"b\" },
            props { (\"a\", 0), (\"b\", 1), (\"a\", 2) },
        }
        ",
        Doc {
            items: vec![DocItem::Code(Ok(CodeBlock{
                language: "plain".to_string(),
                code: "this is code".to_string(),
                tags: hset!(["a", "b"]),
                props: props!([
                    ("a".to_string(), PropVal::Int(2)),
                    ("b".to_string(), PropVal::Int(1))]
                ),
                ..Default::default()
            }))],
            ..Default::default()
        }
    );

    test!(
        t_code_comment,
        "/*c*/code/*c*/{//c
            /*c*/\"plain\"/*c*/,//c
            /*c*/\"show\"/*c*/,/*c*///c
            /*c*/'
                 // comment
                 '//c
        /*c*/}/*c*/",
        Doc {
            items: vec![DocItem::Code(Ok(CodeBlock{
                language: "plain".to_string(),
                code: "// comment".to_string(),
                ..Default::default()
            }))],
            ..Default::default()
        }
    );

    test!(
        t_paragraph,
        "
        par{
            'Some text.',
            'Code that is ',
            em{se, \"important\"},
            ':',
            code {
                \"rust\",
                \"show\",
                '
                    let i = 0;
                '
            },
            list { il, 'item' }
        },
        'Outside the paragraph.',
        ",
        Doc {
            items: vec![
                DocItem::Paragraph(Paragraph {
                    items: vec![
                        ParagraphItem::Text("Some text.".to_string()),
                        ParagraphItem::Text("Code that is ".to_string()),
                        ParagraphItem::Em(Emphasis {
                            etype: EmType::Emphasis,
                            strength: EmStrength::Strong,
                            text: "important".to_string(),
                            ..Default::default()
                        }),
                        ParagraphItem::Text(":".to_string()),
                        ParagraphItem::Code(Ok(CodeBlock {
                            language: "rust".to_string(),
                            mode: CodeModeHint::Show,
                            code: "    let i = 0;".to_string(),
                            ..Default::default()
                        })),
                        ParagraphItem::List(List {
                            ltype: ListType::Identical,
                            items: vec![ParagraphItem::Text("item".to_string())],
                            ..Default::default()
                        })
                    ],
                    ..Default::default()
                }),
                DocItem::Text("Outside the paragraph.".to_string()),
            ],
            ..Default::default()
        }
    );

    test!(
        t_paragraph_meta,
        "par {
            tags { \"a\", \"b\" },
            props { (\"a\", 0), (\"b\", 2000/13/01), (\"d\", 2000/13/01) },
            tags { \"b\", \"c\" },
            props { (\"b\", 1), (\"c\", 2) },
            props { (\"d\", 2000/01/01) },
        }",
        Doc {
            items: vec![DocItem::Paragraph(Paragraph{
                tags: hset!(["a", "b", "c"]),
                props: props!([
                    ("a".to_string(), PropVal::Int(0)),
                    ("b".to_string(), PropVal::Int(1)),
                    ("c".to_string(), PropVal::Int(2)),
                    ("d".to_string(), PropVal::Date(Date::new(2000, 1, 1).unwrap()))
                ]),
                ..Default::default()
            })],
            ..Default::default()
        }
    );

    test!(
        t_paragraph_trailing_comma,
        "par { 'text', }",
        Doc {
            items: vec![
                DocItem::Paragraph(Paragraph {
                    items: vec![
                        ParagraphItem::Text("text".to_string()),
                    ],
                    ..Default::default()
                }),
            ],
            ..Default::default()
        }
    );

    test!(
        t_paragraph_comment,
        "/*c*/par/*c*/{/*c*/'/*c*/'/*c*/,/*c*/'//c'/*c*/,/*c*/}/*c*/",
        Doc {
            items: vec![
                DocItem::Paragraph(Paragraph {
                    items: vec![
                        ParagraphItem::Text("/*c*/".to_string()),
                        ParagraphItem::Text("//c".to_string()),
                    ],
                    ..Default::default()
                }),
            ],
            ..Default::default()
        }
    );

    test!(
        t_heading_c0,
        "head { 0, \"h\" }",
        Doc {
            items: vec![DocItem::Heading(Heading{
                level: 0,
                items: vec![HeadingItem::String("h".to_string())],
                ..Default::default()
            })],
            ..Default::default()
        }
    );

    test!(
        t_heading_c1,
        "head { 1, \"h\" }",
        Doc {
            items: vec![DocItem::Heading(Heading{
                level: 1,
                items: vec![HeadingItem::String("h".to_string())],
                ..Default::default()
            })],
            ..Default::default()
        }
    );

    test!(
        t_heading_c2,
        "head { 256, \"h\" }",
        Doc {
            items: vec![DocItem::Heading(Heading{
                level: 255,
                items: vec![HeadingItem::String("h".to_string())],
                ..Default::default()
            })],
            ..Default::default()
        }
    );

    test!(
        t_heading_c3,
        "head { 9999999999999999999, \"h\" }",
        Doc {
            items: vec![DocItem::Heading(Heading{
                level: 255,
                items: vec![HeadingItem::String("h".to_string())],
                ..Default::default()
            })],
            ..Default::default()
        }
    );

    test!(
        t_heading_c4,
        "head { 0, \"hello\" }",
        Doc {
            items: vec![DocItem::Heading(Heading{
                level: 0,
                items: vec![HeadingItem::String("hello".to_string())],
                ..Default::default()
            })],
            ..Default::default()
        }
    );

    test!(
        t_heading_c5,
        "head { 0, \"hello\", em { md, \" world\" } }",
        Doc {
            items: vec![DocItem::Heading(Heading{
                level: 0,
                items: vec![
                    HeadingItem::String("hello".to_string()),
                    HeadingItem::Em(Emphasis {
                        text: " world".to_string(),
                        etype: EmType::Deemphasis,
                        strength: EmStrength::Medium,
                        ..Default::default()
                    })
                ],
                ..Default::default()
            })],
            ..Default::default()
        }
    );

    test!(
        t_heading_meta0,
        "head { 1, tags { \"a\" }, tags{ \"b\" } }",
        Doc {
            items: vec![DocItem::Heading(Heading{
                level: 1,
                items: vec![],
                tags: hset!(["a", "b"]),
                ..Default::default()
            })],
            ..Default::default()
        }
    );

    test!(
        t_heading_meta1,
        "head { 1, props { (\"a\", 0), (\"b\", 0) }, props{ (\"b\", 1) } }",
        Doc {
            items: vec![DocItem::Heading(Heading{
                level: 1,
                items: vec![],
                props: props!([
                    ("a".to_string(), PropVal::Int(0)),
                    ("b".to_string(), PropVal::Int(1))
                ]),
                ..Default::default()
            })],
            ..Default::default()
        }
    );

    test!(
        t_heading_meta2,
        "head { 1, props { (\"a\", 0), (\"b\", 0) }, tags { \"tag\" }, props{ (\"b\", 1) } }",
        Doc {
            items: vec![DocItem::Heading(Heading{
                level: 1,
                items: vec![],
                tags: hset!(["tag"]),
                props: props!([
                    ("a".to_string(), PropVal::Int(0)),
                    ("b".to_string(), PropVal::Int(1))
                ]),
            })],
            ..Default::default()
        }
    );

    test!(
        t_heading_comment,
        "/*c*/head /*c*/ {/*c*/0/*c*/,/**/\"a\"/*c*/,/*c*/\"b\"/*c*/}//c",
        Doc {
            items: vec![DocItem::Heading(Heading{
                level: 0,
                items: vec![
                    HeadingItem::String("a".to_string()),
                    HeadingItem::String("b".to_string()),
                ],
                ..Default::default()
            })],
            ..Default::default()
        }
    );

    test!(
        t_list_c0,
        "list { dl, 'test' }",
        Doc {
            items: vec![
                DocItem::List(List{
                    items: vec![ParagraphItem::Text("test".to_string())],
                    ltype: ListType::Distinct,
                    ..Default::default()
                })
            ],
            ..Default::default()
        }
    );

    test!(
        t_list_c0_trailing_comma,
        "list { dl, 'test', }",
        Doc {
            items: vec![
                DocItem::List(List{
                    items: vec![ParagraphItem::Text("test".to_string())],
                    ltype: ListType::Distinct,
                    ..Default::default()
                })
            ],
            ..Default::default()
        }
    );

    test!(
        t_list_c1,
        "list { il, 'test' }",
        Doc {
            items: vec![
                DocItem::List(List{
                    items: vec![ParagraphItem::Text("test".to_string())],
                    ltype: ListType::Identical,
                    ..Default::default()
                })
            ],
            ..Default::default()
        }
    );

    test!(
        t_list_c2,
        "list { cl, 'test' }",
        Doc {
            items: vec![
                DocItem::List(List{
                    items: vec![ParagraphItem::Text("test".to_string())],
                    ltype: ListType::Checked,
                    ..Default::default()
                })
            ],
            ..Default::default()
        }
    );

    test!(
        t_list_c3,
        "list { cl, 'test' { tags { \"tag\" }, props { (\"prop\", 0) } } }",
        Doc {
            items: vec![
                DocItem::List(List{
                    items: vec![ParagraphItem::MText(TextWithMeta {
                        text: "test".to_string(),
                        tags: hset!(["tag"]),
                        props: props!([("prop".to_string(), PropVal::Int(0))]),
                    })],
                    ltype: ListType::Checked,
                    ..Default::default()
                })
            ],
            ..Default::default()
        }
    );

    test!(
        t_list_c4,
        "list { cl, em{ le, \"em\" } }",
        Doc {
            items: vec![
                DocItem::List(List{
                    items: vec![ParagraphItem::Em(Emphasis {
                        text: "em".to_string(),
                        etype: EmType::Emphasis,
                        strength: EmStrength::Light,
                        ..Default::default()
                    })],
                    ltype: ListType::Checked,
                    ..Default::default()
                })
            ],
            ..Default::default()
        }
    );

    test!(
        t_list_c5,
        "list { cl, code { \"rust\", \"show\", 'let rust = true;' } }",
        Doc {
            items: vec![
                DocItem::List(List{
                    items: vec![ParagraphItem::Code(Ok(CodeBlock {
                        language: "rust".to_string(),
                        mode: CodeModeHint::Show,
                        code: "let rust = true;".to_string(),
                        ..Default::default()
                    }))],
                    ltype: ListType::Checked,
                    ..Default::default()
                })
            ],
            ..Default::default()
        }
    );

    test!(
        t_list_c6,
        "list { il, '0', list { dl, '1', list { cl, '2' } } }",
        Doc {
            items: vec![
                DocItem::List(List {
                    ltype: ListType::Identical,
                    items: vec![
                        ParagraphItem::Text("0".to_string()),
                        ParagraphItem::List(List {
                            ltype: ListType::Distinct,
                            items: vec![
                                ParagraphItem::Text("1".to_string()),
                                ParagraphItem::List(List {
                                    ltype: ListType::Checked,
                                    items: vec![
                                        ParagraphItem::Text("2".to_string()),
                                    ],
                                    ..Default::default()
                                }),
                            ],
                            ..Default::default()
                        }),
                    ],
                    ..Default::default()
                })
            ],
            ..Default::default()
        }
    );

    test!(
        t_list_meta,
        "
        list {
            il,
            'item',
            props { (\"a\", 0), (\"b\", 0) },
            tags { \"a\" },
            props { (\"c\", 2), (\"b\", 1) },
            tags { \"b\" }
        }
        ",
        Doc {
            items: vec![DocItem::List(List {
                ltype: ListType::Identical,
                items: vec![ParagraphItem::Text("item".to_string())],
                tags: hset!(["a", "b"]),
                props: props!([
                    ("a".to_string(), PropVal::Int(0)),
                    ("b".to_string(), PropVal::Int(1)),
                    ("c".to_string(), PropVal::Int(2)),
                ]),
            })],
            ..Default::default()
        }
    );

    test!(
        t_list_comment,
        "/*c*/list/*c*/{/*c*/dl/*c*/,/*c*/'a'/*c*/,/*c*/'b'/*c*/,/*c*/}//c",
        Doc {
            items: vec![
                DocItem::List(List{
                    items: vec![
                        ParagraphItem::Text("a".to_string()),
                        ParagraphItem::Text("b".to_string()),
                    ],
                    ltype: ListType::Distinct,
                    ..Default::default()
                })
            ],
            ..Default::default()
        }
    );

    test!(
        t_section_c0_f0,
        "
        section {
            head { 0, \"heading\" },
            par { 'paragraph' }
        }
        ",
        Doc {
            items: vec![
                DocItem::Section(Section {
                    heading: Heading {
                        level: 0,
                        items: vec![
                            HeadingItem::String("heading".to_string()),
                        ],
                        ..Default::default()
                    },
                    items: vec![
                        SectionItem::Paragraph(Paragraph {
                            items: vec![
                                ParagraphItem::Text("paragraph".to_string()),
                            ],
                            ..Default::default()
                        })
                    ],
                    ..Default::default()
                }),
            ],
            ..Default::default()
        }
    );

    test!(
        t_section_c0_trailing_comma,
        "
        section {
            head { 0, \"heading\" },
            par { 'paragraph' },
        }
        ",
        Doc {
            items: vec![
                DocItem::Section(Section {
                    heading: Heading {
                        level: 0,
                        items: vec![
                            HeadingItem::String("heading".to_string()),
                        ],
                        ..Default::default()
                    },
                    items: vec![
                        SectionItem::Paragraph(Paragraph {
                            items: vec![
                                ParagraphItem::Text("paragraph".to_string()),
                            ],
                            ..Default::default()
                        })
                    ],
                    ..Default::default()
                }),
            ],
            ..Default::default()
        }
    );

    test!(
        t_section_c1,
        "
        section {
            head { 0, \"heading\" },
            section { head { 1, \"heading\" }, par { 'paragraph' } },
        }
        ",
        Doc {
            items: vec![
                DocItem::Section(Section {
                    heading: Heading {
                        level: 0,
                        items: vec![
                            HeadingItem::String("heading".to_string()),
                        ],
                        ..Default::default()
                    },
                    items: vec![
                        SectionItem::Section(Section {
                            heading: Heading {
                                level: 1,
                                items: vec![
                                    HeadingItem::String("heading".to_string()),
                                ],
                                ..Default::default()
                            },
                            items: vec![SectionItem::Paragraph(Paragraph {
                                items: vec![
                                    ParagraphItem::Text("paragraph".to_string()),
                                ],
                                ..Default::default()
                            })],
                            ..Default::default()
                        })
                    ],
                    ..Default::default()
                }),
            ],
            ..Default::default()
        }
    );

    test!(
        t_section_meta,
        "
        section {
            head { 0, \"heading\" },
            par { 'paragraph' },
            props { (\"a\", 0), (\"b\", 0) },
            tags { \"a\" },
            props { (\"c\", 2), (\"b\", 1) },
            tags { \"b\" }
        }
        ",
        Doc {
            items: vec![
                DocItem::Section(Section {
                    heading: Heading {
                        level: 0,
                        items: vec![
                            HeadingItem::String("heading".to_string()),
                        ],
                        ..Default::default()
                    },
                    items: vec![
                        SectionItem::Paragraph(Paragraph {
                            items: vec![
                                ParagraphItem::Text("paragraph".to_string()),
                            ],
                            ..Default::default()
                        })
                    ],
                    tags: hset!(["a", "b"]),
                    props: props!([
                        ("a".to_string(), PropVal::Int(0)),
                        ("b".to_string(), PropVal::Int(1)),
                        ("c".to_string(), PropVal::Int(2)),
                    ]),
                }),
            ],
            ..Default::default()
        }
    );

    test!(
        t_section_comment,
        "
        /*c*/section/*c*/{//c
            /*c*/head/*c*/{/*c*/0/*c*/,/*c*/\"h\"/*c*/,/*c*/}/*c*/,//c
            /*c*/par/*c*/{/*c*/'p'/*c*/}//c
        /*c*/}//c
        ",
        Doc {
            items: vec![
                DocItem::Section(Section {
                    heading: Heading {
                        level: 0,
                        items: vec![
                            HeadingItem::String("h".to_string()),
                        ],
                        ..Default::default()
                    },
                    items: vec![
                        SectionItem::Paragraph(Paragraph {
                            items: vec![
                                ParagraphItem::Text("p".to_string()),
                            ],
                            ..Default::default()
                        })
                    ],
                    ..Default::default()
                }),
            ],
            ..Default::default()
        }
    );

    test!(
        t_link_c0,
        "link { \"url\", \"link\" }",
        Doc {
            items: vec![
                DocItem::Link(Link {
                    url: "url".to_string(),
                    items: vec![LinkItem::String("link".to_string())],
                    ..Default::default()
                })
            ],
            ..Default::default()
        }
    );

    test!(
        t_link_c0_comment,
        "/*c*/link/*c*/{/*c*/\"url\"/*c*/,/*c*/\"link\"/*c*/}//c",
        Doc {
            items: vec![
                DocItem::Link(Link {
                    url: "url".to_string(),
                    items: vec![LinkItem::String("link".to_string())],
                    ..Default::default()
                })
            ],
            ..Default::default()
        }
    );

    test!(
        t_link_c1,
        "link { \"url\", em { le, \"em\" }, \"string\" }",
        Doc {
            items: vec![
                DocItem::Link(Link {
                    url: "url".to_string(),
                    items: vec![
                        LinkItem::Em(Emphasis {
                            text: "em".to_string(),
                            etype: EmType::Emphasis,
                            strength: EmStrength::Light,
                            ..Default::default()
                        }),
                        LinkItem::String("string".to_string()),
                    ],
                    ..Default::default()
                })
            ],
            ..Default::default()
        }
    );

    test!(
        t_link_meta0,
        "link { \"url\", \"link\", tags { \"tag\" } }",
        Doc {
            items: vec![
                DocItem::Link(Link {
                    url: "url".to_string(),
                    items: vec![LinkItem::String("link".to_string())],
                    tags: hset!(["tag"]),
                    ..Default::default()
                })
            ],
            ..Default::default()
        }
    );

    test!(
        t_link_meta1,
        "link { \"url\", \"link\", props { (\"prop\", 0) } }",
        Doc {
            items: vec![
                DocItem::Link(Link {
                    url: "url".to_string(),
                    items: vec![LinkItem::String("link".to_string())],
                    props: props!([
                        ("prop".to_string(), PropVal::Int(0)),
                    ]),
                    ..Default::default()
                })
            ],
            ..Default::default()
        }
    );

    test!(
        t_link_meta2,
        "link { \"url\", \"link\", tags { \"tag\" }, props { (\"prop\", 0) } }",
        Doc {
            items: vec![
                DocItem::Link(Link {
                    url: "url".to_string(),
                    items: vec![LinkItem::String("link".to_string())],
                    tags: hset!(["tag"]),
                    props: props!([
                        ("prop".to_string(), PropVal::Int(0)),
                    ]),
                })
            ],
            ..Default::default()
        }
    );

    test!(
        t_nav_c0,
        "
        nav {
            snav {
                \"desc\",
                link { \"urla\", \"linka\" },
                link { \"urlb\", \"linkb\" }
            }
        }
        ",
        Doc {
            items: vec![
                DocItem::Nav(vec![
                    SNav {
                        description: "desc".to_string(),
                        links: vec![
                            Link {
                                url: "urla".to_string(),
                                items: vec![
                                    LinkItem::String("linka".to_string())
                                ],
                                ..Default::default()
                            },
                            Link {
                                url: "urlb".to_string(),
                                items: vec![
                                    LinkItem::String("linkb".to_string())
                                ],
                                ..Default::default()
                            }
                        ],
                        ..Default::default()
                    }
                ])
            ],
            ..Default::default()
        }
    );

    test!(
        t_nav_c0_comment,
        "
        /*c*/nav/*c*/{//c
            /*c*/snav/*c*/{//c
                /*c*/\"desc\"/*c*/,
                /*c*/link/*c*/{/*c*/\"urla\"/*c*/,/*c*/\"linka\"/*c*/}/*c*/,//c
                /*c*/link/*c*/{/*c*/\"urlb\"/*c*/,/*c*/\"linkb\"/*c*/}/*c*/,//c
            /*c*/}/*c*/
        /*c*/}//c
        ",
        Doc {
            items: vec![
                DocItem::Nav(vec![
                    SNav {
                        description: "desc".to_string(),
                        links: vec![
                            Link {
                                url: "urla".to_string(),
                                items: vec![
                                    LinkItem::String("linka".to_string())
                                ],
                                ..Default::default()
                            },
                            Link {
                                url: "urlb".to_string(),
                                items: vec![
                                    LinkItem::String("linkb".to_string())
                                ],
                                ..Default::default()
                            }
                        ],
                        ..Default::default()
                    }
                ])
            ],
            ..Default::default()
        }
    );

    test!(
        t_nav_c1,
        "
        nav {
            snav {
                \"desca\",
                link { \"urla\", \"linka\" }
            },
            snav {
                \"descb\",
                link { \"urlb\", \"linkb\" }
            }
        }
        ",
        Doc {
            items: vec![
                DocItem::Nav(vec![
                    SNav {
                        description: "desca".to_string(),
                        links: vec![
                            Link {
                                url: "urla".to_string(),
                                items: vec![
                                    LinkItem::String("linka".to_string())
                                ],
                                ..Default::default()
                            },
                        ],
                        ..Default::default()
                    },
                    SNav {
                        description: "descb".to_string(),
                        links: vec![
                            Link {
                                url: "urlb".to_string(),
                                items: vec![
                                    LinkItem::String("linkb".to_string())
                                ],
                                ..Default::default()
                            },
                        ],
                        ..Default::default()
                    }
                ])
            ],
            ..Default::default()
        }
    );

    test!(
        t_nav_c1_trailing_comma,
        "
        nav {
            snav {
                \"desca\",
                link { \"urla\", \"linka\" },
            },
            snav {
                \"descb\",
                link { \"urlb\", \"linkb\" },
            },
        }
        ",
        Doc {
            items: vec![
                DocItem::Nav(vec![
                    SNav {
                        description: "desca".to_string(),
                        links: vec![
                            Link {
                                url: "urla".to_string(),
                                items: vec![
                                    LinkItem::String("linka".to_string())
                                ],
                                ..Default::default()
                            },
                        ],
                        ..Default::default()
                    },
                    SNav {
                        description: "descb".to_string(),
                        links: vec![
                            Link {
                                url: "urlb".to_string(),
                                items: vec![
                                    LinkItem::String("linkb".to_string())
                                ],
                                ..Default::default()
                            },
                        ],
                        ..Default::default()
                    }
                ])
            ],
            ..Default::default()
        }
    );

    test!(
        t_nav_c2,
        "
        nav {
            snav {
                \"descx\",
                snav {
                    \"desca\",
                    link { \"urla\", \"linka\" }
                },
                snav {
                    \"descb\",
                    link { \"urlb\", \"linkb\" }
                },
                link { \"urlc\", \"linkc\" }
            },
        }
        ",
        Doc {
            items: vec![
                DocItem::Nav(vec![
                    SNav {
                        description: "descx".to_string(),
                        subs: vec![
                            SNav {
                                description: "desca".to_string(),
                                links: vec![
                                    Link {
                                        url: "urla".to_string(),
                                        items: vec![
                                            LinkItem::String("linka".to_string())
                                        ],
                                        ..Default::default()
                                    },
                                ],
                                ..Default::default()
                            },
                            SNav {
                                description: "descb".to_string(),
                                links: vec![
                                    Link {
                                        url: "urlb".to_string(),
                                        items: vec![
                                            LinkItem::String("linkb".to_string())
                                        ],
                                        ..Default::default()
                                    },
                                ],
                                ..Default::default()
                            },
                        ],
                        links: vec![
                            Link {
                                url: "urlc".to_string(),
                                items: vec![
                                    LinkItem::String("linkc".to_string())
                                ],
                                ..Default::default()
                            },
                        ],
                        ..Default::default()
                    },
                ])
            ],
            ..Default::default()
        }
    );

    test!(
        t_nav_c3,
        "
        nav {
            snav {
                \"desc\",
                link { \"urla\", \"linka\" },
                tags { \"tag\" }
            }
        }
        ",
        Doc {
            items: vec![
                DocItem::Nav(vec![
                    SNav {
                        description: "desc".to_string(),
                        links: vec![
                            Link {
                                url: "urla".to_string(),
                                items: vec![
                                    LinkItem::String("linka".to_string())
                                ],
                                ..Default::default()
                            },
                        ],
                        tags: hset!(["tag"]),
                        ..Default::default()
                    }
                ])
            ],
            ..Default::default()
        }
    );

    test!(
        t_nav_meta0,
        "
        nav {
            snav {
                \"desc\",
                link { \"urla\", \"linka\" },
                props { (\"prop\", 0) }
            }
        }
        ",
        Doc {
            items: vec![
                DocItem::Nav(vec![
                    SNav {
                        description: "desc".to_string(),
                        links: vec![
                            Link {
                                url: "urla".to_string(),
                                items: vec![
                                    LinkItem::String("linka".to_string())
                                ],
                                ..Default::default()
                            },
                        ],
                        props: props!([
                            ("prop".to_string(), PropVal::Int(0)),
                        ]),
                        ..Default::default()
                    }
                ])
            ],
            ..Default::default()
        }
    );

    test!(
        t_nav_meta1,
        "
        nav {
            snav {
                \"desc\",
                link { \"urla\", \"linka\" },
                tags { \"tag\" },
                props { (\"prop\", 0) }
            }
        }
        ",
        Doc {
            items: vec![
                DocItem::Nav(vec![
                    SNav {
                        description: "desc".to_string(),
                        links: vec![
                            Link {
                                url: "urla".to_string(),
                                items: vec![
                                    LinkItem::String("linka".to_string())
                                ],
                                ..Default::default()
                            },
                        ],
                        tags: hset!(["tag"]),
                        props: props!([
                            ("prop".to_string(), PropVal::Int(0)),
                        ]),
                        ..Default::default()
                    }
                ])
            ],
            ..Default::default()
        }
    );
}
