# tiny-pao

Software renderer written in Rust

## Status

WIP - currently supports basic shapes and text rendering.

## Features

- [x] Clear screen with solid colors
- [x] Window resize support  
- [x] Draw filled rectangles and rounded rectangles
- [x] Draw circles
- [x] Text rendering (Limited)
- [ ] Anti-aliasing
- [ ] Gradients

## Performance

Benchmarks on [Ryzen 5 5625U]:
- Clear 1080p screen: 145.05 µs
- Draw 1000 rectangles: 1.1631 ms
- Clear 4K screen: 2.4190 ms

## Usage

See `examples/src/text_render.rs` for a complete example with text rendering.
```rust
use tiny_pao::{Canvas, Color, Position, Size};

let mut canvas = Canvas::new(Size { width: 800, height: 600 }, Color::rgb(0, 0, 0);

// Draw shapes
canvas.fill_rect(Position { x: 100, y: 100 }, Size { width: 200, height: 150 }, Color::rgb(221, 39, 33);
canvas.draw_circle(Position { x: 400, y: 300 }, Size { width: 100, height: 100 }, Color::rgb(21, 41, 219);

// Get pixels for display
let pixels = canvas.buffer();
```

## License

MIT
