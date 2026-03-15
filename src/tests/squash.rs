#[cfg(test)]
mod squash {
    use crate::*;

    macro_rules! test_squash {
        ($name:ident, $input:expr, $output:expr) => {
            #[test]
            fn $name() {
                let mut res = $input;
                res.squash();
                assert_eq!(res, $output);
            }
        }
    }

    test_squash!(
        sq_doc,
        Doc {
            items: vec![DocItem::Paragraph(Paragraph {
                items: vec![
                    ParagraphItem::Text("a".to_string()),
                    ParagraphItem::Text("\n".to_string()),
                    ParagraphItem::Text("b\n".to_string()),
                    ParagraphItem::Em(Emphasis {
                        text: "em".to_string(),
                        ..Default::default()
                    }),
                    ParagraphItem::Text("c\n".to_string()),
                    ParagraphItem::MText(TextWithMeta {
                        text: "A".to_string(),
                        tags: hset!(["same"]),
                        ..Default::default()
                    }),
                    ParagraphItem::MText(TextWithMeta {
                        text: "B".to_string(),
                        tags: hset!(["same"]),
                        ..Default::default()
                    }),
                    ParagraphItem::MText(TextWithMeta {
                        text: "C".to_string(),
                        tags: hset!(["different"]),
                        ..Default::default()
                    }),
                    ParagraphItem::MText(TextWithMeta {
                        text: "D".to_string(),
                        tags: hset!(["same"]),
                        ..Default::default()
                    }),
                    ParagraphItem::MText(TextWithMeta {
                        text: "E".to_string(),
                        tags: hset!(["same"]),
                        ..Default::default()
                    }),
                    ParagraphItem::Em(Emphasis {
                        strength: EmStrength::Strong,
                        etype: EmType::Deemphasis,
                        text: "de-em".to_string(),
                        props: props!([("p".to_string(), PropVal::Int(0))]),
                        ..Default::default()
                    }),
                    ParagraphItem::Em(Emphasis {
                        strength: EmStrength::Strong,
                        etype: EmType::Deemphasis,
                        text: "de-em".to_string(),
                        props: props!([("p".to_string(), PropVal::Int(0))]),
                        ..Default::default()
                    }),
                    ParagraphItem::Em(Emphasis {
                        strength: EmStrength::Medium,
                        etype: EmType::Deemphasis,
                        text: "de-em".to_string(),
                        props: props!([("p".to_string(), PropVal::Int(0))]),
                        ..Default::default()
                    }),
                    ParagraphItem::Text("d\n".to_string()),
                    ParagraphItem::MText(TextWithMeta {
                        text: "F".to_string(),
                        tags: hset!(["same"]),
                        ..Default::default()
                    }),
                    ParagraphItem::Em(Emphasis {
                        strength: EmStrength::Strong,
                        etype: EmType::Deemphasis,
                        text: "de-em".to_string(),
                        props: props!([("p".to_string(), PropVal::Int(0))]),
                        ..Default::default()
                    }),
                    ParagraphItem::MText(TextWithMeta {
                        text: "G".to_string(),
                        ..Default::default()
                    }),
                ],
                ..Default::default()
            })],
            ..Default::default()
        },
        Doc {
            items: vec![DocItem::Paragraph(Paragraph {
                items: vec![
                    ParagraphItem::Text("a\nb\n".to_string()),
                    ParagraphItem::Em(Emphasis {
                        text: "em".to_string(),
                        ..Default::default()
                    }),
                    ParagraphItem::Text("c\n".to_string()),
                    ParagraphItem::MText(TextWithMeta {
                        text: "AB".to_string(),
                        tags: hset!(["same"]),
                        ..Default::default()
                    }),
                    ParagraphItem::MText(TextWithMeta {
                        text: "C".to_string(),
                        tags: hset!(["different"]),
                        ..Default::default()
                    }),
                    ParagraphItem::MText(TextWithMeta {
                        text: "DE".to_string(),
                        tags: hset!(["same"]),
                        ..Default::default()
                    }),
                    ParagraphItem::Em(Emphasis {
                        strength: EmStrength::Strong,
                        etype: EmType::Deemphasis,
                        text: "de-emde-em".to_string(),
                        props: props!([("p".to_string(), PropVal::Int(0))]),
                        ..Default::default()
                    }),
                    ParagraphItem::Em(Emphasis {
                        strength: EmStrength::Medium,
                        etype: EmType::Deemphasis,
                        text: "de-em".to_string(),
                        props: props!([("p".to_string(), PropVal::Int(0))]),
                        ..Default::default()
                    }),
                    ParagraphItem::Text("d\n".to_string()),
                    ParagraphItem::MText(TextWithMeta {
                        text: "F".to_string(),
                        tags: hset!(["same"]),
                        ..Default::default()
                    }),
                    ParagraphItem::Em(Emphasis {
                        strength: EmStrength::Strong,
                        etype: EmType::Deemphasis,
                        text: "de-em".to_string(),
                        props: props!([("p".to_string(), PropVal::Int(0))]),
                        ..Default::default()
                    }),
                    ParagraphItem::Text("G".to_string()),
                ],
                ..Default::default()
            })],
            ..Default::default()
        }
    );

