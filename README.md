# rframe

A simple rust CLI tool to draw [Apple device bezels](https://developer.apple.com/design/resources/#product-bezels) over simulator screenshots. 

## Usage

```bash
cargo run --release [screenshot] [output path] --dir=[assets directory] <bezel file name>
```

## Performance

Renders the framed image in 17ms on an Intel Core i7 8750H.
