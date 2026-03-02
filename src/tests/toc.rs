#[cfg(test)]
mod toc {
    use crate::*;
    use crate::actions::toc::*;

    macro_rules! test_toc {
        ($name:ident, $input:expr, $filter:expr, $output:expr) => {
            #[test]
            fn $name() {
                let toc = $input.get_table_of_contents(&$filter);
                assert_eq!(toc, $output);
            }
        }
    }

    fn toc_doc_0() -> Doc {
        Doc {
            navs: vec![
                Nav {
                    description: "nav".to_string(),
                    ..Default::default()
                },
            ],
            items: vec![
                DocItem::Paragraph(Paragraph {
                    items: vec![ParagraphItem::Text("this is some text".to_string())],
                    ..Default::default()
                }),
                DocItem::Section(Section {
                    heading: Heading {
                        level: 0,
                        items: vec![HeadingItem::String("A H1 heading".to_string())],
                        ..Default::default()
                    },
                    items: vec![
                        SectionItem::Section(Section {
                            heading: Heading {
                                level: 0,
                                items: vec![HeadingItem::String("H2 heading".to_string())],
                                ..Default::default()
                            },
                            items: vec![
                                SectionItem::Section(Section {
                                    heading: Heading {
                                        level: 0,
                                        items: vec![HeadingItem::String("H3".to_string())],
                                        ..Default::default()
                                    },
                                    items: vec![],
                                    ..Default::default()
                                }),
                            ],
                            props: props!([
                                (
                                    "id".to_string(),
                                    PropVal::String("h2-id".to_string())
                                ),
                            ]),
                            ..Default::default()
                        }),
                        SectionItem::Section(Section {
                            heading: Heading {
                                level: 0,
                                items: vec![HeadingItem::String("Another H2".to_string())],
                                ..Default::default()
                            },
                            items: vec![
                                SectionItem::Paragraph(Paragraph {
                                    items: vec![
                                        ParagraphItem::MText(TextWithMeta {
                                            text: "mtext".to_string(),
                                            props: props!([
                                                (
                                                    "id".to_string(),
                                                    PropVal::String("mtext-id".to_string())
                                                ),
                                            ]),
                                            ..Default::default()
                                        }),
                                        ParagraphItem::Em(Emphasis {
                                            strength: EmStrength::Light,
                                            etype: EmType::Emphasis,
                                            text: "emphasis".to_string(),
                                            props: props!([
                                                (
                                                    "id".to_string(),
                                                    PropVal::String("emphasis-id".to_string())
                                                ),
                                            ]),
                                            ..Default::default()
                                        }),
                                        ParagraphItem::Code(Ok(CodeBlock {
                                            language: "rust".to_string(),
                                            mode: CodeModeHint::Show,
                                            code: "let x = 0;".to_string(),
                                            props: props!([
                                                (
                                                    "id".to_string(),
                                                    PropVal::String("codeblock-id".to_string())
                                                ),
                                            ]),
                                            ..Default::default()
                                        })),
                                        ParagraphItem::Link(Link {
                                            url: "url".to_string(),
                                            items: vec![LinkItem::String("link".to_string())],
                                            props: props!([
                                                (
                                                    "id".to_string(),
                                                    PropVal::String("link-id".to_string())
                                                ),
                                            ]),
                                            ..Default::default()
                                        }),
                                        ParagraphItem::List(List {
                                            ltype: ListType::Identical,
                                            items: vec![Paragraph {
                                                items: vec![
                                                    ParagraphItem::MText(TextWithMeta {
                                                        text: "list-mtext".to_string(),
                                                        props: props!([
                                                            (
                                                                "id".to_string(),
                                                                PropVal::String(
                                                                    "list-mtext-id".to_string()
                                                                )
                                                            ),
                                                        ]),
                                                        ..Default::default()
                                                    }),
                                                ],
                                                ..Default::default()
                                            }],
                                            props: props!([
                                                (
                                                    "id".to_string(),
                                                    PropVal::String("list-id".to_string())
                                                ),
                                            ]),
                                            ..Default::default()
                                        }),
                                        ParagraphItem::Table(Table {
                                            rows: vec![TableRow {
                                                is_header: false,
                                                items: vec![Paragraph {
                                                    items: vec![
                                                        ParagraphItem::MText(TextWithMeta {
                                                            text: "table-mtext".to_string(),
                                                            props: props!([
                                                                (
                                                                    "id".to_string(),
                                                                    PropVal::String(
                                                                        "table-mtext-id".to_string()
                                                                    )
                                                                ),
                                                            ]),
                                                            ..Default::default()
                                                        }),
                                                    ],
                                                    ..Default::default()
                                                }],
                                                ..Default::default()
                                            }],
                                            props: props!([
                                                (
                                                    "id".to_string(),
                                                    PropVal::String("table-id".to_string())
                                                ),
                                            ]),
                                            ..Default::default()
                                        }),
                                    ],
                                    ..Default::default()
                                }),
                            ],
                            ..Default::default()
                        }),
                    ],
                    ..Default::default()
                }),
            ],
            ..Default::default()
        }
    }

