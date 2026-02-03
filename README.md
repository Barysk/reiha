# レイハ   [ Reiha ]



### Describtion

A tool for creating simple presentations without need in fighting with graphical interface.

Initially tool was targeting _only_ [takahashi method](https://en.wikipedia.org/wiki/Takahashi_method). But I found it quite useful to have more features, such as:

- Image as background
- Text and Image slides
- Codeblock slides
- Page numbering
- Next slide preview



### How it looks (presentation: left side, terminal output: right side)

![An image of how it looks](example.png)



### Usage

```reiha <path>```



### Options

```
-t, --theme dark|light|<bg_hex>x<font_hex> - Set theme
-l, --linear - set texture filtering for images to linear, default is nearest
-f, --font <path/to/font> - Use a custom font
-m, --mono-font <path/to/font> - Use a custom font
-r, --resolution <width>x<height> - Set virtual resolution (default 1600x1200) (max 3840x3840)
-n, --numbering - turn on the slide numbering
-a, --numbering-anchor <position> - position: [ bl | bc | br | tl | tc | tr ]. If incorrect defaults to bl (bottom left)
-b, --background <path/to/image.png> <filtering> - filtering: [ linear | l | nearest | n ].
-p, --preview - shows next slide in your terminal if there is such
```



### Movement

```
To close reiha you can use:
Esc   ||   Q

To move to next slide you can use:
Arrow Down   ||   Arrow Right   ||   J   ||   L   ||   Left Mouse Button

To move to previous slide you can use:
Arrow UP   ||   Arrow Left   ||   K   ||   H   ||   Right Mouse Button

To enter fullscreen mode you can use:
F11   ||   F

Turn on/off slide numbering:
N

Turn on/off next slide preview:
P

Switch background color with Font color
S
```



### Config file
Its location can be ```/home/user/.config/reiha/config```.

Config example:
```
--theme 000011xff4444
--resolution 400x300
--linear
--font /home/user/.fonts/Catholicon.otf
--numbering
--preview
```

Short versions of flags can be used.



### Syntax:

````
| Example syntax that actually can be ran
|   line that starts with '|' is considered a comment
|   the .rh extension is totally optional, you can use .txt [ or not use any ]


レイハ


Default fonts are:
  IPA Font - General
  Ubuntu Mono - Monospace
|  You can provide your own fonts, refer to
|    reiha --help


@./img/scr.png
| this is how you make an image slide


@./img/th.png
Text under image
| this is how you make an image slide with text
| image takes TOP    70% of space
|  text takes BOTTOM 30% of space


| this comment is going to be fully omitted


Text you type
  here is going
    to be scaled
automatically to
      fit the slide
| Those spaces at the line start are preserved.


```
use crate::theming::*;
use crate::utils::*;

async fn main() {
    let config = Config::from_file();
    let mut font: Font = if let Some(path) = &config.font_path {
        let data = std::fs::read(path).expect("Failed to read font file from config");
        load_ttf_font_from_bytes(&data).expect("Failed to load font from config")
    } else {
```
| Codeblock example
| use ``` at the start of the block
|         and ``` at the end of the block to create one.
| The monospace font is going to be applied to the codeblock


\
| this creates an empty slide


A slide with
~
an empty line
| To create an empty line use ~ at the line that should be empty


Comments are
optional.


Questions?
````



### Output to console

```
[time 0:41]
[slide 4/10]
===[ Content ]===============================================

[ image ]

Text under image

= =[ Notes ]= = = = = = = = = = = = = = = = = = = = = = = = =
| this is how you make an image slide with text
| image takes TOP    70% of space
|  text takes BOTTOM 30% of space
=============================================================



___[ Next Slide ]____________________________________________
[slide 5/10]
===[ Content ]===============================================
Text you type
  here is going
    to be scaled
automatically to
      fit the slide

= =[ Notes ]= = = = = = = = = = = = = = = = = = = = = = = = =
| Those spaces at the line start are preserved.
=============================================================
```
