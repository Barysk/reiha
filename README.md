# Reiha レイハ

### Describtion

A tool for creating presentations using a plain text file. Thanks to this, your
presentation content can always be accessed from any device.

It utilises the takahashi method:
https://presentationzen.blogs.com/presentationzen/2005/09/living_large_ta.html

Inspired by ```sent```

Some features borrowed from ```wend```

### usage

```reiha <path>```

### options
```-t, --theme dark|light|<bg_hex>x<font_hex>``` - Set theme
```-l, --linear``` - set texture filtering mode to linear, default is nearest
```-f, --font <font_path>``` - Use a custom font
```-r, --resolution <width>x<height>``` - Set virtual resolution (default 1600x1200) (max 3840x3840)

### movement

```
To close reiha you can use:
Esc || Q

To move to next slide you can use:
Arrow Down || Arrow Right || J || L

To move to previous slide you can use:
Arrow UP || Arrow Left || K || H

To enter fullscreen mode you can use:
F11 || F

*Letter case doesn't metter
```

### config file
Its location can be ```/home/user/.config/reiha/config```

```
--theme 000011xff4444
--resolution 400x300
--linear
--font /home/user/.fonts/Catholicon.otf
```
### how it looks (presentation: left side, terminal output: right side)

![An image of how it looks](example.png)

### syntax:

```
| This is an example syntax
|   - every line that starts with | is considered a comment
|     and only being seen in terminal during presentation
|   - there can be your notes
|   - the first comment block is omitted

レイハ
| The name of the tool
| inspired by sent, wend and takahashi method

@test/screen.png
| To make image slide, start the line with @
|   Blah

It scales
  text that you
    write to fit the
  slide
|
| Automatically
|  Of course

\
| Use \ to create an empty slide

    reiha <filename>
one slide per paragraph

questions?
```

### output to console

```
========
It scales
  text that you
    write to fit the
  slide
--------
|
| Automaticly
|  Of course
========
[slide 3/6]
[time 1:02]
``` 
