#[cfg(test)]
mod prune {
    use crate::*;
    use crate::actions::prune::*;

    macro_rules! test_prune_contentless {
        ($name:ident, $input:expr, $output:expr) => {
            #[test]
            fn $name() {
                let mut res = $input;
                res.prune_contentless();
                assert_eq!(res, $output);
            }
        }
    }

    test_prune_contentless!(
        pc_string_c0,
        "stay the same".to_string(),
        "stay the same".to_string()
    );

    test_prune_contentless!(
        pc_string_c1,
        "   ".to_string(),
        "".to_string()
    );

    test_prune_contentless!(
        pc_string_c2,
        "



        ".to_string(),
        "".to_string()
    );

    test_prune_contentless!(
        pc_string_c3,
        "  dont trim   ".to_string(),
        "  dont trim   ".to_string()
    );

    test_prune_contentless!(
        pc_tags,
        hset!(["needs".to_string(), "  ".to_string(), " ".to_string(), " prune ".to_string()]),
        hset!(["needs".to_string(), " prune ".to_string()])
    );

    test_prune_contentless!(
        pc_props,
        props!([
            ("please".to_string(), PropVal::Int(0)),
            ("key".to_string(), PropVal::String("  ".to_string())),
            (" ".to_string(), PropVal::String("hello".to_string())),
            (" leave ".to_string(), PropVal::Text("
             alone ".to_string())),
        ]),
        props!([
            ("please".to_string(), PropVal::Int(0)),
            (" leave ".to_string(), PropVal::Text("
             alone ".to_string())),
        ])
    );

    test_prune_contentless!(
        pc_section,
        Section {
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
                }),
                SectionItem::Paragraph(Paragraph {
                    items: vec![],
                    tags: hset!(["not content".to_string()]),
                    ..Default::default()
                })
            ],
            tags: hset!(["ok".to_string(), " \n".to_string()]),
            props: props!([
                ("ok".to_string(), PropVal::Int(0)),
                (" ".to_string(), PropVal::Int(0)),
            ]),
        },
        Section {
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
            tags: hset!(["ok".to_string()]),
            props: props!([("ok".to_string(), PropVal::Int(0))]),
        }
    );

    test_prune_contentless!(
        pc_heading,
        Heading {
            level: 0,
            items: vec![
                HeadingItem::String(" ".to_string()),
                HeadingItem::String("header".to_string()),
            ],
            tags: hset!(["ok".to_string(), " \n".to_string()]),
            props: props!([
                ("ok".to_string(), PropVal::Int(0)),
                (" ".to_string(), PropVal::Int(0)),
            ]),
        },
        Heading {
            level: 0,
            items: vec![
                HeadingItem::String("header".to_string()),
            ],
            tags: hset!(["ok".to_string()]),
            props: props!([("ok".to_string(), PropVal::Int(0))]),
        }
    );

    test_prune_contentless!(
        pc_paragraph,
        SectionItem::Paragraph(Paragraph {
            items: vec![
                ParagraphItem::Text(" ".to_string()),
                ParagraphItem::Text("p".to_string()),
                ParagraphItem::Text(" ".to_string()),
                ParagraphItem::Text("p".to_string()),
            ],
            tags: hset!([
                "ok".to_string(),
                " \n".to_string()
            ]),
            props: props!([
                ("ok".to_string(), PropVal::Int(0)),
                (" ".to_string(), PropVal::Int(0)),
            ]),
        }),
        SectionItem::Paragraph(Paragraph {
            items: vec![
                ParagraphItem::Text("p".to_string()),
                ParagraphItem::Text("p".to_string()),
            ],
            tags: hset!([
                "ok".to_string()
            ]),
            props: props!([
                ("ok".to_string(), PropVal::Int(0)),
            ]),
        })
    );

    test_prune_contentless!(
        pc_emphasis_c0,
        Emphasis {
            strength: EmStrength::Light,
            etype: EmType::Emphasis,
            text: " em ".to_string(),
            tags: hset!([
                "ok".to_string(),
                " \n".to_string()
            ]),
            props: props!([
                ("ok".to_string(), PropVal::Int(0)),
                (" ".to_string(), PropVal::Int(0)),
            ]),
        },
        Emphasis {
            strength: EmStrength::Light,
            etype: EmType::Emphasis,
            text: " em ".to_string(),
            tags: hset!([
                "ok".to_string(),
            ]),
            props: props!([
                ("ok".to_string(), PropVal::Int(0)),
            ]),
        }
    );

    test_prune_contentless!(
        pc_emphasis_c1,
        Emphasis {
            strength: EmStrength::Light,
            etype: EmType::Emphasis,
            text: "  ".to_string(),
            tags: hset!([
                "ok".to_string(),
                " \n".to_string()
            ]),
            props: props!([
                ("ok".to_string(), PropVal::Int(0)),
                (" ".to_string(), PropVal::Int(0)),
            ]),
        },
        Emphasis {
            strength: EmStrength::Light,
            etype: EmType::Emphasis,
            text: "".to_string(),
            tags: hset!([
                "ok".to_string(),
            ]),
            props: props!([
                ("ok".to_string(), PropVal::Int(0)),
            ]),
        }
    );

    test_prune_contentless!(
        pc_list,
        List {
            ltype: ListType::Identical,
            items: vec![
                Paragraph {
                    items: vec![
                        ParagraphItem::Text(" ".to_string()),
                        ParagraphItem::Text("p".to_string()),
                    ],
                    ..Default::default()
                },
                Paragraph {
                    items: vec![
                        ParagraphItem::Text(" \n ".to_string()),
                    ],
                    tags: hset!(["not content".to_string()]),
                    ..Default::default()
                },
            ],
            tags: hset!([
                "ok".to_string(),
                " \n".to_string()
            ]),
            props: props!([
                ("ok".to_string(), PropVal::Int(0)),
                (" ".to_string(), PropVal::Int(0)),
            ]),
        },
        List {
            ltype: ListType::Identical,
            items: vec![
                Paragraph {
                    items: vec![
                        ParagraphItem::Text("p".to_string()),
                    ],
                    ..Default::default()
                },
            ],
            tags: hset!([
                "ok".to_string(),
            ]),
            props: props!([
                ("ok".to_string(), PropVal::Int(0)),
            ]),
        }
    );

    test_prune_contentless!(
        pc_nav,
        Nav {
            description: "desc".to_string(),
            subs: vec![
                Nav {
                    description: "desc".to_string(),
                    subs: vec![],
                    links: vec![],
                    tags: hset!(["not content".to_string()]),
                    ..Default::default()
                }
            ],
            links: vec![
                Link {
                    url: "url".to_string(),
                    items: vec![
                        LinkItem::String("link".to_string()),
                    ],
                    ..Default::default()
                },
                Link {
                    url: "".to_string(),
                    items: vec![],
                    tags: hset!(["not content".to_string()]),
                    ..Default::default()
                },
            ],
            ..Default::default()
        },
        Nav {
            description: "desc".to_string(),
            subs: vec![],
            links: vec![
                Link {
                    url: "url".to_string(),
                    items: vec![
                        LinkItem::String("link".to_string()),
                    ],
                    ..Default::default()
                }
            ],
            ..Default::default()
        }
    );

    test_prune_contentless!(
        pc_nav_1,
        Doc {
            navs: vec![
                Nav {
                    description: "".to_string(),
                    subs: vec![
                        Nav {
                            description: "".to_string(),
                            subs: vec![],
                            links: vec![
                                Link {
                                    url: "".to_string(),
                                    items: vec![
                                        LinkItem::String("".to_string()),
                                    ],
                                    ..Default::default()
                                },
                            ],
                            tags: hset!(["not content".to_string()]),
                            ..Default::default()
                        }
                    ],
                    links: vec![
                        Link {
                            url: "".to_string(),
                            items: vec![],
                            tags: hset!(["not content".to_string()]),
                            ..Default::default()
                        },
                    ],
                    ..Default::default()
                },
            ],
            ..Default::default()
        },
        Doc {
            navs: vec![
            ],
            ..Default::default()
        }
    );

    test_prune_contentless!(
        pc_link,
        Link {
            url: "  ".to_string(),
            items: vec![
                LinkItem::String("\n".to_string()),
            ],
            tags: hset!([
                "ok".to_string(),
                " \n".to_string()
            ]),
            props: props!([
                ("ok".to_string(), PropVal::Int(0)),
                (" ".to_string(), PropVal::Int(0)),
            ]),
        },
        Link {
            url: "".to_string(),
            items: vec![],
            tags: hset!([
                "ok".to_string(),
            ]),
            props: props!([
                ("ok".to_string(), PropVal::Int(0)),
            ]),
        }
    );

    test_prune_contentless!(
        pc_code,
        CodeBlock {
            language: " ".to_string(),
            mode: CodeModeHint::Show,
            code: "\n\t".to_string(),
            tags: hset!([
                "ok".to_string(),
                " \n".to_string()
            ]),
            props: props!([
                ("ok".to_string(), PropVal::Int(0)),
                (" ".to_string(), PropVal::Int(0)),
            ]),
        },
        CodeBlock {
            language: "".to_string(),
            mode: CodeModeHint::Show,
            code: "".to_string(),
            tags: hset!([
                "ok".to_string(),
            ]),
            props: props!([
                ("ok".to_string(), PropVal::Int(0)),
            ]),
        }
    );

    test_prune_contentless!(
        pc_mtext,
        TextWithMeta {
            text: " ".to_string(),
            tags: hset!([
                "ok".to_string(),
                " \n".to_string()
            ]),
            props: props!([
                ("ok".to_string(), PropVal::Int(0)),
                (" ".to_string(), PropVal::Int(0)),
            ]),
        },
        TextWithMeta {
            text: "".to_string(),
            tags: hset!([
                "ok".to_string(),
            ]),
            props: props!([
                ("ok".to_string(), PropVal::Int(0)),
            ]),
        }
    );

    test_prune_contentless!(
        pc_table,
        Table {
            rows: vec![
                TableRow {
                    items: vec![
                        Paragraph {
                            items: vec![
                                ParagraphItem::Text(" ".to_string()),
                                ParagraphItem::Text("p".to_string()),
                            ],
                            tags: hset!([
                                "ok".to_string(),
                                " \n".to_string()
                            ]),
                            props: props!([
                                ("ok".to_string(), PropVal::Int(0)),
                                (" ".to_string(), PropVal::Int(0)),
                            ]),
                        },
                        Paragraph {
                            items: vec![
                                ParagraphItem::Text(" \n ".to_string()),
                            ],
                            tags: hset!(["not content".to_string()]),
                            ..Default::default()
                        },
                    ],
                    is_header: false,
                    tags: hset!([
                        "ok".to_string(),
                        " \n".to_string()
                    ]),
                    props: props!([
                        ("ok".to_string(), PropVal::Int(0)),
                        (" ".to_string(), PropVal::Int(0)),
                    ]),
                },
            ],
            tags: hset!([
                "ok".to_string(),
                " \n".to_string()
            ]),
            props: props!([
                ("ok".to_string(), PropVal::Int(0)),
                (" ".to_string(), PropVal::Int(0)),
            ]),
        },
        Table {
            rows: vec![
                TableRow {
                    items: vec![
                        Paragraph {
                            items: vec![
                                ParagraphItem::Text("p".to_string()),
                            ],
                            tags: hset!([
                                "ok".to_string(),
                            ]),
                            props: props!([
                                ("ok".to_string(), PropVal::Int(0)),
                            ]),
                        },
                    ],
                    is_header: false,
                    tags: hset!([
                        "ok".to_string(),
                    ]),
                    props: props!([
                        ("ok".to_string(), PropVal::Int(0)),
                    ]),
                },
            ],
            tags: hset!([
                "ok".to_string(),
            ]),
            props: props!([
                ("ok".to_string(), PropVal::Int(0)),
            ]),
        }
    );
}

