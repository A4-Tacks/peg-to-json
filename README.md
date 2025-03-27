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
comment     = ~<;[^\n]*\n> @comment
_           = ~<[ \t\r\n]*> [comment _]
ident       = ~<(?![0-9])[0-9a-zA-Z\-_]+> @ident
...
$ cargo run -- < grammar.abnf
{
  "comment": {
    "choice": [
      {
        "quiet": {
          "match": ";[^\\n]*\\n"
        }
      },
      {
        "expected": "comment"
      }
    ]
  },
  "_": [
    {
      "quiet": {
        "match": "[ \\t\\r\\n]*"
      }
    },
    {
      "optional": [
        "comment",
        "_"
      ]
    }
  ],
  "ident": {
    "choice": [
      {
        "quiet": {
          "match": "(?![0-9])[0-9a-zA-Z\\-_]+"
        }
      },
      {
        "expected": "ident"
      }
    ]
  },
...
```
