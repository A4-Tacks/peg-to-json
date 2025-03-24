Convert ABNF like PEG declaration to JSON

## Operators
- `&` Positive lookahead
- `!` Negative lookahead
- `~` Quiet pattern
- `$` Slice pattern

# Examples
```
$ cat ./grammar.abnf
...
comment     = ";[^\n]*\n"
_           = "[ \t\r\n]*" [comment _]
ident       = "(?![0-9])[0-9a-zA-Z\-_]+"
...
$ cargo run -- < grammar.abnf
{
  "comment": {
    "match": ";[^\\n]*\\n"
  },
  "_": [
    {
      "match": "[ \\t\\r\\n]*"
    },
    {
      "optional": [
        "comment",
        "_"
      ]
    }
  ],
  "ident": {
    "match": "(?![0-9])[0-9a-zA-Z\\-_]+"
  },
...
```