    fn toc_doc_1() -> Doc {
        Doc {
            items: vec![
                DocItem::Paragraph(Paragraph {
                    items: vec![
                        ParagraphItem::List(List {
                            ltype: ListType::Identical,
                            items: vec![Paragraph {
                                items: vec![
                                    ParagraphItem::MText(TextWithMeta {
                                        text: "list-mtext".to_string(),
                                        props: props!([
                                            (
                                                "id".to_string(),
                                                PropVal::String(
                                                    "list-mtext-id".to_string()
                                                )
                                            ),
                                        ]),
                                        ..Default::default()
                                    }),
                                ],
                                ..Default::default()
                            }],
                            ..Default::default()
                        }),
                        ParagraphItem::Table(Table {
                            rows: vec![TableRow {
                                is_header: false,
                                items: vec![Paragraph {
                                    items: vec![
                                        ParagraphItem::MText(TextWithMeta {
                                            text: "table-mtext".to_string(),
                                            props: props!([
                                                (
                                                    "id".to_string(),
                                                    PropVal::String(
                                                        "table-mtext-id".to_string()
                                                    )
                                                ),
                                            ]),
                                            ..Default::default()
                                        }),
                                    ],
                                    ..Default::default()
                                }],
                                ..Default::default()
                            }],
                            ..Default::default()
                        }),
                    ],
                    ..Default::default()
                }),
            ],
            ..Default::default()
        }
    }

    test_toc!(
        toc_hs_doc,
        toc_doc_0(),
        Some((
            HashSet::from([
                TableOfContentsItemType::Document,
            ]),
            TableOfContentsFilterType::HardStop
        )),
        Some(TableOfContentsItem {
            title: "Table of Contents".to_string(),
            link: ".".to_string(),
            item_type: TableOfContentsItemType::Document,
            children: vec![],
        })
    );

    test_toc!(
        toc_hs_doc_section,
        toc_doc_0(),
        Some((
            HashSet::from([
                TableOfContentsItemType::Document,
                TableOfContentsItemType::Section,
            ]),
            TableOfContentsFilterType::HardStop
        )),
        Some(TableOfContentsItem {
            title: "Table of Contents".to_string(),
            link: ".".to_string(),
            item_type: TableOfContentsItemType::Document,
            children: vec![
                TableOfContentsItem {
                    title: "A H1 heading".to_string(),
                    link: "#a-h1-heading".to_string(),
                    item_type: TableOfContentsItemType::Section,
                    children: vec![
                        TableOfContentsItem {
                            title: "H2 heading".to_string(),
                            link: "#h2-id".to_string(),
                            item_type: TableOfContentsItemType::Section,
                            children: vec![
                                TableOfContentsItem {
                                    title: "H3".to_string(),
                                    link: "#h3".to_string(),
                                    item_type: TableOfContentsItemType::Section,
                                    children: vec![
                                    ],
                                },
                            ],
                        },
                        TableOfContentsItem {
                            title: "Another H2".to_string(),
                            link: "#another-h2".to_string(),
                            item_type: TableOfContentsItemType::Section,
                            children: vec![
                            ],
                        },
                    ],
                },
            ],
        })
    );

