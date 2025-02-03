# Clighlighter

ok so i wanted a minimally-fussy code highlighter

this is basically that

(currently just ts) code goes in, html comes out
formatting (whitespace) is maintained

## usage

```
Usage: clighlighter <IN_FILE> [OUT_FILE]

Arguments:
  <IN_FILE>   path to source file
  [OUT_FILE]  path to output file, defaults to STDOUT

Options:
  -h, --help     Print help
  -V, --version  Print version
```

## todo

- [ ] coverage
  - [ ] more `kind`s
  - [ ] non-alpha `kind`s/mapping names
- [ ] flexibility
  - [x] trait
  - [ ] not just ts
  - [ ] cli argument
- [ ] improve performance by writing directly to output
