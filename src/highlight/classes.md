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

- `as`
- `async`
- `await`
- `boolean_literal` (NOT `false`, `true`)
- `break`
- `char_literal`
- `const`
- `crate`
- `doc_comment`
- `dyn`
- `else`
- `enum`
- `extern`
- `field_identifier`
- `float_literal`
- `fn`
- `for`
- `identifier`
- `if`
- `impl`
- `in`
- `integer_literal`
- `let`
- `line_comment` (includes newline)
- `loop`
- `match`
- `mod`
- `mutable_specifier`
- `primitive_type`
- `pub`
- `raw_string_literal`
- `ref`
- `self`
- `static`
- `string_literal`
- `struct`
- `super`
- `trait`
- `type_identifier`
- `unsafe`
- `use`
- `while`

### regular kinds (will be rendered as-is)

- `(`
- `)`
- `;`
- `::`
- `{`
- `}`
- `
- `:`
- `=`
- `.`
- `<`
- `>`
- `'`
- `_`
- `!`
- `->`
- `=>
- `#`
- `[`
- `]`
- `!=`
- `%`
- `&&`
- `&`
- `*`
- `+`
- `-`
- `/`
- `<<`
- `<<=`
- `<=`
- `==`
- `>=
- `>>=`
- `>>`
- `||`
- `|`
- `#`
- `$`
- `%`
- `%=`
- `&=`
- `'`
- `*/`
- `*=`
- `-=`
- `..`
- `...
- `..=`
- `/*`
- `//`
- `/=`
- `?`

## TS

### special kinds (will be rendered as `<code class="...">...</code>`)

- `any`
- `as`
- `assert`
- `async`
- `await`
- `boolean`
- `case`
- `catch`
- `class`
- `comment`
- `continue`
- `const`
- `debugger`
- `declare`
- `delete`
- `do`
- `else`
- `enum`
- `export`
- `extends`
- `finally`
- `for`
- `from`
- `function`
- `get`
- `global`
- `html_comment`
- `identifier`
- `if`
- `implements`
- `import`
- `in`
- `infer`
- `instanceof`
- `interface`
- `is`
- `literal_type`
- `namespace`
- `new`
- `number`
- `object`
- `of`
- `private`
- `private_property_identifier`
- `property_identifier`
- `protected`
- `public`
- `regex`
- `require`
- `return`
- `set`
- `static`
- `string`
- `super`
- `symbol`
- `this`
- `this_type`
- `throw`
- `type`
- `type_identifier`
- `typeof`
- `undefined`
- `var`
- `void`
- `while`
- `with`
- `yield`

### regular kinds (will be rendered as-is)

- "!"
- "!="
- "!=="
- "="
- "<"
- ">"
- "("
- ")"
- "{"
- "}"
- "|"
- ";"
- ""
- "=>"
- "==="
- ":"
- "?"
- "."
- "\""
- "${"
- "%"
- "%="
- "&"
- "&&"
- "&&="
- "&="
- "'"
- "\*"
- "\*\*"
- "\*\*="
- "\*="
- "+"
- "++"
- "+="
- "+?:"
- "-"
- "--"
- "-="
- "-?:"
- "."
- "..."
- "/"
- "/="
- "<<"
- "<<="
- "<="
- "="
- "=="
- ">="
- ">>"
- ">>="
- ">>>"
- ">>>="
- "?."
- "?:"
- "??"
- "??="
- "@"
- "["
- "]"
- "^"
- "^="
- "`"
- "{|"
- "|="
- "||"
- "||=="
- "|}"
- "~"