    // MText leaf cannot be seen through paragraph
    test_toc!(
        toc_hs_doc_section_mtext,
        toc_doc_0(),
        Some((
            HashSet::from([
                TableOfContentsItemType::Document,
                TableOfContentsItemType::Section,
                TableOfContentsItemType::MText,
            ]),
            TableOfContentsFilterType::HardStop
        )),
        Some(TableOfContentsItem {
            title: "Table of Contents".to_string(),
            link: ".".to_string(),
            item_type: TableOfContentsItemType::Document,
            children: vec![
                TableOfContentsItem {
                    title: "A H1 heading".to_string(),
                    link: "#a-h1-heading".to_string(),
                    item_type: TableOfContentsItemType::Section,
                    children: vec![
                        TableOfContentsItem {
                            title: "H2 heading".to_string(),
                            link: "#h2-id".to_string(),
                            item_type: TableOfContentsItemType::Section,
                            children: vec![
                                TableOfContentsItem {
                                    title: "H3".to_string(),
                                    link: "#h3".to_string(),
                                    item_type: TableOfContentsItemType::Section,
                                    children: vec![
                                    ],
                                },
                            ],
                        },
                        TableOfContentsItem {
                            title: "Another H2".to_string(),
                            link: "#another-h2".to_string(),
                            item_type: TableOfContentsItemType::Section,
                            children: vec![
                            ],
                        },
                    ],
                },
            ],
        })
    );

    test_toc!(
        toc_hs_doc_section_par_mtext,
        toc_doc_0(),
        Some((
            HashSet::from([
                TableOfContentsItemType::Document,
                TableOfContentsItemType::Section,
                TableOfContentsItemType::Paragraph,
                TableOfContentsItemType::MText,
            ]),
            TableOfContentsFilterType::HardStop
        )),
        Some(TableOfContentsItem {
            title: "Table of Contents".to_string(),
            link: ".".to_string(),
            item_type: TableOfContentsItemType::Document,
            children: vec![
                TableOfContentsItem {
                    title: "A H1 heading".to_string(),
                    link: "#a-h1-heading".to_string(),
                    item_type: TableOfContentsItemType::Section,
                    children: vec![
                        TableOfContentsItem {
                            title: "H2 heading".to_string(),
                            link: "#h2-id".to_string(),
                            item_type: TableOfContentsItemType::Section,
                            children: vec![
                                TableOfContentsItem {
                                    title: "H3".to_string(),
                                    link: "#h3".to_string(),
                                    item_type: TableOfContentsItemType::Section,
                                    children: vec![
                                    ],
                                },
                            ],
                        },
                        TableOfContentsItem {
                            title: "Another H2".to_string(),
                            link: "#another-h2".to_string(),
                            item_type: TableOfContentsItemType::Section,
                            children: vec![
                                TableOfContentsItem {
                                    title: "paragraph".to_string(),
                                    link: "".to_string(),
                                    item_type: TableOfContentsItemType::Paragraph,
                                    children: vec![
                                        TableOfContentsItem {
                                            title: "mtext-id".to_string(),
                                            link: "#mtext-id".to_string(),
                                            item_type: TableOfContentsItemType::MText,
                                            children: vec![
                                            ],
                                        },
                                    ],
                                },
                            ],
                        },
                    ],
                },
            ],
        })
    );

