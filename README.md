# Clighlighter

ok so i wanted a minimally-fussy code highlighter

this is basically that

code goes in, html comes out
formatting (whitespace) is maintained

## example

source file:

```rust
use clap::{Parser, ValueEnum};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[arg(short, help = "path to source file")]
    pub in_file: String,
    #[arg(short, help = "path to output file, defaults to STDOUT")]
    pub out_file: Option<String>,
    #[arg(
        short = 'l',
        value_name = "LANGUAGE",
        help = "highlighter to use, defaults to TS",
        default_value = "ts"
    )]
    pub highlighter: HighlighterChoice,
}

#[derive(Clone, Eq, PartialEq, ValueEnum)]
pub enum HighlighterChoice {
    /// Typescript
    TS,
    /// Rust
    Rust,
}
```

output:

```
<pre class="code">
<code class="use">use</code> <code class="identifier">clap</code>::{<code class="identifier">Parser</code>, <code class="identifier">ValueEnum</code>};

#[<code class="identifier">derive</code>(<code class="identifier">Parser</code>)]
#[<code class="identifier">command</code>(<code class="identifier">version</code>, <code class="identifier">about</code>, <code class="identifier">long_about</code> = <code class="identifier">None</code>)]
<code class="pub">pub</code> <code class="struct">struct</code> <code class="type_identifier">Cli</code> {
    #[<code class="identifier">arg</code>(<code class="identifier">short</code>, <code class="identifier">help</code> = <code class="string_literal">"path to source file"</code>)]
    <code class="pub">pub</code> <code class="field_identifier">in_file</code>: <code class="type_identifier">String</code>,
    #[<code class="identifier">arg</code>(<code class="identifier">short</code>, <code class="identifier">help</code> = <code class="string_literal">"path to output file, defaults to STDOUT"</code>)]
    <code class="pub">pub</code> <code class="field_identifier">out_file</code>: <code class="type_identifier">Option</code><<code class="type_identifier">String</code>>,
    #[<code class="identifier">arg</code>(
        <code class="identifier">short</code> = <code class="char_literal">'l'</code>,
        <code class="identifier">value_name</code> = <code class="string_literal">"LANGUAGE"</code>,
        <code class="identifier">help</code> = <code class="string_literal">"highlighter to use, defaults to TS"</code>,
        <code class="identifier">default_value</code> = <code class="string_literal">"ts"</code>
    )]
    <code class="pub">pub</code> <code class="field_identifier">highlighter</code>: <code class="type_identifier">HighlighterChoice</code>,
}

#[<code class="identifier">derive</code>(<code class="identifier">Clone</code>, <code class="identifier">Eq</code>, <code class="identifier">PartialEq</code>, <code class="identifier">ValueEnum</code>)]
<code class="pub">pub</code> <code class="enum">enum</code> <code class="type_identifier">HighlighterChoice</code> {
    <code class="line_comment">/// Typescript
</code>    <code class="identifier">TS</code>,
    <code class="line_comment">/// Rust
</code>    <code class="identifier">Rust</code>,
}
</pre>
```

[classes.md](src/highlight/classes.md) has lists of classes each highlighter produces

## usage

```
Usage: clighlighter [OPTIONS] -i <IN_FILE>

Options:
  -i <IN_FILE>
          path to source file

  -o <OUT_FILE>
          path to output file, defaults to STDOUT

  -l <LANGUAGE>
          highlighter to use, defaults to TS

          [default: ts]

          Possible values:
          - ts:   Typescript
          - rust: Rust

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

## todo

- [ ] coverage
  - [ ] more `kind`s
  - [ ] non-alpha `kind`s/mapping names
- [x] flexibility
  - [x] trait
  - [x] not just ts
  - [x] cli argument
- [ ] improve performance by writing directly to output
- [ ] tests

## contributing/extension

`cargo add tree-sitter-whatever`, then implement [`Highlight`](src/highlight/mod.rs)
and add your new highlighter to [the enum](src/cli.rs) and
[the mapping function](src/lib.rs).

the `explore` feature gates [highlighters](src/highlight/explore.rs) that print node debug info,
they might help you.
