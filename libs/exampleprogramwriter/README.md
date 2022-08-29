# rust-customvmcpu - exampleprogramwriter

Writes binary programs to files in current working directory.

## Reading the code in Linux

With the tool `hexdump`, the files can translated into somewhat readable code:

```bash
hexdump -v -e '1/4 "%08x " "\n"' add_32_100.bin
```

which outputs

```txt
1: 03200064
2: 03000020
3: 04000002
4: 13000000
```

1. Line: 03 is the `li` instruction, `2` is the `$r2` register and `64` is the
   immediate value `100`.

2. Line: Same as in the 1. line, but `0` is the `$r0` register and `20` is the
   immediate value `32`.

3. Line: 04 is the `add` instruction, `0` is the `$r0` register and `2` is the
   `$r2` register.

4. Line: `13` is the `syscalli` instruction with immediate value `0` meaning
   exit.
