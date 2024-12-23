# incodoc

Incorporeal document experiment.
A document type that doesn't dictate appearance.
Could be HTML but in practice it is deeply involved with style.
Markdown is an incorporeal document.
The renderer, thus the user, chooses the font, colours, layout, etc.
Markdown is successful incorporeal document.
I want something like but with the ability to function like a static web page.

features:

metadata
  key pairs
navigation
  links lists of lists of links
headings/containers
text
links (point in text, heading, local file, web url, email, ...)
  link type hint: img, vid, pdf
emphasis (italic, bold)
lists
  ordered, unordered, checked
quotes generalization?
paragraphs (parts of parts)
single lines?
emoji
  user supplies the images
code generalization?
  language hint
  executeable hint
  could be opened in editor
  could be assigned a renderer by the user (eg. bat)
background text: bg-text in meta
  titles for links
  alt for images
  etc
tags
  anything can have a tag
  links can link to tags
  citation can cite a tag
tables
graphs

anti features:

line breaks
horizontal rules

meta{
    ("language", "en"),
    ("date", 31/12/9999),
}
nav{
    snav{
        "top level",
        link{'home', "/home"},
        link{'about', "/about"},
        link{'now', "/now"},
        link{'blog', "/blog/index"},
    },
    snav{
        "other articles",
        link{'I like cheese', "./cheese"},
        link{'Sosig is happiness', "./sosig"},
    },
}
part{
    head{0, 'very important'},
    'One must see this image.',
    link{'very important image', "image.png", meta{('bg-text', 'Extremely important image.')}},
    'Also this one.',
    link{'another important image', "website.com/image", meta{('type-hint', 'image')}},
    'For further questions see ',
    link{'questions', "#questions"},
    '.',
    head{1, 'questions', tag{"#questions"}},
    '
    Why is this important?
    Because it is.
    For even further questions email me at ',
    link{"email@address.com"},
    '.',
}
part{
    head{0, 'demo'}
    'This is a ',
    em{0, 'light'},
    ' emphasis.
    This is a ',
    em{1, 'medium'},
    ' emphasis.
    This is a ',
    em{2, 'strong'},
    ' emphasis.',
    'This is a ',
    em{-1, 'light'},
    ' de-emphasis.
    This is a ',
    em{-2, 'medium'},
    ' de-emphasis.
    This is a ',
    em{-3, 'strong'},
    ' de-emphasis.',
    'This is a list: ',
    ulist{
        item{'hello'},
        item{em{2, 'HELLO'}},
    },
    'This is a weird list: ',
    olist{
        item{'1'},
        item{'0'},
        item{'2'},
    },
    part{
        'This is a paragraph.',
        'I will cite something',
        cite{'@test-citation'},
    },
    part{
        'This is yet another paragraph, with an emoji: ',
        emoji{"laughing", tag{"@test-citation"}},
    },
    part{
        'This is will not compile:',
        code{
            meta{("language", "rust")},
            '
                let mut x = 0;
                let y = &mut x;
                let z = &mut x;
                *y = 1;
            ',
        },
        '
        Your viewer may try to run it, only if it wants to.
        This is a piece of code that suggest to be ran and its result inserted into this document.
        Only if you want to.
        ',
        code{
            meta{
                ("language", "typst-formula"),
                ("ex-hint", "suggested"),
            },
            '
                x -> y
            ',
        },
        'Truth table for X AND Y',
        data{
            meta{
                ("type-hint", "table"),
                ("ex-hint", "suggested"),
            },
            [
                [
                    ['T', F],
                    ['T', F],
                ],
                [
                    ['T', 'F'],
                    ['F', 'F'],
                ],
            ],
        },
        'This is a graph of my happiness:',
        data{
            meta{
                ("type-hint", "line-graph"),
                ("ex-hint", "suggested"),
            },
            [
                [
                    'year',
                    'happiness',
                ],
                [
                    (0, 100), (1, 90.5), (2, 72.3), (3, 45.9), (4, 17.6), (5, 0.3),
                ],
            ],
        },
    },
}
part{
    meta{("type", "footer")},
    '
    Copyright (c) 1337 me
    ',
}

Some markdown like language that would be parsed and turned into the above format.
For actual writing.

```md
$meta language en
$nav ""
  $nav "top level"
    - [home](/home)
    - [about](/about)
    - [now](/now)
    - [blog](/blog/index)
  $nav "other articles"
    - [I like cheese](cheese)
    - [Sosig is happiness](sosig)

# very important

One must see this image.

![very important image](image.png)

Also this one.

![another important image](website.com/image.jpg)

For further questions see [questions](##questions)

## questions

Why is this important?
Because it is.
For even further questions email me at <email@address.com>.

$footer

Copyright(C) some dude 
```

## License

Code under MIT, see `LICENSE`.
This readme under CC BY 4.0:

This work © 2024 by Cody Bloemhard is licensed under CC BY 4.0.
To view a copy of this license, visit https://creativecommons.org/licenses/by/4.0/
