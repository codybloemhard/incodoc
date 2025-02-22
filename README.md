# incodoc

Incorporeal document format.
- document type that doesn't dictate appearance
- should be able to function like a static web page
- the consumer chooses how to render the document
- not meant to write directly but should be human readable and writable
- meant as a middle layer
  - documents written in formats easy for humans
  - converted to incodoc
  - served to consumers as an incodoc
  - either converted or rendered to best serve the consumer
- alternative to static site generators
  - emphasis on the consumer needs

## Quick example.

``` incodoc
props {
    ("date", 9999/12/31),
    ("language", "en"),
},
nav {
    snav {
        "top level",
        link {
            "/home",
            "home",
        },
        link {
            "/about",
            "about",
        },
        link {
            "/now",
            "now",
        },
        link {
            "/blog/index",
            "blog",
        },
    },
    snav {
        "other articles",
        link {
            "./cheese",
            "I like cheese",
        },
        link {
            "./sosig",
            "Sosig is happiness",
        },
    },
},
head {
    0,
    "very important",
},
par {
    'One must see this image.',
    link {
        "image.png",
        "very important image",
        props {
            ("bg-text", 'Extremely important image.'),
        },
    },
    'Also this one.',
    link {
        "website.com/image",
        "another important image",
        props {
            ("type-hint", 'image'),
        },
    },
    'For further questions see ',
    link {
        "#questions",
        "questions",
    },
    '.',
},
head {
    1,
    "questions",
    tags {
        "#questions",
    },
},
par {
    'Why is this important?
Because it is.
For even further questions email me at ',
    link {
        "email@address.com",
        "email@address.com",
    },
    '.',
},
par {
    'This is will not compile:',
    code {
        "rust",
        "show",
        '
            let mut x = 0;
            let y = &mut x;
            let z = &mut x;
            *y = 1;
        ',
    },
    'Your viewer may try to run it, only if they wants to.
This is a piece of code that suggest to be ran and its result inserted into this document.
Only if you want to.',
    code {
        "typst-formula",
        "replace",
        '
            x -> y
        ',
    },
},
par {
    'Copyright (c) 1337 me',
    props {
        ("type", "footer"),
    },
},
```

## Existing document formats

Could be HTML but in practice it is deeply involved with style.
Markdown is an incorporeal document.
The renderer, thus the user, chooses the font, colours, layout, etc.
Markdown is a successful incorporeal document.

## License

Code under MIT, see `LICENSE`.
This readme under CC BY 4.0:

This work © 2024 by Cody Bloemhard is licensed under CC BY 4.0.
To view a copy of this license, visit https://creativecommons.org/licenses/by/4.0/

