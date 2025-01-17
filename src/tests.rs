#[cfg(test)]
mod tests {
    use crate::*;

    macro_rules! meta {
        ($slice:expr) => {
            Meta::from(HashMap::from($slice), vec![])
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
        t_meta_empty_f0,
        "meta{}",
        Doc::default()
    );

    test!(
        t_meta_empty_f1,
        "meta {}",
        Doc::default()
    );

    test!(
        t_meta_empty_f2,
        "meta {  }, ",
        Doc::default()
    );

    test!(
        t_meta_empty_f3,
        "
        meta {

        },
        ",
        Doc::default()
    );

    test!(
        t_meta_tuple_string_f0,
        "meta{(\"prop\",\"test\")}",
        Doc {
            meta: meta!([("prop".to_string(), MetaVal::String("test".to_string()))]),
            ..Default::default()
        }
    );

    test!(
        t_meta_tuple_string_f1,
"
meta { (
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
            meta: meta!([(
                "pr        op".to_string(),
                MetaVal::String("te     st     ".to_string())
            )]),
            ..Default::default()
        }
    );

    test!(
        t_meta_tuple_int_c0,
        "meta { (\"prop\", 5) }",
        Doc {
            meta: meta!([("prop".to_string(), MetaVal::Int(5))]),
            ..Default::default()
        }
    );

    test!(
        t_meta_tuple_int_c1,
        "meta { (\"prop\", +7) }",
        Doc {
            meta: meta!([("prop".to_string(), MetaVal::Int(7))]),
            ..Default::default()
        }
    );

    test!(
        t_meta_tuple_int_c2,
        "meta { (\"prop\", -1) }",
        Doc {
            meta: meta!([("prop".to_string(), MetaVal::Int(-1))]),
            ..Default::default()
        }
    );

    test!(
        t_meta_tuple_date_c0,
        "meta { (\"prop\", 2000/01/01) }",
        Doc {
            meta: meta!([("prop".to_string(), MetaVal::Date(Date::new(2000, 1, 1).unwrap()))]),
            ..Default::default()
        }
    );

    test!(
        t_meta_tuple_date_c1,
        "meta { (\"prop\", -3434/01/01) }",
        Doc {
            meta: meta!([("prop".to_string(), MetaVal::Date(Date::new(-3434, 1, 1).unwrap()))]),
            ..Default::default()
        }
    );

    test!(
        t_meta_tuple_date_c2,
        "meta { (\"prop\", 2000/13/01) }",
        Doc {
            meta: Meta::from(
                HashMap::default(),
                vec![MetaValError::Date(DateError::MonthRange(13))]
            ),
            ..Default::default()
        }
    );

    test!(
        t_meta_tuple_date_c3,
        "meta { (\"prop\", 2000/01/32) }",
        Doc {
            meta: Meta::from(HashMap::default(), vec![MetaValError::Date(DateError::DayRange(32))]),
            ..Default::default()
        }
    );

    test!(
        t_meta_tuple_text_c0,
        "meta { (\"prop\", 'this is text') }",
        Doc {
            meta: meta!([("prop".to_string(), MetaVal::Text("this is text".to_string()))]),
            ..Default::default()
        }
    );

    test!(
        t_meta_tuple_text_c1,
        // looks incorrect because of the escape characters for "
        "
        meta { (\"prop\", '
                        this is text
        ') }",
        Doc {
            meta: meta!([("prop".to_string(), MetaVal::Text("this is text".to_string()))]),
            ..Default::default()
        }
    );

    test!(
        t_meta_absorb_c0,
        "meta { (\"a\", 0), (\"b\", 0) }, meta { (\"c\", 1) }",
        Doc {
            meta: meta!([
                ("a".to_string(), MetaVal::Int(0)),
                ("b".to_string(), MetaVal::Int(0)),
                ("c".to_string(), MetaVal::Int(1)),
            ]),
            ..Default::default()
        }
    );

    test!(
        t_meta_absorb_c1,
        "meta { (\"a\", 0), (\"b\", 0) }, meta { (\"b\", 1) }",
        Doc {
            meta: meta!([
                ("a".to_string(), MetaVal::Int(0)),
                ("b".to_string(), MetaVal::Int(1)),
            ]),
            ..Default::default()
        }
    );

    test!(
        t_meta_absorb_c2,
        "meta { (\"a\", 0), (\"b\", 2000/13/01) }, meta { (\"b\", 1) }",
        Doc {
            meta: Meta::from(
                HashMap::from([
                    ("a".to_string(), MetaVal::Int(0)),
                    ("b".to_string(), MetaVal::Int(1)),
                ]),
                vec![MetaValError::Date(DateError::MonthRange(13))]
            ),
            ..Default::default()
        }
    );

    test!(
        t_meta_absorb_c3,
        "
        meta { (\"a\", 0), (\"b\", 2000/13/01), (\"a\", 1) },
        meta { (\"b\", 2000/13/01), (\"b\", 1), (\"a\", 2) }
        ",
        Doc {
            meta: Meta::from(
                HashMap::from([
                    ("a".to_string(), MetaVal::Int(2)),
                    ("b".to_string(), MetaVal::Int(1)),
                ]),
                vec![
                    MetaValError::Date(DateError::MonthRange(13)),
                    MetaValError::Date(DateError::MonthRange(13))
                ]
            ),
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
            meta { (\"test tuple\", 'yay') }
        }",
        Doc {
            items: vec![DocItem::Code(CodeBlock{
                language: "plain".to_string(),
                code: "this is code".to_string(),
                meta: meta!([("test tuple".to_string(), MetaVal::Text("yay".to_string()))]),
                ..Default::default()
            })],
            ..Default::default()
        }
    );
}
