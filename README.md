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

## License

MIT
