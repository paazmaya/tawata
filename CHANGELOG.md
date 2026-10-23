# Version history for tawata (多和田)

This changelog covers the version history and possible upcoming changes.
It follows the guidance from https://keepachangelog.com/en/1.0.0/.

## `v0.1.0` (2026-10-23)

- Convert EPUB, MOBI, AZW3, AZW, and KFX ebooks to Markdown.
- Convert DJVU and DJV documents to one Markdown file per page using their text layers.
- Accept an ebook file, a `*` glob pattern, or a directory of ebooks; optionally search directories recursively.
- Write ebook content as one Markdown file per chapter by default, or as `book.md` with `--single-file`. If chapter boundaries cannot be identified reliably, write `book.md` instead.
- Extract ebook images to an `images/` subdirectory and rewrite Markdown image links to use those files.
- Optionally correct CJK/Western spacing and join words split by spaces with `--autocorrect`.
- Format generated Markdown with the OXC Markdown formatter by default; `--no-oxfmt` disables formatting.
- Keep input files unchanged and write converted content to a separate output directory named after the input file's stem.
