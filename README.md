# lzfdecompressor
Just a small app I use at work for unpacking lzf compressed content. 

I use it along with a few other tools to unpack base64 encoded protobuf messages, but it can decompress all kinds of content.

The app itself is meant to be used like the example below, with the input coming from a pipe.

## Usage

The example below will take the following steps:
1. Pull the current buffer from the clipboard (`pbpaste` works for mac, there are other tools for other OS's)
2. Strip all newlines
3. Unpack the base64 encoded content
4. Run the content through the *lzfdecompress* app (ie, this app)

```bash
pbpaste | tr -d '\r' | base64 --decode | lzfdecompress >> $FILENAME
```

## Compile and stuff
To compile this application, make sure you have a rust toolchain install. Right now, I'd say [rustup](https://www.rustup.rs/) is the way to go.

Then, clone the repo: `git clone https://github.com/silverbeak/lzfdecompressor.git`

Compile the code: `cargo build --release`

And finally, move the app from `./target/release/lzfdecompressor` to your desired location (add the .exe stuff if you're on windows)

Happy decompressing!
