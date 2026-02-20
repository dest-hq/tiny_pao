# tiny-pao

Software renderer written in Rust

## Status

WIP - currently supports basic shapes and text rendering.

## Features

- [x] Clear screen with solid colors
- [x] Window resize support  
- [x] Draw filled rectangles and rounded rectangles
- [x] Draw circles
- [x] Text rendering
- [ ] Anti-aliasing
- [ ] Gradients

## Performance

Benchmarks on [Ryzen 5 5625U]:
- Clear 1080p screen: 145.05 µs
- Draw 1000 rectangles: 1.1631 ms
- Clear 4K screen: 2.4190 ms

## Usage

Text Renderer Example
`examples/src/text_render.rs`

Run Text Renderer
```bash
cargo run --release --bin text_render
```
<img width="500" height="500" alt="Text render example" src="https://github.com/user-attachments/assets/9fc9cad7-8e87-43c4-a6d6-38ffc852ef7f" />

## License

MIT
