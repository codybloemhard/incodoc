features:

- [x] metadata
  - tags: string
  - props: key pairs
- [x] navigation
  - links lists of lists of links
- [x] headings
- [x] sections
- [x] text
- [x] links
  - tag, local file, web url, email, ...
  - link type hint in meta: img, vid, pdf
- [x] emphasis
  - emphasis/deemphasis
  - light, medium, strong
- [x] lists
  - ordered, unordered, checked, recursive
- [x] paragraphs
  - single lines by hinting in meta
- [x] code
  - language hint
  - mode hint: show, executable, replace with result, ...
  - could be opened in editor
  - could be assigned a renderer by the user (eg. bat)
- background text: in props
  - titles for links
  - alt for images
  - etc
- tags
  - anything can have a tag
  - links can link to tags

things to write about

- all the features and suggested use

think about:

citation?
quotes generalization?
common image
  string -> image
  user supplies the images
  emoji, stickers, gifs, etc
  fallback img
tables and graphs
  as element? as code that executes?

anti features:

colours
fonts
spacing
layout

are these anti features?

line breaks?
horizontal rules?

half cooked examples:

'Truth table for X AND Y',
data{
    props{
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
    props{
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

Some markdown like language that would be parsed and turned into the above format.
For actual writing.

```md
$prop language en
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