    test_squash!(
        sq_section,
        Section {
            items: vec![
                SectionItem::Paragraph(Paragraph {
                    items: vec![
                        ParagraphItem::Text("a".to_string()),
                        ParagraphItem::Text("\nb".to_string()),
                    ],
                    ..Default::default()
                }),
                SectionItem::Section(Section {
                    items: vec![
                        SectionItem::Paragraph(Paragraph {
                            items: vec![
                                ParagraphItem::Text("a".to_string()),
                                ParagraphItem::Text("\nb".to_string()),
                            ],
                            ..Default::default()
                        }),
                    ],
                    ..Default::default()
                }),
            ],
            ..Default::default()
        },
        Section {
            items: vec![
                SectionItem::Paragraph(Paragraph {
                    items: vec![
                        ParagraphItem::Text("a\nb".to_string()),
                    ],
                    ..Default::default()
                }),
                SectionItem::Section(Section {
                    items: vec![
                        SectionItem::Paragraph(Paragraph {
                            items: vec![
                                ParagraphItem::Text("a\nb".to_string()),
                            ],
                            ..Default::default()
                        }),
                    ],
                    ..Default::default()
                }),
            ],
            ..Default::default()
        }
    );

    test_squash!(
        sq_section_in_doc,
        Doc {
            items: vec![
                DocItem::Section(
                    Section {
                        items: vec![
                            SectionItem::Paragraph(Paragraph {
                                items: vec![
                                    ParagraphItem::Text("a".to_string()),
                                    ParagraphItem::Text("\nb".to_string()),
                                ],
                                ..Default::default()
                            }),
                        ],
                        ..Default::default()
                    }
                ),
            ],
            ..Default::default()
        },
        Doc {
            items: vec![
                DocItem::Section(
                    Section {
                        items: vec![
                            SectionItem::Paragraph(Paragraph {
                                items: vec![
                                    ParagraphItem::Text("a\nb".to_string()),
                                ],
                                ..Default::default()
                            }),
                        ],
                        ..Default::default()
                    }
                ),
            ],
            ..Default::default()
        }
    );

    test_squash!(
        sq_heading,
        Heading {
            items: vec![
                EmOrText::Text("a\n".to_string()),
                EmOrText::Text("b\n".to_string()),
                EmOrText::Em(Emphasis {
                    text: "em ".to_string(),
                    ..Default::default()
                }),
                EmOrText::Em(Emphasis {
                    text: "em".to_string(),
                    ..Default::default()
                }),
                EmOrText::Text("c".to_string()),
                EmOrText::Em(Emphasis {
                    text: "em".to_string(),
                    ..Default::default()
                }),
                EmOrText::Text("d".to_string()),
                EmOrText::Em(Emphasis {
                    text: "em".to_string(),
                    tags: hset!(["A"]),
                    ..Default::default()
                }),
                EmOrText::Em(Emphasis {
                    text: "em".to_string(),
                    tags: hset!(["A"]),
                    ..Default::default()
                }),
                EmOrText::Em(Emphasis {
                    text: "em".to_string(),
                    tags: hset!(["A", "B"]),
                    ..Default::default()
                }),
            ],
            ..Default::default()
        },
        Heading {
            items: vec![
                EmOrText::Text("a\nb\n".to_string()),
                EmOrText::Em(Emphasis {
                    text: "em em".to_string(),
                    ..Default::default()
                }),
                EmOrText::Text("c".to_string()),
                EmOrText::Em(Emphasis {
                    text: "em".to_string(),
                    ..Default::default()
                }),
                EmOrText::Text("d".to_string()),
                EmOrText::Em(Emphasis {
                    text: "emem".to_string(),
                    tags: hset!(["A"]),
                    ..Default::default()
                }),
                EmOrText::Em(Emphasis {
                    text: "em".to_string(),
                    tags: hset!(["A", "B"]),
                    ..Default::default()
                }),
            ],
            ..Default::default()
        }
    );