    test_toc!(
        toc_hs_doc_section_par_all_but_mtext,
        toc_doc_0(),
        Some((
            HashSet::from([
                TableOfContentsItemType::Document,
                TableOfContentsItemType::Section,
                TableOfContentsItemType::Paragraph,
                TableOfContentsItemType::Emphasis,
                TableOfContentsItemType::CodeBlock,
                TableOfContentsItemType::Link,
                TableOfContentsItemType::List,
                TableOfContentsItemType::Table,
            ]),
            TableOfContentsFilterType::HardStop
        )),
        Some(TableOfContentsItem {
            title: "Table of Contents".to_string(),
            link: ".".to_string(),
            item_type: TableOfContentsItemType::Document,
            children: vec![
                TableOfContentsItem {
                    title: "A H1 heading".to_string(),
                    link: "#a-h1-heading".to_string(),
                    item_type: TableOfContentsItemType::Section,
                    children: vec![
                        TableOfContentsItem {
                            title: "H2 heading".to_string(),
                            link: "#h2-id".to_string(),
                            item_type: TableOfContentsItemType::Section,
                            children: vec![
                                TableOfContentsItem {
                                    title: "H3".to_string(),
                                    link: "#h3".to_string(),
                                    item_type: TableOfContentsItemType::Section,
                                    children: vec![
                                    ],
                                },
                            ],
                        },
                        TableOfContentsItem {
                            title: "Another H2".to_string(),
                            link: "#another-h2".to_string(),
                            item_type: TableOfContentsItemType::Section,
                            children: vec![
                                TableOfContentsItem {
                                    title: "paragraph".to_string(),
                                    link: "".to_string(),
                                    item_type: TableOfContentsItemType::Paragraph,
                                    children: vec![
                                        TableOfContentsItem {
                                            title: "emphasis".to_string(),
                                            link: "#emphasis-id".to_string(),
                                            item_type: TableOfContentsItemType::Emphasis,
                                            children: vec![],
                                        },
                                        TableOfContentsItem {
                                            title: "codeblock-id".to_string(),
                                            link: "#codeblock-id".to_string(),
                                            item_type: TableOfContentsItemType::CodeBlock,
                                            children: vec![],
                                        },
                                        TableOfContentsItem {
                                            title: "link".to_string(),
                                            link: "#link-id".to_string(),
                                            item_type: TableOfContentsItemType::Link,
                                            children: vec![],
                                        },
                                        TableOfContentsItem {
                                            title: "list-id".to_string(),
                                            link: "#list-id".to_string(),
                                            item_type: TableOfContentsItemType::List,
                                            children: vec![],
                                        },
                                        TableOfContentsItem {
                                            title: "table-id".to_string(),
                                            link: "#table-id".to_string(),
                                            item_type: TableOfContentsItemType::Table,
                                            children: vec![],
                                        },
                                    ],
                                },
                            ],
                        },
                    ],
                },
            ],
        })
    );

    test_toc!(
        toc_iwc_doc,
        toc_doc_0(),
        Some((
            HashSet::from([
                TableOfContentsItemType::Document,
            ]),
            TableOfContentsFilterType::IncludeWithChildren
        )),
        Some(TableOfContentsItem {
            title: "Table of Contents".to_string(),
            link: ".".to_string(),
            item_type: TableOfContentsItemType::Document,
            children: vec![],
        })
    );

    test_toc!(
        toc_iwc_doc_section,
        toc_doc_0(),
        Some((
            HashSet::from([
                TableOfContentsItemType::Document,
                TableOfContentsItemType::Section,
            ]),
            TableOfContentsFilterType::IncludeWithChildren
        )),
        Some(TableOfContentsItem {
            title: "Table of Contents".to_string(),
            link: ".".to_string(),
            item_type: TableOfContentsItemType::Document,
            children: vec![
                TableOfContentsItem {
                    title: "A H1 heading".to_string(),
                    link: "#a-h1-heading".to_string(),
                    item_type: TableOfContentsItemType::Section,
                    children: vec![
                        TableOfContentsItem {
                            title: "H2 heading".to_string(),
                            link: "#h2-id".to_string(),
                            item_type: TableOfContentsItemType::Section,
                            children: vec![
                                TableOfContentsItem {
                                    title: "H3".to_string(),
                                    link: "#h3".to_string(),
                                    item_type: TableOfContentsItemType::Section,
                                    children: vec![
                                    ],
                                },
                            ],
                        },
                        TableOfContentsItem {
                            title: "Another H2".to_string(),
                            link: "#another-h2".to_string(),
                            item_type: TableOfContentsItemType::Section,
                            children: vec![
                            ],
                        },
                    ],
                },
            ],
        })
    );

