# Reiha レイハ

### Describtion

A tool for making presentations using simple text file as an input, and a slide view as an output

it uses takahashi method.
https://presentationzen.blogs.com/presentationzen/2005/09/living_large_ta.html

### syntax example:

```

| This is an example syntax
|   - every line that starts with | is considered a comment
|     and only being seen in terminal during presentation
|   - there can be your notes
|   - the first comment block is omitted

Reiha

| The name of the tool
| inspired by sent, wend and takahashi method

@path/to/image.png

| To make image slide, start the line with @

depends on
- Raylib

| Raylib is a powerful gamedevelopment library that is flexible enough

\

| Use \ to create an empty slide

present FILENAME
one slide per paragraph

questions?
```

### output to console example

```
========
Present

| The name of the tool
| inspired by sent, wend and takahashi method
========
[1/5]
3:12
``` 