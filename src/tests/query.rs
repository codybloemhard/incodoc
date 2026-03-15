#[cfg(test)]
mod query {
    use crate::*;
    use crate::actions::deemphasise::DeEmphasise;

    #[test]
    fn first_heading() {
        let doc = Doc {
            items: vec![
                DocItem::Section(Section {
                    heading: Heading {
                        level: 3,
                        items: vec![EmOrText::Text("first heading".to_string())],
                        ..Default::default()
                    },
                    ..Default::default()
                }),
                DocItem::Section(Section {
                    heading: Heading {
                        level: 0,
                        items: vec![EmOrText::Text("first h1".to_string())],
                        ..Default::default()
                    },
                    ..Default::default()
                }),
                DocItem::Section(Section {
                    heading: Heading {
                        level: 0,
                        items: vec![EmOrText::Text("second h1".to_string())],
                        ..Default::default()
                    },
                    ..Default::default()
                }),
            ],
            ..Default::default()
        };

        assert_eq!(doc.first_heading().unwrap().items.deemphasise(), "first heading");
        assert_eq!(doc.first_biggest_heading().unwrap().items.deemphasise(), "first h1");
    }
}
