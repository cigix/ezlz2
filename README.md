# EZLZ2: a simple implementation of a Lempel-Ziv compression scheme

## Usage

```sh
$ ezlz [input [output]] # compression
$ unzelz [input [output]] # decompression
```

If both input or output files are omitted, `stdin` and `stdout` are used.

## Building

Running `make` at the root will build `ezlz` and `unezlz` and copy them at the
root.

## Escape value

Lempel-Ziv requires the use of an escape byte. By default, ezlz2 will use
`0xAA`. To change this value, edit the `ESCAPE` value in `src/lib.rs`.

Note that `unezlz` will not properly decompress streams compressed with a
different escape value.
