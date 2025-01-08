macro_rules! test {
    ($name:ident, $string:expr, $result:expr) => {
        #[test]
        fn $name() {
            assert_eq!(parse($string), Ok($result));
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    test!(
        t_empty_f0,
        "",
        Doc::default()
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
            meta: vec![("prop".to_string(), MetaVal::String("test".to_string()))],
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
            errors: vec![MetaValError::String(StringLBError)],
            ..Default::default()
        }
    );

    test!(
        t_meta_tuple_int_c0,
        "meta { (\"prop\", 5) }",
        Doc {
            meta: vec![("prop".to_string(), MetaVal::Int(5))],
            ..Default::default()
        }
    );

    test!(
        t_meta_tuple_int_c1,
        "meta { (\"prop\", +7) }",
        Doc {
            meta: vec![("prop".to_string(), MetaVal::Int(7))],
            ..Default::default()
        }
    );

    test!(
        t_meta_tuple_int_c2,
        "meta { (\"prop\", -1) }",
        Doc {
            meta: vec![("prop".to_string(), MetaVal::Int(-1))],
            ..Default::default()
        }
    );

    test!(
        t_meta_tuple_date_c0,
        "meta { (\"prop\", 2000/01/01) }",
        Doc {
            meta: vec![("prop".to_string(), MetaVal::Date(Date::new(2000, 1, 1).unwrap()))],
            ..Default::default()
        }
    );

    test!(
        t_meta_tuple_date_c1,
        "meta { (\"prop\", -3434/01/01) }",
        Doc {
            meta: vec![("prop".to_string(), MetaVal::Date(Date::new(-3434, 1, 1).unwrap()))],
            ..Default::default()
        }
    );

    test!(
        t_meta_tuple_date_c2,
        "meta { (\"prop\", 2000/13/01) }",
        Doc {
            errors: vec![MetaValError::Date(DateError::MonthRange(13))],
            ..Default::default()
        }
    );

    test!(
        t_meta_tuple_date_c3,
        "meta { (\"prop\", 2000/01/32) }",
        Doc {
            errors: vec![MetaValError::Date(DateError::DayRange(32))],
            ..Default::default()
        }
    );
}
