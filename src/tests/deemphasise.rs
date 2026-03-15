#[cfg(test)]
mod deemphasise {
    use crate::*;
    use crate::actions::deemphasise::DeEmphasise;

    macro_rules! test_deemphasise {
        ($name:ident, $input:expr, $output:expr) => {
            #[test]
            fn $name() {
                assert_eq!(&$input.deemphasise(), $output);
            }
        }
    }

    test_deemphasise!(
        emphasis,
        Emphasis {
            text: "with emphasis".to_string(),
            ..Default::default()
        },
        "with emphasis"
    );

    test_deemphasise!(
        emphasised_text,
        vec![
            EmOrText::Text("This is a ".to_string()),
            EmOrText::Em(Emphasis {
                text: "sentence".to_string(),
                ..Default::default()
            }),
            EmOrText::Text(" with ".to_string()),
            EmOrText::Em(Emphasis {
                text: "emphasis".to_string(),
                ..Default::default()
            }),
            EmOrText::Text(".".to_string()),
        ],
        "This is a sentence with emphasis."
    );
}
