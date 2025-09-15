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
- [x] tables
  - header rows and regular rows
  - rows of paragraphs
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

heading
  - wrong level: auto fix?
link
  - empty items: fix by putting url as text

dedup nav, par, section

think about doing:

recursive is_contentless? eg section with empty par

things to write about

- all the features and suggested use
https://www.gnu.org/licenses/license-recommendations.html
https://www.marginalia.nu/log/a_119_pdf/

think about:

common image
  string -> image
  user supplies the images
  emoji, stickers, gifs, etc
  fallback img
  link with url convention?
tables and graphs
  regular table with incodoc items in it
  data frames that can be rendered as tables or graphs
  links to outside resources (csv, etc)

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

