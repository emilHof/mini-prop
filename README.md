# Mini-Prop
Mini-Prop is a CLI tool for parsing [*LaTex*](https://www.latex-project.org/) formatted 
propositional statements and performing normalization and analysis steps on them.

## Installation
#### Crates.io:
```bash
$ cargo install mini-prop
```

#### From source:
Clone the main branch to your local mashine.
```bash
$ git clone https://github.com/emilHof/mini-prop.git && cd mini-prop
```

Build the binary with cargo.
```bash
$ cargo build --release
```

Move the binary into your path.
```bash
$ cp ./target/release/mini-prop /usr/bin/
```

## Usage
There are currently two main ways of passing propositions. One is through the terminal
itself.
```bash
$ mini-prop "A \land (B \lor C)" normal
```

Alternatively you can pass `mini-prop` the path to a text file containing the propositions.
```bash
$ mini-prop -f ./path/to/file/props.txt normal
```


