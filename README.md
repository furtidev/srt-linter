# `srt-linter` 💬🔍
<p align="center">
<img src="./assets/demo.gif">

</p>

💬🔍 `srt-linter` is a CLI tool to inspect and detect issues inside [SubRip Text](https://en.wikipedia.org/wiki/SubRip) (`.srt`) files.

# ToC
- [Installation 📥](#installation-)
- [Usage ⌨️](#usage-)
- [Wishlist 💭](#wishlist-)
- [Contributing 🤝](#contributing-)
- [Acknowledgements ✨](#acknowledgements-)
- [License ⚖️](#license-)

## Installation 📥
Install `srt-linter` with `cargo`:
```
$ cargo install srt-linter
```

If you don't have `cargo` or don't want to build `srt-linter` yourself, you can get the latest pre-built binary from [here.](https://github.com/furtidev/srt-linter/releases)

## Usage ⌨️
`srt-linter` has a very simple interface:
```bash
$ srt-linter --help
Look for issues inside SubRip text (.srt) files.

Usage: srt-linter [OPTIONS] <FILE_PATH>

Arguments:
  <FILE_PATH>

Options:
  -v, --verbose  Logs additional information about internal actions
  -s, --strict   Enforces stricter rules for suspicious behavior
  -h, --help     Print help
  -V, --version  Print version
```

Consider this:
```bash
$ srt-linter -s -v ./the.sopranos.s6.ep4.srt
[INFO] Detected BOM.
[SUCCESS] File is semantically OK.
[SUCCESS] File is structurally OK. Read 1565 line(s).
```
`srt-linter` is saying the following things:
1. It detected the byte-order mark (`U+FEFF`) character on the file. (`-v/--verbose`)
2. It successfully went through the file and ensured each line is accurate as per the SubRip Text format.
3. It successfully parsed the file, ensuring structural soundness and read 1565 lines of subtitle text.

`srt-linter` says this file is valid. Let's load it up on a media player and see if `srt-linter` is right.

I'm using `mpv` and there you go:

<img src="./assets/mpv_screenshot.png" width=80%>

## Wishlist 💭
- [ ] Implement a TUI to visualize the subtitles.
- [ ] Implement `.srt` format's unofficial extensions.

## Contributing 🤝
Hey, thanks! If you've found a bug or something that works unexpectedly, feel free to [open an issue](https://github.com/furtidev/srt-linter/issues/new). 

If you're a programmer and are interested in fixing it yourself, take a look at [CONTRIBUTING.md](./CONTRIBUTING.md). I really appreciate your help!

## Acknowledgements ✨
Thanks to [@hitblast](https://github.com/hitblast) for helping me test `srt-linter`.

## License ⚖️
This project is licensed under the [MIT license](./LICENSE).
