Deterministic selection of N number of options, based on the consistent hash of an input value.

The input value currently supports two character hex value of a byte, with base64 added shortly.

The input value is converted to a byte array and subsequently xor'ed into a single byte

The resulting byte modulo the number of choices given provides an index value to the available choices.

This means that you _MUST_ always supply the same available choices string (values AND order) to get the same resulting choices.


Building
--------

Build CLI

```
cargo build --bin cel --features="clap" --release
```

Build as library

```
cargo build  --release
```

Runnning CLI
------------

```
target/release/cel -v --choices 'A|B|C|D|E|F|G|H|I|J|K' --choice-count=3 --input-type 'hex' --input $(echo -n 'hello world' | md5sum | awk '{printf "%s", $1}')
```

will output

```
Args {
    choices: [
        "A",
        "B",
        "C",
        "D",
        "E",
        "F",
        "G",
        "H",
        "I",
        "J",
        "K",
    ],
    choice_count: 3,
    input_type: HexV1,
    input: "5eb63bbbe01eeed093cb22bb8f5acdc3",
    delimiter: "\n",
    omit_new_line: false,
    verbose: 1,
}
--- ---
C
D
E
```