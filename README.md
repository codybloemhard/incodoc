# incodoc

Incorporeal document format.
- document type that doesn't dictate appearance
- should be able to function like a static web page
- the consumer chooses how to render the document
- not meant to write directly but should be human readable and writable
- meant as a middle layer
  - documents written in formats easy for humans
  - converted to incodoc
  - served to consumers as an incodoc or incodoc compatible document
  - either converted or rendered to best serve the consumer
- alternative to static site generators
  - emphasis on the consumer needs

## Quick example

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

## Why

### Situation

You browse the web. Many things are mostly text based with simple media inserted:

- blogs
- documentation
- government information
- books
- personal websites
- emails

By only supporting a few basic things you get most of the way:

- text
- links
- emphasised text
- headings
- lists
- simple structure
  - paragraphs
  - sections
- images

Most of these come with their own aesthetics:

- fonts, font sizes
- colour themes
- margins, padding and other spacing

Most also dictate how you interact with them:

- how links behave when you click
- how the menu works (eg. auto expand sub menu vs click to expand)

### Problem

All of these variations are not at all serving the consumer.
The documents are often designed to be internally consistent.
But the user likely experiences multiple of these documents at once.
All with different authors and designs.
Simply, it is a mess with no consistency.
The consumer may wish to render all these documents in a consistent way but this is very difficult.

Slightly different interactions slow down the user.
This websites menu works like this, that one like that.
Even though they all do exactly the same.
It takes a second to adjust.
The user would like to consistently interact with similar constructs.

The user may be severely impeded by the design but could otherwise consume the document properly:

- a font that is hard to read for the consumer
- a colour scheme may be difficult for a colour blind consumer
- images that download and show by default on a slow connection
- accessibility tools may struggle when design and content are entangled

In the end, the spaghetti like entanglement of content and style/design limits the consumer.
The consumer it limited in reading the document in a way that serves them best.
The author has forced a particular way of consuming the document onto the consumer.

### Existing solutions

Solutions are half baked or buggy work arounds.

- Dark Reader browser extension can change the colour theme of websites
  - it works well most of the time but sometimes very poorly
  - it is an uphill battle for the extension devs and end users
  - sites that don't play along nicely are a real pain to use
- you can force a font in your browser
  - text is rendered in the font of your choosing
  - this breaks a surprising amount of websites
    - seemingly simple sites don't render correctly anymore
    - previously invisibly text shows up and makes the site unreadable
- you can try inverting the render of pdf files
  - mostly gets you a dark theme
  - breaks with images
  - doesn't work when authors refer to colours in the document

It is difficult to change small design elements to serve the consumer more effectively.

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