    test_squash!(
        sq_paragraph_c0,
        Paragraph {
            items: vec![
                ParagraphItem::Text("a".to_string()),
                ParagraphItem::Text("\n".to_string()),
                ParagraphItem::Text("b\n".to_string()),
                ParagraphItem::Em(Emphasis {
                    text: "em".to_string(),
                    ..Default::default()
                }),
                ParagraphItem::Text("c\n".to_string()),
                ParagraphItem::MText(TextWithMeta {
                    text: "A".to_string(),
                    tags: hset!(["same"]),
                    ..Default::default()
                }),
                ParagraphItem::MText(TextWithMeta {
                    text: "B".to_string(),
                    tags: hset!(["same"]),
                    ..Default::default()
                }),
                ParagraphItem::MText(TextWithMeta {
                    text: "C".to_string(),
                    tags: hset!(["different"]),
                    ..Default::default()
                }),
                ParagraphItem::MText(TextWithMeta {
                    text: "D".to_string(),
                    tags: hset!(["same"]),
                    ..Default::default()
                }),
                ParagraphItem::MText(TextWithMeta {
                    text: "E".to_string(),
                    tags: hset!(["same"]),
                    ..Default::default()
                }),
                ParagraphItem::Em(Emphasis {
                    strength: EmStrength::Strong,
                    etype: EmType::Deemphasis,
                    text: "de-em".to_string(),
                    props: props!([("p".to_string(), PropVal::Int(0))]),
                    ..Default::default()
                }),
                ParagraphItem::Em(Emphasis {
                    strength: EmStrength::Strong,
                    etype: EmType::Deemphasis,
                    text: "de-em".to_string(),
                    props: props!([("p".to_string(), PropVal::Int(0))]),
                    ..Default::default()
                }),
                ParagraphItem::Em(Emphasis {
                    strength: EmStrength::Medium,
                    etype: EmType::Deemphasis,
                    text: "de-em".to_string(),
                    props: props!([("p".to_string(), PropVal::Int(0))]),
                    ..Default::default()
                }),
                ParagraphItem::Text("d\n".to_string()),
                ParagraphItem::MText(TextWithMeta {
                    text: "F".to_string(),
                    tags: hset!(["same"]),
                    ..Default::default()
                }),
                ParagraphItem::Em(Emphasis {
                    strength: EmStrength::Strong,
                    etype: EmType::Deemphasis,
                    text: "de-em".to_string(),
                    props: props!([("p".to_string(), PropVal::Int(0))]),
                    ..Default::default()
                }),
            ],
            ..Default::default()
        },
        Paragraph {
            items: vec![
                ParagraphItem::Text("a\nb\n".to_string()),
                ParagraphItem::Em(Emphasis {
                    text: "em".to_string(),
                    ..Default::default()
                }),
                ParagraphItem::Text("c\n".to_string()),
                ParagraphItem::MText(TextWithMeta {
                    text: "AB".to_string(),
                    tags: hset!(["same"]),
                    ..Default::default()
                }),
                ParagraphItem::MText(TextWithMeta {
                    text: "C".to_string(),
                    tags: hset!(["different"]),
                    ..Default::default()
                }),
                ParagraphItem::MText(TextWithMeta {
                    text: "DE".to_string(),
                    tags: hset!(["same"]),
                    ..Default::default()
                }),
                ParagraphItem::Em(Emphasis {
                    strength: EmStrength::Strong,
                    etype: EmType::Deemphasis,
                    text: "de-emde-em".to_string(),
                    props: props!([("p".to_string(), PropVal::Int(0))]),
                    ..Default::default()
                }),
                ParagraphItem::Em(Emphasis {
                    strength: EmStrength::Medium,
                    etype: EmType::Deemphasis,
                    text: "de-em".to_string(),
                    props: props!([("p".to_string(), PropVal::Int(0))]),
                    ..Default::default()
                }),
                ParagraphItem::Text("d\n".to_string()),
                ParagraphItem::MText(TextWithMeta {
                    text: "F".to_string(),
                    tags: hset!(["same"]),
                    ..Default::default()
                }),
                ParagraphItem::Em(Emphasis {
                    strength: EmStrength::Strong,
                    etype: EmType::Deemphasis,
                    text: "de-em".to_string(),
                    props: props!([("p".to_string(), PropVal::Int(0))]),
                    ..Default::default()
                }),
            ],
            ..Default::default()
        }
    );

