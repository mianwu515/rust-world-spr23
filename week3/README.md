# A Rust CLI tool - JSON formatter

![json-formatter](https://res.cloudinary.com/practicaldev/image/fetch/s--C2y3hpPL--/c_imagga_scale,f_auto,fl_progressive,h_500,q_auto,w_1000/https://dev-to-uploads.s3.amazonaws.com/uploads/articles/de0jdh2ebym2rpitoyns.png)

This tool takes in a json string without formatting, and prints the formatted json string to output stream, as well as writes to a specified file path.

## Usage
```bash
make format

make lint

make run-default
```
which will run 

```
cargo run -- jsonformatter --obj "{\"key\": 2}" "a.txt"
```

Or run your own json formatter with cmds

```bash
cargo run -- jsonformatter --obj [unformatted json string] [output file path]
```


### todo: make [output file path] param optional; make taking in a file possible.

## References
* [serde-json-crate](https://docs.rs/serde_json/latest/serde_json/fn.from_str.html)