    // MText leaf can be seen through paragraph
    test_toc!(
        toc_iwc_doc_section_mtext,
        toc_doc_0(),
        Some((
            HashSet::from([
                TableOfContentsItemType::Document,
                TableOfContentsItemType::MText,
            ]),
            TableOfContentsFilterType::IncludeWithChildren
        )),
        Some(TableOfContentsItem {
            title: "Table of Contents".to_string(),
            link: ".".to_string(),
            item_type: TableOfContentsItemType::Document,
            children: vec![
                TableOfContentsItem {
                    title: "A H1 heading".to_string(),
                    link: "#a-h1-heading".to_string(),
                    item_type: TableOfContentsItemType::Section,
                    children: vec![
                        TableOfContentsItem {
                            title: "Another H2".to_string(),
                            link: "#another-h2".to_string(),
                            item_type: TableOfContentsItemType::Section,
                            children: vec![
                                TableOfContentsItem {
                                    title: "paragraph".to_string(),
                                    link: "".to_string(),
                                    item_type: TableOfContentsItemType::Paragraph,
                                    children: vec![
                                        TableOfContentsItem {
                                            title: "mtext-id".to_string(),
                                            link: "#mtext-id".to_string(),
                                            item_type: TableOfContentsItemType::MText,
                                            children: vec![
                                            ],
                                        },
                                        TableOfContentsItem {
                                            title: "list-id".to_string(),
                                            link: "#list-id".to_string(),
                                            item_type: TableOfContentsItemType::List,
                                            children: vec![
                                                TableOfContentsItem {
                                                    title: "paragraph".to_string(),
                                                    link: "".to_string(),
                                                    item_type: TableOfContentsItemType::Paragraph,
                                                    children: vec![
                                                        TableOfContentsItem {
                                                            title: "list-mtext-id".to_string(),
                                                            link: "#list-mtext-id".to_string(),
                                                            item_type:
                                                                TableOfContentsItemType::MText,
                                                            children: vec![
                                                            ],
                                                        },
                                                    ],
                                                },
                                            ],
                                        },
                                        TableOfContentsItem {
                                            title: "table-id".to_string(),
                                            link: "#table-id".to_string(),
                                            item_type: TableOfContentsItemType::Table,
                                            children: vec![
                                                TableOfContentsItem {
                                                    title: "paragraph".to_string(),
                                                    link: "".to_string(),
                                                    item_type: TableOfContentsItemType::Paragraph,
                                                    children: vec![
                                                        TableOfContentsItem {
                                                            title: "table-mtext-id".to_string(),
                                                            link: "#table-mtext-id".to_string(),
                                                            item_type:
                                                                TableOfContentsItemType::MText,
                                                            children: vec![
                                                            ],
                                                        },
                                                    ],
                                                },
                                            ],
                                        },
                                    ],
                                },
                            ],
                        },
                    ],
                },
            ],
        })
    );

