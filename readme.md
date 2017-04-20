# MayJun2017
This is a small card game. The goal is to have a card game that has no random effects at all. Cards will be put in decks in order, cards will have no random effects, etc.

## How to build
For all platforms, you can just run `cargo run` and it should work. The only library that might have issues is `freetype`. Read below how to make freetype work on your machine.

### Linux
It should just work &trade;. If not, please let us know.

### OSX
On OSX you need to install freetype as an external dependency.
`brew install freetype`

### Windows
After building you need to copy freetype.dll to the directory that the executable exists. Normally this will either be `target/debug` or `target/release`. Running this project with Cargo will work because freetype.dll exists in the root of the folder.

If for some reason you're compiling the x32 version. Copy the `.cargo/freetype/i686/freetype.dll` to the root folder instead.
