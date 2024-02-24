# JSON Parser (jp)

`jp` is simple JSON parser writen in Rust that breaks parsing into two steps: lexical analysis and syntactic analysis. If parsing is successful, the JSON input is formatted and written to standard output (stdout).

## Usage
```
jp file
```

## Examples
```
jp file.json
cat file.json | jp
```