    test_toc!(
        toc_iwc_doc_section_par_all_but_mtext,
        toc_doc_0(),
        Some((
            HashSet::from([
                TableOfContentsItemType::Document,
                TableOfContentsItemType::Section,
                TableOfContentsItemType::Paragraph,
                TableOfContentsItemType::Emphasis,
                TableOfContentsItemType::CodeBlock,
                TableOfContentsItemType::Link,
                TableOfContentsItemType::List,
                TableOfContentsItemType::Table,
            ]),
            TableOfContentsFilterType::IncludeWithChildren
        )),
        Some(TableOfContentsItem {
            title: "Table of Contents".to_string(),
            link: ".".to_string(),
            item_type: TableOfContentsItemType::Document,
            children: vec![
                TableOfContentsItem {
                    title: "A H1 heading".to_string(),
                    link: "#a-h1-heading".to_string(),
                    item_type: TableOfContentsItemType::Section,
                    children: vec![
                        TableOfContentsItem {
                            title: "H2 heading".to_string(),
                            link: "#h2-id".to_string(),
                            item_type: TableOfContentsItemType::Section,
                            children: vec![
                                TableOfContentsItem {
                                    title: "H3".to_string(),
                                    link: "#h3".to_string(),
                                    item_type: TableOfContentsItemType::Section,
                                    children: vec![
                                    ],
                                },
                            ],
                        },
                        TableOfContentsItem {
                            title: "Another H2".to_string(),
                            link: "#another-h2".to_string(),
                            item_type: TableOfContentsItemType::Section,
                            children: vec![
                                TableOfContentsItem {
                                    title: "paragraph".to_string(),
                                    link: "".to_string(),
                                    item_type: TableOfContentsItemType::Paragraph,
                                    children: vec![
                                        TableOfContentsItem {
                                            title: "emphasis".to_string(),
                                            link: "#emphasis-id".to_string(),
                                            item_type: TableOfContentsItemType::Emphasis,
                                            children: vec![],
                                        },
                                        TableOfContentsItem {
                                            title: "codeblock-id".to_string(),
                                            link: "#codeblock-id".to_string(),
                                            item_type: TableOfContentsItemType::CodeBlock,
                                            children: vec![],
                                        },
                                        TableOfContentsItem {
                                            title: "link".to_string(),
                                            link: "#link-id".to_string(),
                                            item_type: TableOfContentsItemType::Link,
                                            children: vec![],
                                        },
                                        TableOfContentsItem {
                                            title: "list-id".to_string(),
                                            link: "#list-id".to_string(),
                                            item_type: TableOfContentsItemType::List,
                                            children: vec![],
                                        },
                                        TableOfContentsItem {
                                            title: "table-id".to_string(),
                                            link: "#table-id".to_string(),
                                            item_type: TableOfContentsItemType::Table,
                                            children: vec![],
                                        },
                                    ],
                                },
                            ],
                        },
                    ],
                },
            ],
        })
    );

    test_toc!(
        toc_iwc_through_id_less_list_and_table,
        toc_doc_1(),
        Some((
            HashSet::from([
                TableOfContentsItemType::Document,
                TableOfContentsItemType::MText,
            ]),
            TableOfContentsFilterType::IncludeWithChildren
        )),
        Some(TableOfContentsItem {
            title: "Table of Contents".to_string(),
            link: ".".to_string(),
            item_type: TableOfContentsItemType::Document,
            children: vec![
                TableOfContentsItem {
                    title: "paragraph".to_string(),
                    link: "".to_string(),
                    item_type: TableOfContentsItemType::Paragraph,
                    children: vec![
                        TableOfContentsItem {
                            title: "list".to_string(),
                            link: "".to_string(),
                            item_type: TableOfContentsItemType::List,
                            children: vec![
                                TableOfContentsItem {
                                    title: "paragraph".to_string(),
                                    link: "".to_string(),
                                    item_type: TableOfContentsItemType::Paragraph,
                                    children: vec![
                                        TableOfContentsItem {
                                            title: "list-mtext-id".to_string(),
                                            link: "#list-mtext-id".to_string(),
                                            item_type: TableOfContentsItemType::MText,
                                            children: vec![
                                            ],
                                        },
                                    ],
                                },
                            ],
                        },
                        TableOfContentsItem {
                            title: "table".to_string(),
                            link: "".to_string(),
                            item_type: TableOfContentsItemType::Table,
                            children: vec![
                                TableOfContentsItem {
                                    title: "paragraph".to_string(),
                                    link: "".to_string(),
                                    item_type: TableOfContentsItemType::Paragraph,
                                    children: vec![
                                        TableOfContentsItem {
                                            title: "table-mtext-id".to_string(),
                                            link: "#table-mtext-id".to_string(),
                                            item_type: TableOfContentsItemType::MText,
                                            children: vec![
                                            ],
                                        },
                                    ],
                                },
                            ],
                        },
                    ],
                },
            ],
        })
    );

}
