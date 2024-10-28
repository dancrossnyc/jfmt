# jfmt: Jazzed Format

`jfmt` is a small utility that takes an unsigned number as its
sole argument, and prints it in binary, in a manner similar to
the `=j` format from the illumos/Solaris debugger `mdb`.

The number can be given in decimal, hex, octal, or binary, using
the usual C-like prefix syntax for each radix.  `0t` is also
supported for decimal, which is the default if no prefix is
given.  The number may be up to 128 bits wide.

In addition to printing the number in binary, it is also printed
in hex, decimal, and octal.

## Usage

A few examples of invocations and their outputs:

```
term% jfmt 11
                1011
                ▴ ▴▴
                │ │╰── bit 0 mask 0x1
                │ ╰─── bit 1 mask 0x2
                ╰───── bit 3 mask 0x8

                hex: 0xb
                dec: 11
                oct: 0o13
term% jfmt 0xf001000b
                11110000000000010000000000001011
                ▴▴▴▴           ▴            ▴ ▴▴
                ││││           │            │ │╰── bit  0 mask 0x00000001
                ││││           │            │ ╰─── bit  1 mask 0x00000002
                ││││           │            ╰───── bit  3 mask 0x00000008
                ││││           ╰────────────────── bit 16 mask 0x00010000
                │││╰────────────────────────────── bit 28 mask 0x10000000
                ││╰─────────────────────────────── bit 29 mask 0x20000000
                │╰──────────────────────────────── bit 30 mask 0x40000000
                ╰───────────────────────────────── bit 31 mask 0x80000000

                hex: 0xf001000b
                dec: 4026597387
                oct: 0o36000200013
term% jfmt 04751
                100111101001
                ▴  ▴▴▴▴ ▴  ▴
                │  ││││ │  ╰── bit  0 mask 0x001
                │  ││││ ╰───── bit  3 mask 0x008
                │  │││╰─────── bit  5 mask 0x020
                │  ││╰──────── bit  6 mask 0x040
                │  │╰───────── bit  7 mask 0x080
                │  ╰────────── bit  8 mask 0x100
                ╰───────────── bit 11 mask 0x800

                hex: 0x9e9
                dec: 2537
                oct: 0o4751
term%
```

## Bugs

Perhaps we should take more than one argument, formatting each
in turn.

The use of UNICODE drawing characters is hard-coded, which may
cause problems on some terminals or with fonts that do not
include the drawing glyphs.  An argument should be provided to
force a fallback to ASCII characters.
