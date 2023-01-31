# A Rust CLI tool - JSON formatter

![json-formatter](https://res.cloudinary.com/practicaldev/image/fetch/s--C2y3hpPL--/c_imagga_scale,f_auto,fl_progressive,h_500,q_auto,w_1000/https://dev-to-uploads.s3.amazonaws.com/uploads/articles/de0jdh2ebym2rpitoyns.png)
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

