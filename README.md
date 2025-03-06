# incodoc

An incorporeal document format.

- document type that doesn't dictate appearance
- should be able to function like a static web page
- the consumer decides how to render the document
- not meant to read or write directly but should be human readable and writeable
- meant as a intermediate layer or final document
  - documents written in formats easy for humans
  - served to consumers as an incodoc or incodoc compatible document
  - either converted or rendered to best serve the consumer
- alternative to static site generators
  - emphasis on the consumer needs

In this README:

- Quick example
- Usage
- Why
  - Situation
  - Problem
  - Existing solutions
  - Existing document formats
    - Markdown
    - HTML
    - PDF
  - Name
- License

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

## Usage

### General

Incodoc is a  document type that doesn't encode how the document is supposed to look.
The consumer decides how to render the document.
It is not meant to be read or written directly but it is human readable and writeable.
Can be used directly or as an intermediate layer.

#### Direct usecase:

0. a document is written in a format easily readable and writeable for humans
1. the document is converted to incodoc
2. the incodoc is served
3. the consumer retrieves the incodoc
4. the consumer's document viewer/browser renders the incodoc as specified by the consumer 
  - either incodoc is rendered directly by the viewer/browser
  - or incodoc is converted behind the scenes by the viewer/browser and then displayed

Example: markdown -> incodoc -> HTML + CSS

0. the author writes their blog in markdown
1. the author converts it to incodoc
2. the author hosts their blog as incodoc
3. the consumer retrieves an article as an incodoc with their browser
4. the browser converts the incodoc to HTML + CSS as specified by the user config
5. the browser renders the HTML + CSS to be consumed

#### Indirect usecase:

0. a document is served in format that is not incodoc
1. the consumer retrieves the document
2. the document is converted to incodoc
3. the incodoc is rendered as specified by the consumer

Example: HTML + CSS + JS -> incodoc -> ANSI

0. a document is served as a HTML + CSS + JS webpage
1. the consumer retrieves the document with a terminal browser
2. the document is converted to incodoc as well possible converting/ignoring style
3. the incodoc is rendered to ANSI as specified by the user config
4. the content is outputted through stdout to be consumed

#### As alternative to SSG's

Usual static site generator (SSG) workflow:

0. the author writes a site in some markdown flavour
1. the author defines a style/theme
2. the SSG converts the markdown to HTML + CSS + JS with given theme
3. HTML + CSS + JS is served
4. HTML + CSS + JS is retrieved and rendered in browser
5. consumer required to:
  - consume content as styled by author
  - use a workaround like the indirect usecase above

By publishing as incodoc:

0. the author writes a site in
  - an incodoc markdown flavour
  - an incodoc compatible document format like plain markdown
1. the site is hosted as
  - the markdown converted to incodoc
  - just the markdown
2. the consumer retrieves the documents
3. the consumer's browser/viewer either
  - renders the incodoc
  - converts the markdown to incodoc and renders it

This way the author does not need to bother with style.
And the user can consume the document as desired or needed with their own theme/style/aesthetics/accessibility features.

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
The consumer is limited in reading the document in a way that serves them best.
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
- Firefox reader view
  - strips pages of many corporeal elements
  - allows consumers to choose font and colour scheme
  - has text-to-speech feature
  - could have more customisation
  - no document format seems to be behind it
  - <https://support.mozilla.org/en-US/kb/firefox-reader-view-clutter-free-web-pages>
  - <https://blog.mozilla.org/en/products/firefox/reader-view/>

It is difficult to change small design elements to serve the consumer more effectively.

### Existing document formats

When shortcomings or weak points of other formats are being discussed, it is taken to be relative
to the goal that incodoc has.
These design decisions often do make sense for the format's own goals.

#### Markdown

Markdown enjoys much use.
It is a mostly incorporeal document format.
Some issues:

- line breaks
  - may or may not be incorporeal depending on how you look at it
- bold and italic
  - corporeal but can be easily interpreted as incodoc's light, medium and heavy emphasis
- horizontal rules
  - may or may not be incorporeal depending on how you look at it
- HTML
  - usually used for corporeal elements like font and colour

See: <https://www.markdownguide.org/basic-syntax/>

Additional shortcomings:

- no meta
- no navigation
- bit vague
  - markdown vs commonmark
  - whitespace dependent

Strong points:

- mostly incorporeal
- wide adoption
- very human readable and writeable
- maps to incodoc with only small issues

Markdown is important to incododoc.
It will be the first document format convertible to incodoc.
An incodoc flavoured markdown is planned.

#### HTML

