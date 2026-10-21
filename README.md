# tawata (多和田)

> Convert epub files to markdown files

[![CircleCI](https://circleci.com/gh/paazmaya/tawata.svg?style=svg)](https://circleci.com/gh/paazmaya/tawata)
[![codecov](https://codecov.io/gh/paazmaya/tawata/branch/master/graph/badge.svg)](https://codecov.io/gh/paazmaya/tawata)

## The "Autocorrect" Feature Explained

It is designed to fix spacing and punctuation between CJK (Chinese/Japanese/Korean) characters and English words.

However, EPUBs have a second, more annoying whitespace issue: Split Words.
Often, publishers use individual <span> tags for every letter to apply specific kerning or CSS effects. This results in extracted text like H e l l o w o r l d.
I added a Regex pass (\b\w(?: \w)+\b) to detect sequences of single letters separated by spaces and merge them back into single words. Note: This occasionally merges very short valid phrases (like "I am" -> "Iam"), but for EPUB extraction, it fixes far more than it breaks.

## Background for the project name

The name of the project is for honouring the legacy of a late master, Mr Tawata from the Ryukyu archipelago, who contributed to the martial arts that we today know as **karate** and **ryukyu kobujutsu**.

There are forms carrying his name, such as "Tawata no Passai" and ["Tawata no Sai"](https://www.youtube.com/watch?v=YAQlFJIj32E), which both have distinctive use of a stance called "nekoashidachi" and sweeping motion related to the given stance.

[Read more about why these martial arts are important for me at `karatejukka.fi`.](https://karatejukka.fi)

## Version history

[Changes happening across different versions and upcoming changes are tracked in the `CHANGELOG.md` file.](CHANGELOG.md)

## License

Copyright (c) [Juga Paazmaya](https://paazmaya.fi) <paazmaya@yahoo.com>

Licensed under the [MIT license](LICENSE).
