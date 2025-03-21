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
- alternative/addition to static site generators
  - emphasis on the consumer needs

incodoc is a work in progress.

In this README:

- [Quick example](#quick-example)
- [What](#what)
  - [General](#general)
  - [Incorporeality](#incorporeality)
- [How](#how)
  - [Usage](#usage)
  - [UX possibilities](#ux-possibilities)
- [Why](#why)
  - [Situation](#situation)
  - [Problem](#problem)
  - [Solution](#solution)
  - [Existing solutions](#existing-solutions)
  - [Existing document formats](#existing-document-formats)
    - [Markdown](#markdown)
    - [HTML](#html)
    - [PDF](#pdf)
    - [Org Mode](#org-mode)
- [Extra](#extra)
  - [Name](#name)
  - ["Consumer"](#"consumer")
- [License](#license)

## Projects

[md-to-incodoc](https://github.com/codybloemhard/md-to-incodoc)
Parse markdown and yield an incodoc data structure.

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
section {
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
    section {
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
},
par {
    'Copyright (c) 1337 me',
    props {
        ("type", "footer"),
    },
},
```

## What

### General

Incodoc is a document format that doesn't encode how the document is supposed to look.
The consumer decides how to render the document.
The idea is that you only encode what the content is and how it is structured.
What the final document looks like is up to the consumer.
It aims to serve the consumer first and foremost.
Incodoc is as much an ideology as a document format.
Documents should be as incorporeal as possible.
Any unnecessary corporeality is seen as a restriction, suppression of consumer freedom.

### Incorporeality

Incorporeal: "not having a physical body but a spiritual form",
as per <https://dictionary.cambridge.org/dictionary/english/incorporeal> (March 2025).
Describes the document as not having a physical body as the consumer decides what it looks like.
Yet describes the document as having form.
This refers to abstract form and structure like sections, paragraphs, lists, etc.
For motivation on the choice of words see the "Name" section.

Some incorporeal elements a document can have:

- text
- list
- paragraph
- section
- heading
- meta data

Some corporeal elements often encoded in many document formats:

- font
  - specific font
  - font size
  - font justification
- colour
  - colour theme
  - assigning certain colours to certain elements (e.g. links, headings)
- spacing
  - line height
  - word spacing
  - page margins
  - heading margins

Incorporeal documents try to encode the important bits while discarding the superfluous
style details leaving those to be decided by the consumer.
Incorporeality is a spectrum.
Some document formats allow for more incorporeality than others.
Some documents use more incorporeal elements than others.
The aim is to reduce corporeality in documents as much as possible and give power to the consumer.
While writing a document, minimising the corporeal elements maximises accessibility and freedom.

## How

### Usage

Incodoc is not meant to be read or written directly but it is human readable and writeable.
It can be used directly or as an intermediate layer.

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

By publishing incodoc alongside regular SSG output:

0. the author writes a site in
  - a format suitable for the SSG
  - possibly an incodoc compatible format if the SSG can take it
1. the SSG outputs both
  - regular corporeal output
  - incodoc compatible output
2. both versions are hosted side by side
3. the consumer retrieves either version

Regular web documents are served to regular consumers.
Incodoc version is served to those who prefer it.

### UX possibilities

When the consumer decides how to render documents, a much better experience can be crafted.
A designer may design a document that serves well as many people as possible.
But there is no design that truly works for all.
For example: make the line height bigger and dyslectic readers may have an easier time.
But other readers rather have more text fitted on their screen.
Every corporeal decision you make will improve the document for one group of consumers,
but will make it worse for another.

See this webpage for an overview of corporeal considerations for dyslectic readers:
<https://uxplanet.org/what-to-consider-when-designing-for-dyslexia-b99d373905ac>.

#### Line tracking

Every sentence on a new line.
A slightly increased line height.
The current line is highlighted.
This can help readers that have trouble keeping track where they are.
I suffer a bit from this.

#### Contrast and colour

Just the right contrast and colours.
Some readers require high contrast for visibility.
Others require lower contrast against eye fatigue.
The right amount of contrast and the right colours are very personal.
That is why the consumers knows best.

#### Folding

Paragraphs and sections can be folded and unfolded.
This helps readers who are overwhelmed by large amounts of content at once.
Based on preference documents can start out with certain elements folded or unfolded.

#### Images

Most webpages automatically download images and display them in the page.
Consumers with slow internet can choose not to and only retrieve images on command.
Consumers reading in a terminal can choose to only show images in a program of choice on command.
A consumer might want to see an overview of all images on a page.
A consumer might wish to have a slide show inserted into the page if multiple images are come one
after the other.

#### Emphasis

Readers may render emphasis as as usual: bold, strike-through, etc.
Blind consumers may listen to the text instead and might have different voices for different
emphasis.

#### Links

Consumers may choose whether links open in a new tab or not.
The consumer decides so it is always consistent.
The document may suggest in the meta data if it intends on a new tab or not.
The consumer may ignore this.
The reader might assign different colours depending on if the link is local, on the same site or 
going to another website.
The consumer has their own way of marking whether a site was visited by them already.

#### Navigation
 
The consumer has their preferred way of browsing the document/website menu.
Maybe they prefer a drop-down menu.
Maybe they prefer it out of sight with a fuzzy finder bound to a key combination.
The consumer can consistently browse the menus across documents.

#### Lists

The consumer can have their preferred style for lists.
Think about indentation for sub lists, what icon on what level.
Ordered lists starting at 0 instead of 1.
Ordered lists with alphabetic ordering or Roman numerals.
List may implement folding.

#### Code blocks

Code blocks can be rendered with the consumers preferences.
Colours and fonts are even more important and personal when reading code.
Renderers could allow consumers to the run the code in the document.
The results of the code can be inserted into the document.
The consumer can open the code directly in their favourite editor without copying and pasting it.

#### Table of contents

The consumer may wish to summon a table of contents or have it inserted at the desired place in
the document.
If this was an incodoc document or if this markdown was rendered through incodoc,
you would only see the table of contents if you had it enabled it or if you summoned it.

#### Heuristics and statistics

The consumer may want to have a word count or estimated time to consume shown.
Consumers have their own average pace of consuming.
The estimated time can be personalised for the specific consumer.

#### Multiple/specific configurations

Consumers may have different configurations for various document types.
The might have a style for academic papers, one for wikis, one for blog posts and one for printing.
The consumer might have specific configurations for specific websites that they like to visit.
A special one for Wikipedia for example.
They may consume each class of documents as they wish.

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

All of these variations of corporeal elements are not at all serving the consumer.
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

Furthermore, it can be said that many contemporary documents are really applications.
PDFs are basically programs. HTML + CSS + JS pages are web applications rather than web documents.
See also the "Existing document formats" section.
Documents are more accessible and give the user more freedom, security and privacy.
If something can be a document instead of an application, it should be strongly considered.

In the end, the spaghetti like entanglement of content and style/behaviour limits the consumer.
The consumer is limited in reading the document in a way that serves them best.
The author has forced a particular way of consuming the document onto the consumer.

### Solution

The solution is to serve documents that have no corporeal elements at all.
By having no corporeal elements the consumer is free to shape their experience.

#### Personal experience

Changing fonts, colours and other corporeal elements is trivial when you don't have to untangle
style from content.
Incodoc is designed to be easy to deal with.
Everyone can consume documents in a way they want to.

#### Consistent experience

Consumers can consume documents from a wide range of sources and authors but do so in a consistent
experience.
All documents will look and behave the same.
The consumer interacts consistently with all of them.
All documents elements will behave the same across all documents.

#### Accessibility

The aforementioned freedom can help people with disabilities.
People with disabilities need a different corporeal representation of the content than most.
This is trivial with incorporeal documents.

#### Less data and bloat

Despite incodoc not being designed to minimise file sizes, it can reduce document size just by
not having to include style and scripting.
Media like background images may be ignored and thus not downloaded when converting corporeal sites.
With incorporeal documents they would not be included to begin with.

#### Privacy and security

Incodoc does not have document level scripting abilities.
The only scripting incodoc supports is code blocks.
Users have complete control over whether to execute a code block or not and what to execute it with.
In general, incodoc documents should still be readable when the consumer decides to not run any code.

In browsers, JS (document level scripting) can access things like screen resolution,
fonts installed, audio devices available, etc.
This is needed because site decides how it is rendered and that depends on the environment.
Therefore the environment supplies these details when the document scripts are ran.
This can be used to fingerprint users and send back information to the document issuer.
The user can disable JS but then most websites do not work any more.
Because of the incorporeal nature of incodoc, a browser or incodoc viewer does not have to expose
such details.
No information can be send back unless the consumer runs a code block with code in it that does so.

Complex document formats sometimes include macros, some type of scripting that shapes the document.
These complex systems are probable to have bugs that allow for exploitations.
This can lead to users being compromised just by opening a document, because rendering the document
correctly requires some kind of script to be ran.

Because incodocs really are just documents and not applications, a whole class of risks for the
consumer is eliminated.
Viewing incodoc documents should be as safe as viewing plain HTML or markdown documents.

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
It will be the first document format convertible to incodoc
(see: [md-to-incodoc](https://github.com/codybloemhard/md-to-incodoc))
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
Exporting incodoc as HTML + CSS allows the user to consume content in their browser,
rendered as they prefer.
Combining the two, the (HTML + CSS) -> incodoc -> (HTML + CSS) pipeline can be of utility as well.

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

# Extra

## Name

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

## "Consumer"

In this document the term "consumer" is used for the one who consumes incodoc documents.
Because of the incorporeal nature, we cannot be sure our consumers will read our documents.
They might listen to it or take in the content in other ways.
That is why the consumer is referred to as a consumer and not as a reader.
Consumer is chosen instead of user, to make clear that we are talking about the consuming user
and not the producing user.
The user of incodoc that produces documents is referred to as an author.

## License

Code under MIT, see `LICENSE`.
This readme under CC BY 4.0:

This work © 2025 by Cody Bloemhard is licensed under CC BY 4.0.
To view a copy of this license, visit https://creativecommons.org/licenses/by/4.0/