    test_squash!(
        sq_paragraph_c1,
        Paragraph {
            items: vec![
                ParagraphItem::Link(Link {
                    items: vec![
                        EmOrText::Text("a".to_string()),
                        EmOrText::Text("\n".to_string()),
                        EmOrText::Text("b".to_string()),
                    ],
                    ..Default::default()
                }),
                ParagraphItem::List(List {
                    items: vec![
                        Paragraph {
                            items: vec![
                                ParagraphItem::Text("a".to_string()),
                                ParagraphItem::Text("\n".to_string()),
                                ParagraphItem::Text("b".to_string()),
                            ],
                            ..Default::default()
                        }
                    ],
                    ..Default::default()
                }),
            ],
            ..Default::default()
        },
        Paragraph {
            items: vec![
                ParagraphItem::Link(Link {
                    items: vec![
                        EmOrText::Text("a\nb".to_string()),
                    ],
                    ..Default::default()
                }),
                ParagraphItem::List(List {
                    items: vec![
                        Paragraph {
                            items: vec![
                                ParagraphItem::Text("a\nb".to_string()),
                            ],
                            ..Default::default()
                        }
                    ],
                    ..Default::default()
                }),
            ],
            ..Default::default()
        }
    );

    test_squash!(
        sq_list,
        List {
            items: vec![
                Paragraph {
                    items: vec![
                        ParagraphItem::Text("a".to_string()),
                        ParagraphItem::Text("\n".to_string()),
                        ParagraphItem::Text("b".to_string()),
                    ],
                    ..Default::default()
                }
            ],
            ..Default::default()
        },
        List {
            items: vec![
                Paragraph {
                    items: vec![
                        ParagraphItem::Text("a\nb".to_string()),
                    ],
                    ..Default::default()
                }
            ],
            ..Default::default()
        }
    );

    test_squash!(
        sq_link,
        Link {
            items: vec![
                EmOrText::Text("a".to_string()),
                EmOrText::Text("\n".to_string()),
                EmOrText::Text("b\n".to_string()),
                EmOrText::Em(Emphasis {
                    text: "em ".to_string(),
                    ..Default::default()
                }),
                EmOrText::Em(Emphasis {
                    text: "em".to_string(),
                    ..Default::default()
                }),
                EmOrText::Text("c".to_string()),
                EmOrText::Em(Emphasis {
                    text: "em".to_string(),
                    ..Default::default()
                }),
                EmOrText::Text("d".to_string()),
                EmOrText::Em(Emphasis {
                    text: "em".to_string(),
                    tags: hset!(["A"]),
                    ..Default::default()
                }),
                EmOrText::Em(Emphasis {
                    text: "em".to_string(),
                    tags: hset!(["A"]),
                    ..Default::default()
                }),
                EmOrText::Em(Emphasis {
                    text: "em".to_string(),
                    tags: hset!(["A", "B"]),
                    ..Default::default()
                }),
            ],
            ..Default::default()
        },
        Link {
            items: vec![
                EmOrText::Text("a\nb\n".to_string()),
                EmOrText::Em(Emphasis {
                    text: "em em".to_string(),
                    ..Default::default()
                }),
                EmOrText::Text("c".to_string()),
                EmOrText::Em(Emphasis {
                    text: "em".to_string(),
                    ..Default::default()
                }),
                EmOrText::Text("d".to_string()),
                EmOrText::Em(Emphasis {
                    text: "emem".to_string(),
                    tags: hset!(["A"]),
                    ..Default::default()
                }),
                EmOrText::Em(Emphasis {
                    text: "em".to_string(),
                    tags: hset!(["A", "B"]),
                    ..Default::default()
                }),
            ],
            ..Default::default()
        }
    );

    test_squash!(
        sq_table,
        Table {
            rows: vec![
                TableRow {
                    items: vec![
                        Paragraph {
                            items: vec![
                                ParagraphItem::Text("a".to_string()),
                                ParagraphItem::Text("\n".to_string()),
                                ParagraphItem::Text("b".to_string()),
                            ],
                            ..Default::default()
                        },
                    ],
                    ..Default::default()
                },
            ],
            ..Default::default()
        },
        Table {
            rows: vec![
                TableRow {
                    items: vec![
                        Paragraph {
                            items: vec![
                                ParagraphItem::Text("a\nb".to_string()),
                            ],
                            ..Default::default()
                        },
                    ],
                    ..Default::default()
                },
            ],
            ..Default::default()
        }
    );

}
