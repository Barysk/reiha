# Present (working name)

A tool for making presentations using simple text file as an input, and a slide view as an output

future syntax example (! not even alpha):

### Config

Every presentation starts by providing config information
```
[config]
resolution = [1920, 1080]
max_fps = 60
fallback_font = "Sans"
```

### Slide

to create a slide write

```
[slide]
```

### Text

after this you would probably want to add some text

```
[text]
position = [12, 12]
font = "Serif"
size = 32
And write there some text as you like. Use \n to new line
```

Want to place only text on slide? Use auto_text

```
[text_slide]
font = "Monospace"
Write some text here, it will be scaled to match the slide.
```

### Image

Image? Use this

```
[image]
position = [12, 32]
scale = 1.5
path/to/image
```

Want to have only image on slide? Here it goes

```
[image_slide]
path/to/image
```

### Comments

Comment? Use this ```//``` at the **start of the line!**