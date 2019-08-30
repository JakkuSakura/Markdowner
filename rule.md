# markdown syntax rules

## TOKEN
line_beginning,

byte,

## Grammar

heading = line_beginning # text

bold = ** text **

text = (heading, bold, byte) +