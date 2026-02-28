use std::hint::black_box;

use criterion::{Criterion, criterion_group, criterion_main};
use tiny_pao::{Canvas, Color, Position, Size};

fn render_bench(c: &mut Criterion) {
    // Clear bench
    let mut clear_group = c.benchmark_group("clear_1080p");
    clear_group.measurement_time(std::time::Duration::from_secs(10));
    clear_group.bench_function("clear_1080p", |b| {
        let mut canvas = Canvas::new(
            Size {
                width: 1920,
                height: 1080,
            },
            Color::rgb(0, 0, 0),
        );

        b.iter(|| {
            canvas.clear(black_box(Color::rgb(255, 255, 255)));
        });
    });
    clear_group.finish();

    // Draw 1000 rects and 1080p
    let mut draw_group = c.benchmark_group("draw_1000_rects_1080p");
    draw_group.measurement_time(std::time::Duration::from_secs(10));
    draw_group.bench_function("draw_1000_rects_1080p", |b| {
        let mut canvas = Canvas::new(
            Size {
                width: 1920,
                height: 1080,
            },
            Color::rgb(0, 0, 0),
        );

        b.iter(|| {
            canvas.clear(Color::rgb(0, 0, 0));

            for i in 0..1000 {
                canvas.draw_rect(
                    Position {
                        x: (i % 1000) as u32,
                        y: (i % 1000) as u32,
                    },
                    Size {
                        width: 50,
                        height: 50,
                    },
                    black_box(Color::rgb(255, 0, 0)),
                );
            }
        });
    });
    draw_group.finish();

    // 1000 Frame rects bench
    let mut stress_group = c.benchmark_group("stress_frame_1000_rects");
    stress_group.measurement_time(std::time::Duration::from_secs(10));
    stress_group.bench_function("stress_frame_1000_rects", |b| {
        let mut canvas = Canvas::new(
            Size {
                width: 1920,
                height: 1080,
            },
            Color::rgb(0, 0, 0),
        );

        b.iter(|| {
            canvas.clear(Color::rgb(255, 255, 255));

            for i in 0..1000 {
                canvas.draw_rect(
                    Position {
                        x: (i % 1000) as u32,
                        y: (i % 1000) as u32,
                    },
                    Size {
                        width: 50,
                        height: 50,
                    },
                    Color::rgb(255, 0, 0),
                );
            }

            black_box(canvas.buffer());
        });
    });
    stress_group.finish();

    // Resolutions bench
    let resolutions = [(800, 600), (1280, 720), (1920, 1080), (3840, 2160)];

    let mut res_group = c.benchmark_group("resolutions_clear");
    res_group.measurement_time(std::time::Duration::from_secs(16));

    for (w, h) in resolutions {
        let id = format!("clear_{}x{}", w, h);

        res_group.bench_function(&id, |b| {
            let mut canvas = Canvas::new(
                Size {
                    width: w,
                    height: h,
                },
                Color::rgb(0, 0, 0),
            );

            b.iter(|| {
                canvas.clear(black_box(Color::rgb(255, 255, 255)));
            });
        });
    }

    res_group.finish();
}

criterion_group!(benches, render_bench);
criterion_main!(benches);
