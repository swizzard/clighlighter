# classes

each tree-sitter grammar uses a different set of labels for the nodes it produces.
some of these (like `identifier` or `new`) identify nodes worth highlighting (aka "special"), while
others (mostly punctuation and punctuation sequences like `{` and `===`) should be
returned as-is (aka "regular"). the rest are skipped.

because the tree is traversed depth-first without lookahead, it's important to avoid
redundancy by not processing both a 'higher-level' node and its 'children'; to this
end i tend to prefer 'lower-level' kinds like `export`, which in TS is literally just the string `export`,
to `export_statement`, which goes to the `;`, because an `export_statement` can contain nodes of
kinds like `interface` or `type_identifier` that need to be highlighted elsewhere.

as for what exactly comprise "the rest": you can look at the parser's `node_types.json` file
([ts](https://github.com/tree-sitter/tree-sitter-typescript/blob/master/typescript/src/node-types.json),
[rust](https://github.com/tree-sitter/tree-sitter-rust/blob/master/src/node-types.json)) but i personally
do not find them particularly informative. using the debug-printing `highlighters" in the
[explore](./explore.rs) module might prove more informative.

## Rust

### special kinds (will be rendered as `<code class="...">...</code>`)

- `&`
- `boolean_literal`
- `char_literal`
- `crate`
- `else`
- `enum`
- `field_identifier`
- `fn`
- `for`
- `identifier`
- `if`
- `impl`
- `let`
- `line_comment`
- `mod`
- `mutable_specifier`
- `primitive_type`
- `pub`
- `self`
- `static`
- `string_literal`
- `struct`
- `trait`
- `type_identifier`
- `use`
- `while`

### regular kinds (will be rendered as-is)

- `(`
- `)`
- `;`
- `::`
- `{`
- `}`
- `,`
- `:`
- `=`
- `.`
- `<`
- `>`
- `'`
- `\_`
- `!`
- `->`
- `#`
- `[`
- `]`,

## TS

### special kinds

- `as`
- `const`
- `export`
- `function`
- `identifier`
- `interface`
- `literal_type`
- `new`
- `number`
- `property_identifier`
- `return`
- `type_identifier`
- `undefined`

### regular kinds

- `=`
- `<`
- `>`
- `(`
- `)`
- `{`
- `}`
- `|`
- `;`
- `,`
- `=>`
- `===`
- `:`
- `?`
- `.`