HTML in practice is deeply involved with style.
In principle you can use it incorporeally by using only incorporeal elements.
In practice HTML almost never goes without accompanying CSS.
The structure of the document usually is dictated by style.
Often `div` and `span` are just there to separate out parts of the document so they can be assigned
different style elements.
For my own very simple webpage, even I used `span` like this.
Anyone who ever tried to make a simple HTML + CSS webpage has most likely experienced this: having
to change the HTML to get it styled right.
But the document content was not changed.
This shows HTML encodes style in practice and not just content.
While we could use HTML as an incorporeal document, it would not be wise.
Upon receiving a HTML document, it may or may not be incorporeal.
It is desirable for the user to know whether they are dealing with incorporeal content or not before
rendering it.

A breakdown of the incorporeality of some HTML elements:

Incorporeal: `html`, `head`, `body`, `title`, `h1`..`h6`, `p`, `em`, `blockquote`, `q`, `abbr`, `address`, `cite`, `table`, `ul`, `ol`, `li`, `dl`, `dt`, `dd`

Mostly incorporeal: `a`, `img`

Maybe incorporeal: `br`, `hr`, `pre`, `sub`, `sup`

Corporeal with easy conversion: `i`, `mark`

Incorporeal: `small`, `del`, `ins`, `bdo`

Non document functionality: `button`

It is desirable to both convert (HTML + CSS) to incodoc and render incodoc to (HTML + CSS).
Parsing HTML + CSS and stripping it down as best as we can will allow consumers to consume as much
content as possible through incodoc, and thus via their preferred aesthetics and workflow which
they have control over.
Exporting incodoc as HTML + CSS allows the user to consume content in their browser, presented in a
way that they might prefer over the original presentation.
The (HTML + CSS) -> incodoc -> (HTML + CSS) pipeline can be of utility as well.

See: <https://www.w3schools.com/tags/>

#### PDF

PDF is one of the most used document formats.
A quick look at the Wikipedia article shows it is heavily corporeal in nature.
Based on PostScript, a PDF file is almost more like an application than it is a document.
It controls content, style and interaction, this in a very similar way to HTML + CSS + JS.

See: <https://en.wikipedia.org/wiki/PDF>

#### Org Mode

Org is a plain text file format.
It is a markup language similar to markdown (with different syntax).
It is mostly incorporeal, with small issues similar to markdown.
It does feature metadata capabilities.
There are some advanced features like tables.
It is not just a simple document format that encodes some content.
From the features overview it is clear that you are supposed to work directly in an Org document,
almost like some sort of application.
Mostly used in Emacs along with its scripting abilities.
A subset of Org could be used as an incodoc like document.
If no advanced and Emacs specific features are used, it should be relatively straight forward to
convert to incodoc.

See:
- <https://orgmode.org/features.html>
- <https://orgmode.org/quickstart.html>

### Name

Why does incodoc have the name it does?
Incodoc is short for "incorporeal document" which is a bit of a mouth full.

I wanted a single word to describe the document format.
I considered negative descriptions:

- styleless
  - "without a particular style"
    - <https://dictionary.cambridge.org/dictionary/english/styleless>
  - Referring to stylesheets. However it is quite vague as style doesn't have to refer to appearance.

- formless
  - "without clear shape or structure"
    - <https://dictionary.cambridge.org/dictionary/english/formless>
- "without clear shape" is accurate but we do have structure.

These negative descriptions are also not very catchy.
Then I combed through some words and found:

- ethereal
  - "very light and delicate, especially in a way that does not seem to come from the real, physical world"
    - https://dictionary.cambridge.org/dictionary/english/ethereal
  - Another play on not being physical but this time like it's not real.

- spectral
  - "coming from or seeming to be the spirit of a dead person"
    - https://dictionary.cambridge.org/dictionary/english/spectral
  - Yet another version of not being physical but now having to do with the dead.

Catchy but seem to imply some things that are not true or relevant of the document format.
The closest word I could find:

- incorporeal
  - "not having a physical body but a spiritual form"
    - https://dictionary.cambridge.org/dictionary/english/incorporeal
  - Describes the document as not having a physical body (the consumer decides what it looks like).
  - Describes the document as having form (the document has abstract/semantic structure (eg. paragraphs and sections))

I consider incorporeal document formats a category of document formats.
There may be other document formats that are incorporeal.
Some document formats may be more incorporeal in spirit than others.

## License

Code under MIT, see `LICENSE`.
This readme under CC BY 4.0:

This work © 2024 by Cody Bloemhard is licensed under CC BY 4.0.
To view a copy of this license, visit https://creativecommons.org/licenses/by/4.0/

