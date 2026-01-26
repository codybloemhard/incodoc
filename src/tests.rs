#[cfg(test)]
mod tests {
    use crate::*;
    use crate::parsing::*;
    use crate::output::*;
    use crate::actions::*;

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
                    doc_a.prune_errors();
                    doc_a.prune_contentless();
                    doc_out(&doc_a, &mut output);
                    let doc_b = parse(&output).expect("test_out: could not parse doc b");
                    assert_eq!(doc_a, doc_b);
                }
            }
        }
    }

    macro_rules! test_par {
        ($name:ident, $string:expr, $result:expr) => {
            #[test]
            fn $name() {
                let final_string = format!("{}{}{}", "par {\n", $string, "\n}");
                let final_res = Doc {
                    items: vec![DocItem::Paragraph($result)],
                    ..Default::default()
                };
                let doc_a_raw = parse(&final_string);
                assert_eq!(doc_a_raw, Ok(final_res));
                if let Ok(mut doc_a) = doc_a_raw {
                    let mut output = String::new();
                    doc_a.prune_errors();
                    doc_a.prune_contentless();
                    doc_out(&doc_a, &mut output);
                    let doc_b = parse(&output).expect("test_out: could not parse doc b");
                    assert_eq!(doc_a, doc_b);
                }
            }
        }
    }

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
        sq_heading,
        Heading {
            items: vec![
                HeadingItem::String("a\n".to_string()),
                HeadingItem::String("b\n".to_string()),
                HeadingItem::Em(Emphasis {
                    text: "em ".to_string(),
                    ..Default::default()
                }),
                HeadingItem::Em(Emphasis {
                    text: "em".to_string(),
                    ..Default::default()
                }),
                HeadingItem::String("c".to_string()),
                HeadingItem::Em(Emphasis {
                    text: "em".to_string(),
                    ..Default::default()
                }),
                HeadingItem::String("d".to_string()),
                HeadingItem::Em(Emphasis {
                    text: "em".to_string(),
                    tags: hset!(["A"]),
                    ..Default::default()
                }),
                HeadingItem::Em(Emphasis {
                    text: "em".to_string(),
                    tags: hset!(["A"]),
                    ..Default::default()
                }),
                HeadingItem::Em(Emphasis {
                    text: "em".to_string(),
                    tags: hset!(["A", "B"]),
                    ..Default::default()
                }),
            ],
            ..Default::default()
        },
        Heading {
            items: vec![
                HeadingItem::String("a\nb\n".to_string()),
                HeadingItem::Em(Emphasis {
                    text: "em em".to_string(),
                    ..Default::default()
                }),
                HeadingItem::String("c".to_string()),
                HeadingItem::Em(Emphasis {
                    text: "em".to_string(),
                    ..Default::default()
                }),
                HeadingItem::String("d".to_string()),
                HeadingItem::Em(Emphasis {
                    text: "emem".to_string(),
                    tags: hset!(["A"]),
                    ..Default::default()
                }),
                HeadingItem::Em(Emphasis {
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
                        LinkItem::String("a".to_string()),
                        LinkItem::String("\n".to_string()),
                        LinkItem::String("b".to_string()),
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
                        LinkItem::String("a\nb".to_string()),
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
                LinkItem::String("a".to_string()),
                LinkItem::String("\n".to_string()),
                LinkItem::String("b\n".to_string()),
                LinkItem::Em(Emphasis {
                    text: "em ".to_string(),
                    ..Default::default()
                }),
                LinkItem::Em(Emphasis {
                    text: "em".to_string(),
                    ..Default::default()
                }),
                LinkItem::String("c".to_string()),
                LinkItem::Em(Emphasis {
                    text: "em".to_string(),
                    ..Default::default()
                }),
                LinkItem::String("d".to_string()),
                LinkItem::Em(Emphasis {
                    text: "em".to_string(),
                    tags: hset!(["A"]),
                    ..Default::default()
                }),
                LinkItem::Em(Emphasis {
                    text: "em".to_string(),
                    tags: hset!(["A"]),
                    ..Default::default()
                }),
                LinkItem::Em(Emphasis {
                    text: "em".to_string(),
                    tags: hset!(["A", "B"]),
                    ..Default::default()
                }),
            ],
            ..Default::default()
        },
        Link {
            items: vec![
                LinkItem::String("a\nb\n".to_string()),
                LinkItem::Em(Emphasis {
                    text: "em em".to_string(),
                    ..Default::default()
                }),
                LinkItem::String("c".to_string()),
                LinkItem::Em(Emphasis {
                    text: "em".to_string(),
                    ..Default::default()
                }),
                LinkItem::String("d".to_string()),
                LinkItem::Em(Emphasis {
                    text: "emem".to_string(),
                    tags: hset!(["A"]),
                    ..Default::default()
                }),
                LinkItem::Em(Emphasis {
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

    macro_rules! test_toc {
        ($name:ident, $filter:expr, $output:expr) => {
            #[test]
            fn $name() {
                let doc = toc_doc();
                let toc = doc.get_table_of_contents(&$filter);
                assert_eq!(toc, $output);
            }
        }
    }

    fn toc_doc() -> Doc {
        Doc {
            items: vec![
                DocItem::Nav(Nav {
                    description: "nav".to_string(),
                    ..Default::default()
                }),
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
                                            text: "mtext a".to_string(),
                                            ..Default::default()
                                        }),
                                        ParagraphItem::MText(TextWithMeta {
                                            text: "mtext a".to_string(),
                                            props: props!([
                                                ("id".to_string(), PropVal::String("id0".to_string())),
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

    test_toc!(
        toc_just_doc,
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

    test!(
        po_empty,
        "",
        Doc::default()
    );

    test!(
        po_empty_comment0,
        "// comment",
        Doc::default()
    );

    test!(
        po_empty_comment1,
        "
        /* comment */
        ",
        Doc::default()
    );

    test!(
        po_empty_comment2,
        "
        //
        ",
        Doc::default()
    );

    test!(
        po_empty_comment3,
        "
        /**/
        ",
        Doc::default()
    );

    test_par!(
        po_text_c0,
        "'this is text'",
        Paragraph {
            items: vec![
                ParagraphItem::Text("this is text".to_string())
            ],
            ..Default::default()
        }
    );

    test_par!(
        po_text_c1,
        "
        '
        this is text
        '
        ",
        Paragraph {
            items: vec![
                ParagraphItem::Text("this is text".to_string())
            ],
            ..Default::default()
        }
    );

    test_par!(
        po_text_c2,
        "
        '
        this is text
        '
        ",
        Paragraph {
            items: vec![
                ParagraphItem::Text("this is text".to_string())
            ],
            ..Default::default()
        }
    );

    test_par!(
        po_text_c3,
        "
        '
        this is     text
        '
        ",
        Paragraph {
            items: vec![
                ParagraphItem::Text("this is text".to_string())
            ],
            ..Default::default()
        }
    );

    test_par!(
        po_text_c4,
        "
        '
        this
          is     text



        '
        ",
        Paragraph {
            items: vec![
                ParagraphItem::Text("this\nis text".to_string())
            ],
            ..Default::default()
        }
    );

    test_par!(
        po_text_c5,
        "
        ' pre'
        ",
        Paragraph {
            items: vec![
                ParagraphItem::Text(" pre".to_string())
            ],
            ..Default::default()
        }
    );

    test_par!(
        po_text_c6,
        "
        '    pre'
        ",
        Paragraph {
            items: vec![
                ParagraphItem::Text(" pre".to_string())
            ],
            ..Default::default()
        }
    );

    test_par!(
        po_text_c7,
        "
        'post '
        ",
        Paragraph {
            items: vec![
                ParagraphItem::Text("post ".to_string())
            ],
            ..Default::default()
        }
    );

    test_par!(
        po_text_c8,
        "
        'post        '
        ",
        Paragraph {
            items: vec![
                ParagraphItem::Text("post ".to_string())
            ],
            ..Default::default()
        }
    );

    test_par!(
        po_text_c9,
        "
        '   prepost        '
        ",
        Paragraph {
            items: vec![
                ParagraphItem::Text(" prepost ".to_string())
            ],
            ..Default::default()
        }
    );

    test_par!(
        po_text_c10,
        "
        '   pre


        '
        ",
        Paragraph {
            items: vec![
                ParagraphItem::Text(" pre".to_string())
            ],
            ..Default::default()
        }
    );

    test_par!(
        po_text_meta0,
        "
        'text'{ tags { } }
        ",
        Paragraph {
            items: vec![ParagraphItem::Text("text".to_string())],
            ..Default::default()
        }
    );

    test_par!(
        po_text_meta1,
        "
        'text'{ props { } }
        ",
        Paragraph {
            items: vec![ParagraphItem::Text("text".to_string())],
            ..Default::default()
        }
    );

    test_par!(
        po_text_meta2,
        "
        'text'{ tags { }, props { } }
        ",
        Paragraph {
            items: vec![ParagraphItem::Text("text".to_string())],
            ..Default::default()
        }
    );

    test_par!(
        po_text_meta3,
        "
        'text'{ tags { \"a\", \"b\" } }
        ",
        Paragraph {
            items: vec![ParagraphItem::MText(TextWithMeta {
                text: "text".to_string(),
                tags: hset!(["a", "b"]),
                props: Props::default(),
            })],
            ..Default::default()
        }
    );

    test_par!(
        po_text_meta4,
        "
        'text'{ props { (\"a\", 0), (\"b\", 1), (\"a\", 2) } }
        ",
        Paragraph {
            items: vec![ParagraphItem::MText(TextWithMeta {
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

    test_par!(
        po_text_meta5,
        "
        'text'{ tags { \"a\", \"b\" }, props { (\"a\", 0), (\"b\", 1), (\"a\", 2) } }
        ",
        Paragraph {
            items: vec![ParagraphItem::MText(TextWithMeta {
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

    test_par!(
        po_text_comment0,
        "
        // comment
        'text' // comment
        // comment
        ",
        Paragraph {
            items: vec![ParagraphItem::Text("text".to_string())],
            ..Default::default()
        }
    );

    test_par!(
        po_text_comment1,
        "
        /* comment */
        /* comment */ 'text' /* comment */
        /* comment */
        ",
        Paragraph {
            items: vec![ParagraphItem::Text("text".to_string())],
            ..Default::default()
        }
    );

    test_par!(
        po_text_comment2,
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
        Paragraph {
            items: vec![ParagraphItem::MText(TextWithMeta {
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

    test_par!(
        po_text_comment3,
        "
        /* comment */
        /* comment */ '//comment' /* comment */,
        /* comment */ '/*comment*/' /* comment */
        /* comment */
        ",
        Paragraph {
            items: vec![
                ParagraphItem::Text("//comment".to_string()),
                ParagraphItem::Text("/*comment*/".to_string()),
            ],
            ..Default::default()
        }
    );

    test!(
        po_props_empty_f0,
        "props{}",
        Doc::default()
    );

    test!(
        po_props_empty_f1,
        "props {}",
        Doc::default()
    );

    test!(
        po_props_empty_f2,
        "props {  }, ",
        Doc::default()
    );

    test!(
        po_props_empty_f3,
        "
        props {

        },
        ",
        Doc::default()
    );

    test!(
        po_props_tuple_string_f0,
        "props{(\"prop\",\"test\")}",
        Doc {
            props: props!([("prop".to_string(), PropVal::String("test".to_string()))]),
            ..Default::default()
        }
    );

    test!(
        po_props_tuple_string_f1,
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
        po_props_tuple_int_c0,
        "props { (\"prop\", 5) }",
        Doc {
            props: props!([("prop".to_string(), PropVal::Int(5))]),
            ..Default::default()
        }
    );

    test!(
        po_props_tuple_int_c1,
        "props { (\"prop\", +7) }",
        Doc {
            props: props!([("prop".to_string(), PropVal::Int(7))]),
            ..Default::default()
        }
    );

    test!(
        po_props_tuple_int_c2,
        "props { (\"prop\", -1) }",
        Doc {
            props: props!([("prop".to_string(), PropVal::Int(-1))]),
            ..Default::default()
        }
    );

    test!(
        po_props_tuple_date_c0,
        "props { (\"prop\", 2000/01/01) }",
        Doc {
            props: props!([("prop".to_string(), PropVal::Date(Date::new(2000, 1, 1).unwrap()))]),
            ..Default::default()
        }
    );

    test!(
        po_props_tuple_date_c1,
        "props { (\"prop\", -3434/01/01) }",
        Doc {
            props: props!([("prop".to_string(), PropVal::Date(Date::new(-3434, 1, 1).unwrap()))]),
            ..Default::default()
        }
    );

    test!(
        po_props_tuple_date_c2,
        "props { (\"prop\", 2000/13/01) }",
        Doc {
            props: props!([
                ("prop".to_string(), PropVal::Error(PropValError::Date(DateError::MonthRange(13))))
            ]),
            ..Default::default()
        }
    );

    test!(
        po_props_tuple_date_c3,
        "props { (\"prop\", 2000/01/32) }",
        Doc {
            props: props!([
                ("prop".to_string(), PropVal::Error(PropValError::Date(DateError::DayRange(32))))
            ]),
            ..Default::default()
        }
    );

    test!(
        po_props_tuple_date_c4,
        "props { (\"prop\", 1/1/1) }",
        Doc {
            props: props!([
                ("prop".to_string(), PropVal::Date(Date::new(1, 1, 1).unwrap())),
            ]),
            ..Default::default()
        }
    );

    test!(
        po_props_tuple_date_c5,
        "props { (\"prop\", 0/0/0) }",
        Doc {
            props: props!([
                ("prop".to_string(), PropVal::Error(PropValError::Date(DateError::MonthRange(0))))
            ]),
            ..Default::default()
        }
    );

    test!(
        po_props_tuple_date_c6,
        "props { (\"prop\", 0/1/0) }",
        Doc {
            props: props!([
                ("prop".to_string(), PropVal::Error(PropValError::Date(DateError::DayRange(0))))
            ]),
            ..Default::default()
        }
    );

    test!(
        po_props_tuple_text_c0,
        "props { (\"prop\", 'this is text') }",
        Doc {
            props: props!([("prop".to_string(), PropVal::Text("this is text".to_string()))]),
            ..Default::default()
        }
    );

    test!(
        po_props_tuple_text_c1,
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
        po_props_absorb_c0,
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
        po_props_absorb_c1,
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
        po_props_absorb_c2,
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
        po_props_absorb_c3,
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
        po_props_absorb_c4,
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
        po_props_overwrite_c0,
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
        po_props_overwrite_c1,
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
        po_props_overwrite_c2,
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
        po_props_trailing_comma,
        "props{ (\"a\", 0), }",
        Doc {
            props: props!([("a".to_string(), PropVal::Int(0))]),
            ..Default::default()
        }
    );

    test!(
        po_props_comment,
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
        po_tags_c0_f0,
        "tags{\"tag0\", \"tag1\"}",
        Doc {
            tags: hset!(["tag0", "tag1"]),
            ..Default::default()
        }
    );

    test!(
        po_tags_c0_f1,
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
        po_tags_c0_f2,
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
        po_tags_c1,
        "tags{}",
        Doc {
            ..Default::default()
        }
    );

    test!(
        po_tags_c2,
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
        po_tags_c3,
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
        po_tags_comment,
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

    test_par!(
        po_emphasis_c0_f0,
        "em{le, \"light emphasis\"}",
        Paragraph {
            items: vec![ParagraphItem::Em(Emphasis {
                strength: EmStrength::Light,
                etype: EmType::Emphasis,
                text: "light emphasis".to_string(),
                ..Default::default()
            })],
            ..Default::default()
        }

    );

    test_par!(
        po_emphasis_c0_f1,
        " em{
            le   ,
            \"light emphasis\"
        }",
        Paragraph {
            items: vec![ParagraphItem::Em(Emphasis {
                strength: EmStrength::Light,
                etype: EmType::Emphasis,
                text: "light emphasis".to_string(),
                ..Default::default()
            })],
            ..Default::default()
        }

    );

    test_par!(
        po_emphasis_c1,
        "em{me, \"medium emphasis\"}",
        Paragraph {
            items: vec![ParagraphItem::Em(Emphasis {
                strength: EmStrength::Medium,
                etype: EmType::Emphasis,
                text: "medium emphasis".to_string(),
                ..Default::default()
            })],
            ..Default::default()
        }

    );

    test_par!(
        po_emphasis_c2,
        "em{se, \"strong emphasis\"}",
        Paragraph {
            items: vec![ParagraphItem::Em(Emphasis {
                strength: EmStrength::Strong,
                etype: EmType::Emphasis,
                text: "strong emphasis".to_string(),
                ..Default::default()
            })],
            ..Default::default()
        }

    );

    test_par!(
        po_emphasis_c3,
        "em{ld, \"light deemphasis\"}",
        Paragraph {
            items: vec![ParagraphItem::Em(Emphasis {
                strength: EmStrength::Light,
                etype: EmType::Deemphasis,
                text: "light deemphasis".to_string(),
                ..Default::default()
            })],
            ..Default::default()
        }

    );

    test_par!(
        po_emphasis_c4,
        "em{md, \"medium deemphasis\"}",
        Paragraph {
            items: vec![ParagraphItem::Em(Emphasis {
                strength: EmStrength::Medium,
                etype: EmType::Deemphasis,
                text: "medium deemphasis".to_string(),
                ..Default::default()
            })],
            ..Default::default()
        }

    );

    test_par!(
        po_emphasis_c5,
        "em{sd, \"strong deemphasis\"}",
        Paragraph {
            items: vec![ParagraphItem::Em(Emphasis {
                strength: EmStrength::Strong,
                etype: EmType::Deemphasis,
                text: "strong deemphasis".to_string(),
                ..Default::default()
            })],
            ..Default::default()
        }

    );

    test_par!(
        po_emphasis_c6,
        "
        'This is a ',
        em{le, \"light\"},
        ' emphasis.',
        ",
        Paragraph {
            items: vec![
                ParagraphItem::Text("This is a ".to_string()),
                ParagraphItem::Em(Emphasis {
                    strength: EmStrength::Light,
                    etype: EmType::Emphasis,
                    text: "light".to_string(),
                    ..Default::default()
                }),
                ParagraphItem::Text(" emphasis.".to_string()),
            ],
            ..Default::default()
        }

    );

    test_par!(
        po_emphasis_c7,
        "
        em{
            le,
            \"light\",
            tags { \"a\", \"b\" },
        }
        ",
        Paragraph {
            items: vec![
                ParagraphItem::Em(Emphasis {
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

    test_par!(
        po_emphasis_c8,
        "
        em{
            le,
            \"light\",
            props { (\"a\", 0), (\"b\", 1) },
        }
        ",
        Paragraph {
            items: vec![
                ParagraphItem::Em(Emphasis {
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

    test_par!(
        po_emphasis_c9,
        "
        em{
            le,
            \"light\",
            tags { \"a\", \"b\" },
            props { (\"a\", 0), (\"b\", 1) },
        }
        ",
        Paragraph {
            items: vec![
                ParagraphItem::Em(Emphasis {
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

    test_par!(
        po_emphasis_comment,
        "/*c*/em/*c*/{/*c*/le/*c*/,/*c*/\"le\"/*c*/,/*c*/tags{}/*c*/,/*c*/props{}/*c*/}//c",
        Paragraph {
            items: vec![ParagraphItem::Em(Emphasis {
                strength: EmStrength::Light,
                etype: EmType::Emphasis,
                text: "le".to_string(),
                ..Default::default()
            })],
            ..Default::default()
        }

    );

    test_par!(
        po_code_c0_f0,
        "code { \"plain\", \"show\", '' }",
        Paragraph {
            items: vec![ParagraphItem::Code(Ok(CodeBlock {
                language: "plain".to_string(),
                ..Default::default()
            }))],
            ..Default::default()
        }
    );

    test_par!(
        po_code_c0_f1,
        "code
        {
    \"plain\",
            \"show\",
                ''}",
        Paragraph {
            items: vec![ParagraphItem::Code(Ok(CodeBlock {
                language: "plain".to_string(),
                ..Default::default()
            }))],
            ..Default::default()
        }
    );

    test_par!(
        po_code_c1,
        "code { \"plain\", \"runnable\", '' }",
        Paragraph {
            items: vec![ParagraphItem::Code(Ok(CodeBlock {
                language: "plain".to_string(),
                mode: CodeModeHint::Runnable,
                ..Default::default()
            }))],
            ..Default::default()
        }
    );

    test_par!(
        po_code_c2,
        "code { \"plain\", \"run\", '' }",
        Paragraph {
            items: vec![ParagraphItem::Code(Ok(CodeBlock {
                language: "plain".to_string(),
                mode: CodeModeHint::Run,
                ..Default::default()
            }))],
            ..Default::default()
        }
    );

    test_par!(
        po_code_c3,
        "code { \"plain\", \"replace\", '' }",
        Paragraph {
            items: vec![ParagraphItem::Code(Ok(CodeBlock {
                language: "plain".to_string(),
                mode: CodeModeHint::Replace,
                ..Default::default()
            }))],
            ..Default::default()
        }
    );

    test_par!(
        po_code_c4,
        "code { \"plain\", \"not a mode!\", '' }",
        Paragraph {
            items: vec![ParagraphItem::Code(Ok(CodeBlock {
                language: "plain".to_string(),
                ..Default::default()
            }))],
            ..Default::default()
        }
    );

    test_par!(
        po_code_c5,
        "code {
            \"plain\",
            \"show\",
            '
            this is code
            '
        }",
        Paragraph {
            items: vec![ParagraphItem::Code(Ok(CodeBlock {
                language: "plain".to_string(),
                code: "this is code".to_string(),
                ..Default::default()
            }))],
            ..Default::default()
        }
    );

    test_par!(
        po_code_c6,
        "code {
            \"plain\",
            \"show\",
            'this is code
            '
        }",
        Paragraph {
            items: vec![ParagraphItem::Code(Ok(CodeBlock {
                language: "plain".to_string(),
                code: "this is code".to_string(),
                ..Default::default()
            }))],
            ..Default::default()
        }
    );

    test_par!(
        po_code_c7,
        "code {
            \"plain\",
            \"show\",
            '
                this is code
            '
        }",
        Paragraph {
            items: vec![ParagraphItem::Code(Ok(CodeBlock {
                language: "plain".to_string(),
                code: "    this is code".to_string(),
                ..Default::default()
            }))],
            ..Default::default()
        }
    );

    test_par!(
        po_code_c8,
        "code {
            \"plain\",
            \"show\",
            '

                this is code
            '
        }",
        Paragraph {
            items: vec![ParagraphItem::Code(Ok(CodeBlock {
                language: "plain".to_string(),
                code: "\n    this is code".to_string(),
                ..Default::default()
            }))],
            ..Default::default()
        }
    );

    test_par!(
        po_code_c9,
        "code {
            \"plain\",
            \"show\",
            '
            this is code

            '
        }",
        Paragraph {
            items: vec![ParagraphItem::Code(Ok(CodeBlock {
                language: "plain".to_string(),
                code: "this is code\n".to_string(),
                ..Default::default()
            }))],
            ..Default::default()
        }
    );

    test_par!(
        po_code_c10,
        "code {
            \"plain\",
            \"show\",
            '

                this is code
                 more code

            '
        }",
        Paragraph {
            items: vec![ParagraphItem::Code(Ok(CodeBlock {
                language: "plain".to_string(),
                code: "\n    this is code\n     more code\n".to_string(),
                ..Default::default()
            }))],
            ..Default::default()
        }
    );

    test!(
        po_code_c11,
        "
        par {
            code {
                \"plain\",
                \"show\",
                '
               this is code
                '
            }
        }
        ",
        Doc {
            items: vec![
                DocItem::Paragraph(Paragraph {
                    items: vec![ParagraphItem::Code(Err(CodeIdentError))],
                    ..Default::default()
                })
            ],
            ..Default::default()
        }
    );

    test_par!(
        po_code_meta0,
        "
        code {
            \"plain\",
            \"show\",
            'this is code',
            tags { \"a\", \"b\" }
        }
        ",
        Paragraph {
            items: vec![ParagraphItem::Code(Ok(CodeBlock {
                language: "plain".to_string(),
                code: "this is code".to_string(),
                tags: hset!(["a", "b"]),
                ..Default::default()
            }))],
            ..Default::default()
        }
    );

    test_par!(
        po_code_meta1,
        "
        code {
            \"plain\",
            \"show\",
            'this is code',
            props { (\"a\", 0), (\"b\", 1), (\"a\", 2) }
        }
        ",
        Paragraph {
            items: vec![ParagraphItem::Code(Ok(CodeBlock {
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

    test_par!(
        po_code_meta2,
        "
        code {
            \"plain\",
            \"show\",
            'this is code',
            tags { \"a\", \"b\" },
            props { (\"a\", 0), (\"b\", 1), (\"a\", 2) }
        }
        ",
        Paragraph {
            items: vec![ParagraphItem::Code(Ok(CodeBlock {
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

    test_par!(
        po_code_trailing_comma0,
        "
        code {
            \"plain\",
            \"show\",
            'this is code',
        }
        ",
        Paragraph {
            items: vec![ParagraphItem::Code(Ok(CodeBlock {
                language: "plain".to_string(),
                code: "this is code".to_string(),
                ..Default::default()
            }))],
            ..Default::default()
        }
    );

    test_par!(
        po_code_trailing_comma1,
        "
        code {
            \"plain\",
            \"show\",
            'this is code',
            tags { \"a\", \"b\" },
            props { (\"a\", 0), (\"b\", 1), (\"a\", 2) },
        }
        ",
        Paragraph {
            items: vec![ParagraphItem::Code(Ok(CodeBlock {
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

    test_par!(
        po_code_comment,
        "/*c*/code/*c*/{//c
            /*c*/\"plain\"/*c*/,//c
            /*c*/\"show\"/*c*/,/*c*///c
            /*c*/'
                 // comment
                 '//c
        /*c*/}/*c*/",
        Paragraph {
            items: vec![ParagraphItem::Code(Ok(CodeBlock {
                language: "plain".to_string(),
                code: "// comment".to_string(),
                ..Default::default()
            }))],
            ..Default::default()
        }
    );

    test!(
        po_paragraph,
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
            list { il, par { 'item' } }
        },
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
                            items: vec![
                                Paragraph {
                                    items: vec![ParagraphItem::Text("item".to_string())],
                                    ..Default::default()
                                }
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
        po_paragraph_meta,
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
        po_paragraph_trailing_comma,
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
        po_paragraph_comment,
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
        po_heading_c0,
        "
        section {
            head { 0, \"h\" },
            par { 'p' },
        }
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
        po_heading_c1,
        "
        section {
            head { 1, \"h\" },
            par { 'p' },
        }
        ",
        Doc {
            items: vec![
                DocItem::Section(Section {
                    heading: Heading {
                        level: 1,
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
        po_heading_c2,
        "
        section {
            head { 256, \"h\" },
            par { 'p' },
        }
        ",
        Doc {
            items: vec![
                DocItem::Section(Section {
                    heading: Heading {
                        level: 255,
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
        po_heading_c3,
        "
        section {
            head { 9999999999999999999, \"h\" },
            par { 'p' },
        }
        ",
        Doc {
            items: vec![
                DocItem::Section(Section {
                    heading: Heading {
                        level: 255,
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
        po_heading_c4,
        "
        section {
            head { 0, \"hello\", em { md, \" world\" } },
            par { 'p' },
        }
        ",
        Doc {
            items: vec![
                DocItem::Section(Section {
                    heading: Heading {
                        level: 0,
                        items: vec![
                            HeadingItem::String("hello".to_string()),
                            HeadingItem::Em(Emphasis {
                                text: " world".to_string(),
                                etype: EmType::Deemphasis,
                                strength: EmStrength::Medium,
                                ..Default::default()
                            }),
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
        po_heading_meta0,
        "
        section {
            head { 1, tags { \"a\" }, tags{ \"b\" } },
            par { 'p' },
        }
        ",
        Doc {
            items: vec![
                DocItem::Section(Section {
                    heading: Heading {
                        level: 1,
                        items: vec![],
                        tags: hset!(["a", "b"]),
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
        po_heading_meta1,
        "
        section {
            head { 1, props { (\"a\", 0), (\"b\", 0) }, props{ (\"b\", 1) } },
            par { 'p' },
        }
        ",
        Doc {
            items: vec![
                DocItem::Section(Section {
                    heading: Heading {
                        level: 1,
                        items: vec![],
                        props: props!([
                            ("a".to_string(), PropVal::Int(0)),
                            ("b".to_string(), PropVal::Int(1))
                        ]),
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
        po_heading_meta2,
        "
        section {
            head { 1, props { (\"a\", 0), (\"b\", 0) }, tags { \"tag\" }, props{ (\"b\", 1) } },
            par { 'p' },
        }
        ",
        Doc {
            items: vec![
                DocItem::Section(Section {
                    heading: Heading {
                        level: 1,
                        items: vec![],
                        tags: hset!(["tag"]),
                        props: props!([
                            ("a".to_string(), PropVal::Int(0)),
                            ("b".to_string(), PropVal::Int(1))
                        ]),
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
        po_heading_comment,
        "
        section {
            /*c*/head /*c*/ {/*c*/0/*c*/,/**/\"a\"/*c*/,/*c*/\"b\"/*c*/},//c,
            par { 'p' },
        }
        ",
        Doc {
            items: vec![
                DocItem::Section(Section {
                    heading: Heading {
                        level: 0,
                        items: vec![
                            HeadingItem::String("a".to_string()),
                            HeadingItem::String("b".to_string()),
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

    test_par!(
        po_list_c0,
        "list { dl, par { 'test' } }",
        Paragraph {
            items: vec![
                ParagraphItem::List(List {
                    items: vec![
                        Paragraph {
                            items: vec![ParagraphItem::Text("test".to_string())],
                            ..Default::default()
                        }
                    ],
                    ltype: ListType::Distinct,
                    ..Default::default()
                })
            ],
            ..Default::default()
        }
    );

    test_par!(
        po_list_c0_trailing_comma,
        "list { dl, par { 'test', }, }",
        Paragraph {
            items: vec![
                ParagraphItem::List(List {
                    items: vec![
                        Paragraph {
                            items: vec![ParagraphItem::Text("test".to_string())],
                            ..Default::default()
                        }
                    ],
                    ltype: ListType::Distinct,
                    ..Default::default()
                })
            ],
            ..Default::default()
        }
    );

    test_par!(
        po_list_c1,
        "list { il, par { 'test' } }",
        Paragraph {
            items: vec![
                ParagraphItem::List(List {
                    items: vec![
                        Paragraph {
                            items: vec![ParagraphItem::Text("test".to_string())],
                            ..Default::default()
                        }
                    ],
                    ltype: ListType::Identical,
                    ..Default::default()
                })
            ],
            ..Default::default()
        }
    );

    test_par!(
        po_list_c2,
        "list { cl, par { 'test' } }",
        Paragraph {
            items: vec![
                ParagraphItem::List(List {
                    items: vec![
                        Paragraph {
                            items: vec![ParagraphItem::Text("test".to_string())],
                            ..Default::default()
                        }
                    ],
                    ltype: ListType::Checked,
                    ..Default::default()
                })
            ],
            ..Default::default()
        }
    );

    test_par!(
        po_list_c3,
        "list { cl, par { 'a0', 'a1' }, par { 'b0', 'b1' }, par { 'c0', 'c1' } }",
        Paragraph {
            items: vec![
                ParagraphItem::List(List {
                    items: vec![
                        Paragraph {
                            items: vec![
                                ParagraphItem::Text("a0".to_string()),
                                ParagraphItem::Text("a1".to_string()),
                            ],
                            ..Default::default()
                        },
                        Paragraph {
                            items: vec![
                                ParagraphItem::Text("b0".to_string()),
                                ParagraphItem::Text("b1".to_string()),
                            ],
                            ..Default::default()
                        },
                        Paragraph {
                            items: vec![
                                ParagraphItem::Text("c0".to_string()),
                                ParagraphItem::Text("c1".to_string()),
                            ],
                            ..Default::default()
                        },
                    ],
                    ltype: ListType::Checked,
                    ..Default::default()
                })
            ],
            ..Default::default()
        }
    );

    test_par!(
        po_list_c4,
        "list { il, par { '0', list { dl, par { '1', list { cl, par { '2' } } } } } }",
        Paragraph {
            items: vec![
                ParagraphItem::List(List {
                    ltype: ListType::Identical,
                    items: vec![Paragraph {
                        items: vec![
                            ParagraphItem::Text("0".to_string()),
                            ParagraphItem::List(List {
                                ltype: ListType::Distinct,
                                items: vec![Paragraph {
                                    items: vec![
                                        ParagraphItem::Text("1".to_string()),
                                        ParagraphItem::List(List {
                                            ltype: ListType::Checked,
                                            items: vec![Paragraph {
                                                items: vec![
                                                    ParagraphItem::Text("2".to_string()),
                                                ],
                                                ..Default::default()
                                            }],
                                            ..Default::default()
                                        }),
                                    ],
                                    ..Default::default()
                                }],
                                ..Default::default()
                            }),
                        ],
                        ..Default::default()
                    }],
                    ..Default::default()
                })
            ],
            ..Default::default()
        }
    );

    test_par!(
        po_list_meta_c0,
        "
        list {
            il,
            par {'item', tags { \"tag\" }, props { (\"prop\", 0) } },
            tags { \"a\", \"b\" },
        }
        ",
        Paragraph {
            items: vec![ParagraphItem::List(List {
                ltype: ListType::Identical,
                items: vec![
                    Paragraph {
                        items: vec![ParagraphItem::Text("item".to_string())],
                        tags: hset!(["tag"]),
                        props: props!([("prop".to_string(), PropVal::Int(0))]),
                    }
                ],
                tags: hset!(["a", "b"]),
                ..Default::default()
            })],
            ..Default::default()
        }
    );

    test_par!(
        po_list_meta_c1,
        "
        list {
            il,
            par {'item', tags { \"tag\" }, props { (\"prop\", 0) } },
            props { (\"a\", 0), (\"b\", 1) },
        }
        ",
        Paragraph {
            items: vec![ParagraphItem::List(List {
                ltype: ListType::Identical,
                items: vec![
                    Paragraph {
                        items: vec![ParagraphItem::Text("item".to_string())],
                        tags: hset!(["tag"]),
                        props: props!([("prop".to_string(), PropVal::Int(0))]),
                    }
                ],
                props: props!([
                    ("a".to_string(), PropVal::Int(0)),
                    ("b".to_string(), PropVal::Int(1)),
                ]),
                ..Default::default()
            })],
            ..Default::default()
        }
    );

    test_par!(
        po_list_meta_c2,
        "
        list {
            il,
            par {'item', tags { \"tag\" }, props { (\"prop\", 0) } },
            tags { \"a\", \"b\" },
            props { (\"a\", 0), (\"b\", 1) },
        }
        ",
        Paragraph {
            items: vec![ParagraphItem::List(List {
                ltype: ListType::Identical,
                items: vec![
                    Paragraph {
                        items: vec![ParagraphItem::Text("item".to_string())],
                        tags: hset!(["tag"]),
                        props: props!([("prop".to_string(), PropVal::Int(0))]),
                    }
                ],
                tags: hset!(["a", "b"]),
                props: props!([
                    ("a".to_string(), PropVal::Int(0)),
                    ("b".to_string(), PropVal::Int(1)),
                ]),
                ..Default::default()
            })],
            ..Default::default()
        }
    );

    test_par!(
        po_list_comment,
        "/*c*/list/*c*/{/*c*/dl/*c*/,/*c*/par/*c*/{/*c*/'a'/*c*/,/*c*/'b'/*c*/}/*c*/}//c",
        Paragraph {
            items: vec![
                ParagraphItem::List(List {
                    items: vec![
                        Paragraph {
                            items: vec![
                                ParagraphItem::Text("a".to_string()),
                                ParagraphItem::Text("b".to_string()),
                            ],
                            ..Default::default()
                        }
                    ],
                    ltype: ListType::Distinct,
                    ..Default::default()
                })
            ],
            ..Default::default()
        }
    );

    test!(
        po_section_c0_f0,
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
        po_section_c0_trailing_comma,
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
        po_section_c1,
        "
        section {
            head { 0, \"heading\" },
            section { head { 0, \"heading\" }, par { 'paragraph' } },
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
        po_section_c2,
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
                                level: 2,
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
        po_section_c3,
        "
        section {
            head { 2, \"heading\" },
            section { head { 0, \"heading\" }, par { 'paragraph' } },
        }
        ",
        Doc {
            items: vec![
                DocItem::Section(Section {
                    heading: Heading {
                        level: 2,
                        items: vec![
                            HeadingItem::String("heading".to_string()),
                        ],
                        ..Default::default()
                    },
                    items: vec![
                        SectionItem::Section(Section {
                            heading: Heading {
                                level: 3,
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
        po_section_c4,
        "
        section {
            head { 2, \"heading\" },
            section {
                head { 1, \"heading\" },
                section {
                    head { 0, \"heading\" },
                    par { 'paragraph' },
                },
            },
        }
        ",
        Doc {
            items: vec![
                DocItem::Section(Section {
                    heading: Heading {
                        level: 2,
                        items: vec![
                            HeadingItem::String("heading".to_string()),
                        ],
                        ..Default::default()
                    },
                    items: vec![
                        SectionItem::Section(Section {
                            heading: Heading {
                                level: 4,
                                items: vec![
                                    HeadingItem::String("heading".to_string()),
                                ],
                                ..Default::default()
                            },
                            items: vec![
                                SectionItem::Section(Section {
                                    heading: Heading {
                                        level: 5,
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
                                        }),
                                    ],
                                    ..Default::default()
                                })
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
        po_section_meta,
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
        po_section_comment,
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

    test_par!(
        po_link_c0,
        "link { \"url\", \"link\" }",
        Paragraph {
            items: vec![
                ParagraphItem::Link(Link {
                    url: "url".to_string(),
                    items: vec![LinkItem::String("link".to_string())],
                    ..Default::default()
                })
            ],
            ..Default::default()
        }
    );

    test_par!(
        po_link_c0_comment,
        "/*c*/link/*c*/{/*c*/\"url\"/*c*/,/*c*/\"link\"/*c*/}//c",
        Paragraph {
            items: vec![
                ParagraphItem::Link(Link {
                    url: "url".to_string(),
                    items: vec![LinkItem::String("link".to_string())],
                    ..Default::default()
                })
            ],
            ..Default::default()
        }
    );

    test_par!(
        po_link_c1,
        "link { \"url\", em { le, \"em\" }, \"string\" }",
        Paragraph {
            items: vec![
                ParagraphItem::Link(Link {
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

    test_par!(
        po_link_meta0,
        "link { \"url\", \"link\", tags { \"tag\" } }",
        Paragraph {
            items: vec![
                ParagraphItem::Link(Link {
                    url: "url".to_string(),
                    items: vec![LinkItem::String("link".to_string())],
                    tags: hset!(["tag"]),
                    ..Default::default()
                })
            ],
            ..Default::default()
        }
    );

    test_par!(
        po_link_meta1,
        "link { \"url\", \"link\", props { (\"prop\", 0) } }",
        Paragraph {
            items: vec![
                ParagraphItem::Link(Link {
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

    test_par!(
        po_link_meta2,
        "link { \"url\", \"link\", tags { \"tag\" }, props { (\"prop\", 0) } }",
        Paragraph {
            items: vec![
                ParagraphItem::Link(Link {
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
        po_nav_c0,
        "
        nav {
            link { \"urla\", \"linka\" },
            link { \"urlb\", \"linkb\" }
        }
        ",
        Doc {
            items: vec![
                DocItem::Nav(Nav {
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
                })
            ],
            ..Default::default()
        }
    );

    test!(
        po_nav_c0_comment,
        "
        /*c*/nav/*c*/{//c
            /*c*/link/*c*/{/*c*/\"urla\"/*c*/,/*c*/\"linka\"/*c*/}/*c*/,//c
            /*c*/link/*c*/{/*c*/\"urlb\"/*c*/,/*c*/\"linkb\"/*c*/}/*c*/,//c
        /*c*/}/*c*/
        ",
        Doc {
            items: vec![
                DocItem::Nav(Nav {
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
                })
            ],
            ..Default::default()
        }
    );

    test!(
        po_nav_c1,
        "
        nav {
            nav {
                \"desca\",
                link { \"urla\", \"linka\" }
            },
            nav {
                \"descb\",
                link { \"urlb\", \"linkb\" }
            }
        }
        ",
        Doc {
            items: vec![
                DocItem::Nav(Nav {
                    subs: vec![
                        Nav {
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
                        Nav {
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
                    ..Default::default()
                })
            ],
            ..Default::default()
        }
    );

    test!(
        po_nav_c1_trailing_comma,
        "
        nav {
            nav {
                \"desca\",
                link { \"urla\", \"linka\" },
            },
            nav {
                \"descb\",
                link { \"urlb\", \"linkb\" },
            },
        },
        ",
        Doc {
            items: vec![
                DocItem::Nav(Nav {
                    subs: vec![
                        Nav {
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
                        Nav {
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
                    ..Default::default()
                })
            ],
            ..Default::default()
        }
    );

    test!(
        po_nav_c2,
        "
        nav {
            nav {
                \"desca\",
                link { \"urla\", \"linka\" }
            },
            nav {
                \"descb\",
                link { \"urlb\", \"linkb\" }
            },
            link { \"urlc\", \"linkc\" }
        }
        ",
        Doc {
            items: vec![
                DocItem::Nav(Nav {
                    subs: vec![
                        Nav {
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
                        Nav {
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
                })
            ],
            ..Default::default()
        }
    );

    test!(
        po_nav_meta0,
        "
        nav {
            link { \"urla\", \"linka\" },
            tags { \"tag\" }
        }
        ",
        Doc {
            items: vec![
                DocItem::Nav(Nav {
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
                })
            ],
            ..Default::default()
        }
    );

    test!(
        po_nav_meta1,
        "
        nav {
            link { \"urla\", \"linka\" },
            props { (\"prop\", 0) }
        }
        ",
        Doc {
            items: vec![
                DocItem::Nav(Nav {
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
                })
            ],
            ..Default::default()
        }
    );

    test!(
        po_nav_meta2,
        "
        nav {
            link { \"urla\", \"linka\" },
            tags { \"tag\" },
            props { (\"prop\", 0) }
        }
        ",
        Doc {
            items: vec![
                DocItem::Nav(Nav {
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
                })
            ],
            ..Default::default()
        }
    );

    test_par!(
        po_table_c0,
        "
        table {
            trow {
                par { 'text' },
            },
        },
        ",
        Paragraph {
            items: vec![
                ParagraphItem::Table(Table {
                    rows: vec![
                        TableRow {
                            items: vec![
                                Paragraph {
                                    items: vec![
                                        ParagraphItem::Text("text".to_string()),
                                    ],
                                    ..Default::default()
                                },
                            ],
                            ..Default::default()
                        },
                    ],
                    ..Default::default()
                }),
            ],
            ..Default::default()
        }
    );

    test_par!(
        po_table_c0_comment,
        "
        /*c*/table/*c*/{//c
            /*c*/trow/*c*/{//c
                /*c*/par/*c*/{/*c*/'text'/*c*/}/*c*/,//c
            /*c*/}/*c*/,//c
        /*c*/}/*c*/,//c
        ",
        Paragraph {
            items: vec![
                ParagraphItem::Table(Table {
                    rows: vec![
                        TableRow {
                            items: vec![
                                Paragraph {
                                    items: vec![
                                        ParagraphItem::Text("text".to_string()),
                                    ],
                                    ..Default::default()
                                },
                            ],
                            ..Default::default()
                        },
                    ],
                    ..Default::default()
                }),
            ],
            ..Default::default()
        }
    );

    test_par!(
        po_table_c1,
        "
        table {
            trow {
                par { 'text' },
                par { 'text' },
            },
        },
        ",
        Paragraph {
            items: vec![
                ParagraphItem::Table(Table {
                    rows: vec![
                        TableRow {
                            items: vec![
                                Paragraph {
                                    items: vec![
                                        ParagraphItem::Text("text".to_string()),
                                    ],
                                    ..Default::default()
                                },
                                Paragraph {
                                    items: vec![
                                        ParagraphItem::Text("text".to_string()),
                                    ],
                                    ..Default::default()
                                },
                            ],
                            ..Default::default()
                        },
                    ],
                    ..Default::default()
                }),
            ],
            ..Default::default()
        }
    );

    test_par!(
        po_table_c2,
        "
        table {
            throw {
                par { 'text' },
                par { 'text' },
            },
        },
        ",
        Paragraph {
            items: vec![
                ParagraphItem::Table(Table {
                    rows: vec![
                        TableRow {
                            is_header: true,
                            items: vec![
                                Paragraph {
                                    items: vec![
                                        ParagraphItem::Text("text".to_string()),
                                    ],
                                    ..Default::default()
                                },
                                Paragraph {
                                    items: vec![
                                        ParagraphItem::Text("text".to_string()),
                                    ],
                                    ..Default::default()
                                },
                            ],
                            ..Default::default()
                        },
                    ],
                    ..Default::default()
                }),
            ],
            ..Default::default()
        }
    );

    test_par!(
        po_table_c3,
        "
        table {
            throw {
                par { 'text' },
                par { 'text' },
            },
            trow {
                par { 'text' },
                par { 'text' },
            },
        },
        ",
        Paragraph {
            items: vec![
                ParagraphItem::Table(Table {
                    rows: vec![
                        TableRow {
                            is_header: true,
                            items: vec![
                                Paragraph {
                                    items: vec![
                                        ParagraphItem::Text("text".to_string()),
                                    ],
                                    ..Default::default()
                                },
                                Paragraph {
                                    items: vec![
                                        ParagraphItem::Text("text".to_string()),
                                    ],
                                    ..Default::default()
                                },
                            ],
                            ..Default::default()
                        },
                        TableRow {
                            items: vec![
                                Paragraph {
                                    items: vec![
                                        ParagraphItem::Text("text".to_string()),
                                    ],
                                    ..Default::default()
                                },
                                Paragraph {
                                    items: vec![
                                        ParagraphItem::Text("text".to_string()),
                                    ],
                                    ..Default::default()
                                },
                            ],
                            ..Default::default()
                        },
                    ],
                    ..Default::default()
                }),
            ],
            ..Default::default()
        }
    );

    test_par!(
        po_table_c4,
        "
        table {
            tags { \"a\" },
            props { (\"a\", 0) },
            throw {
                tags { \"a\" },
                props { (\"a\", 0) },
                par { 'text' },
                par { 'text' },
                tags { \"b\" },
                props { (\"b\", 0) },
            },
            tags { \"b\" },
            props { (\"b\", 0) },
            trow {
                par { 'text' },
            },
        },
        ",
        Paragraph {
            items: vec![
                ParagraphItem::Table(Table {
                    rows: vec![
                        TableRow {
                            is_header: true,
                            items: vec![
                                Paragraph {
                                    items: vec![
                                        ParagraphItem::Text("text".to_string()),
                                    ],
                                    ..Default::default()
                                },
                                Paragraph {
                                    items: vec![
                                        ParagraphItem::Text("text".to_string()),
                                    ],
                                    ..Default::default()
                                },
                            ],
                            tags: hset!(["a", "b"]),
                            props: props!([
                                ("a".to_string(), PropVal::Int(0)),
                                ("b".to_string(), PropVal::Int(0)),
                            ]),
                        },
                        TableRow {
                            items: vec![
                                Paragraph {
                                    items: vec![
                                        ParagraphItem::Text("text".to_string()),
                                    ],
                                    ..Default::default()
                                },
                            ],
                            ..Default::default()
                        },
                    ],
                    tags: hset!(["a", "b"]),
                    props: props!([
                        ("a".to_string(), PropVal::Int(0)),
                        ("b".to_string(), PropVal::Int(0)),
                    ]),
                    ..Default::default()
                }),
            ],
            ..Default::default()
        }
    );

    test_par!(
        po_table_c5,
        "
        table {
            throw {
                par {
                    table {
                        trow {
                            par {
                                'text'
                            },
                        },
                    },
                },
            },
        },
        ",
        Paragraph {
            items: vec![
                ParagraphItem::Table(Table {
                    rows: vec![
                        TableRow {
                            is_header: true,
                            items: vec![
                                Paragraph {
                                    items: vec![
                                        ParagraphItem::Table(Table {
                                            rows: vec![
                                                TableRow {
                                                    items: vec![
                                                        Paragraph {
                                                            items: vec![
                                                                ParagraphItem::Text(
                                                                    "text".to_string()
                                                                ),
                                                            ],
                                                            ..Default::default()
                                                        },
                                                    ],
                                                    ..Default::default()
                                                },
                                            ],
                                            ..Default::default()
                                        }),
                                    ],
                                    ..Default::default()
                                },
                            ],
                            ..Default::default()
                        },
                    ],
                    ..Default::default()
                }),
            ],
            ..Default::default()
        }
    );
}
