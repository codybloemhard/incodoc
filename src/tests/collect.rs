#[cfg(test)]
mod toc {
    use crate::*;
    use crate::actions::deemphasise::DeEmphasise;

    #[test]
    fn collect_links() {
        let ml = |t: &str| {
            Link{ items: vec![EmOrText::Text(t.to_string())], ..Default::default() }
        };
        let mut doc = Doc {
            navs: vec![
                Nav {
                    links: vec![ml("nav0")],
                    subs: vec![
                        Nav {
                            links: vec![ml("nav1")],
                            ..Default::default()
                        },
                    ],
                    ..Default::default()
                },
            ],
            items: vec![
                DocItem::Section(Section {
                    items: vec![SectionItem::Paragraph(Paragraph {
                        items: vec![
                            ParagraphItem::Link(ml("par")),
                            ParagraphItem::List(List {
                                items: vec![Paragraph {
                                    items: vec![ParagraphItem::Link(ml("list"))],
                                    ..Default::default()
                                }],
                                ..Default::default()
                            }),
                            ParagraphItem::Table(Table {
                                rows: vec![TableRow {
                                    items: vec![Paragraph {
                                        items: vec![ParagraphItem::Link(ml("table"))],
                                        ..Default::default()
                                    }],
                                    ..Default::default()
                                }],
                                ..Default::default()
                            }),
                        ],
                        ..Default::default()
                    })],
                    ..Default::default()
                }),
            ],
            ..Default::default()
        };
        let mut iter = doc.links_mut(true).into_iter();
        assert_eq!(&iter.next().unwrap().items.deemphasise(), "nav0");
        assert_eq!(&iter.next().unwrap().items.deemphasise(), "nav1");
        assert_eq!(&iter.next().unwrap().items.deemphasise(), "par");
        assert_eq!(&iter.next().unwrap().items.deemphasise(), "list");
        assert_eq!(&iter.next().unwrap().items.deemphasise(), "table");
    }
}
