//! Icon generation module - creates app icon programmatically

use image::{Rgba, RgbaImage};
use eframe::egui::IconData;

/// Generate the application icon (32x32 pixels)
#[allow(dead_code)]
pub fn generate_icon() -> RgbaImage {
    let size = 32u32;
    let mut img = RgbaImage::new(size, size);

    // Colors
    let bg_color = Rgba([59, 130, 246, 255]);       // Blue background
    let clipboard_color = Rgba([255, 255, 255, 255]); // White clipboard
    let line_color = Rgba([200, 220, 255, 255]);     // Light blue lines
    let accent_color = Rgba([34, 197, 94, 255]);     // Green accent (checkmark)

    // Fill background with rounded corners effect
    for y in 0..size {
        for x in 0..size {
            let corner_radius = 6.0f32;
            let in_corner = |cx: f32, cy: f32| -> bool {
                let dx = (x as f32 - cx).abs();
                let dy = (y as f32 - cy).abs();
                dx * dx + dy * dy > corner_radius * corner_radius
            };

            let is_corner =
                (x < 6 && y < 6 && in_corner(6.0, 6.0)) ||
                (x >= size - 6 && y < 6 && in_corner((size - 7) as f32, 6.0)) ||
                (x < 6 && y >= size - 6 && in_corner(6.0, (size - 7) as f32)) ||
                (x >= size - 6 && y >= size - 6 && in_corner((size - 7) as f32, (size - 7) as f32));

            if !is_corner {
                img.put_pixel(x, y, bg_color);
            }
        }
    }

    // Draw clipboard shape (white rectangle with clip)
    for y in 8..28 {
        for x in 7..25 {
            img.put_pixel(x, y, clipboard_color);
        }
    }

    // Clipboard clip (top)
    for y in 5..10 {
        for x in 11..21 {
            img.put_pixel(x, y, clipboard_color);
        }
    }
    // Clip hole
    for y in 6..9 {
        for x in 13..19 {
            img.put_pixel(x, y, bg_color);
        }
    }

    // Draw lines on clipboard
    for x in 9..23 {
        img.put_pixel(x, 12, line_color);
        img.put_pixel(x, 15, line_color);
        img.put_pixel(x, 18, line_color);
    }

    // Draw green checkmark
    let check_points = [
        (19, 22), (20, 23), (21, 24),
        (22, 23), (23, 22), (24, 21), (25, 20),
    ];
    for (x, y) in check_points {
        if x < size && y < size {
            img.put_pixel(x, y, accent_color);
            if x + 1 < size { img.put_pixel(x + 1, y, accent_color); }
            if y + 1 < size { img.put_pixel(x, y + 1, accent_color); }
        }
    }

    img
}

/// Generate larger icon (64x64) for better quality
pub fn generate_icon_64() -> RgbaImage {
    let size = 64u32;
    let mut img = RgbaImage::new(size, size);

    // Colors
    let bg_color = Rgba([59, 130, 246, 255]);         // Blue
    let clipboard_color = Rgba([255, 255, 255, 255]); // White
    let line_color = Rgba([180, 200, 240, 255]);      // Light blue
    let accent_color = Rgba([34, 197, 94, 255]);      // Green
    let shadow_color = Rgba([40, 100, 200, 255]);     // Darker blue

    // Fill background with rounded corners
    for y in 0..size {
        for x in 0..size {
            let corner_radius = 12.0f32;
            let in_corner = |cx: f32, cy: f32| -> bool {
                let dx = (x as f32 - cx).abs();
                let dy = (y as f32 - cy).abs();
                dx * dx + dy * dy > corner_radius * corner_radius
            };

            let is_corner =
                (x < 12 && y < 12 && in_corner(12.0, 12.0)) ||
                (x >= size - 12 && y < 12 && in_corner((size - 13) as f32, 12.0)) ||
                (x < 12 && y >= size - 12 && in_corner(12.0, (size - 13) as f32)) ||
                (x >= size - 12 && y >= size - 12 && in_corner((size - 13) as f32, (size - 13) as f32));

            if !is_corner {
                img.put_pixel(x, y, bg_color);
            }
        }
    }

    // Shadow for clipboard
    for y in 18..56 {
        for x in 16..50 {
            img.put_pixel(x, y, shadow_color);
        }
    }

    // Main clipboard body
    for y in 16..54 {
        for x in 14..48 {
            img.put_pixel(x, y, clipboard_color);
        }
    }

    // Clipboard clip (top center)
    for y in 10..20 {
        for x in 22..42 {
            img.put_pixel(x, y, clipboard_color);
        }
    }
    // Clip hole
    for y in 12..18 {
        for x in 26..38 {
            img.put_pixel(x, y, bg_color);
        }
    }

    // Draw text lines
    for line in 0..4u32 {
        let y_pos = 24 + line * 7;
        let line_width = if line == 3 { 20 } else { 28 };
        for x in 18..(18 + line_width) {
            for dy in 0..3 {
                if y_pos + dy < size {
                    img.put_pixel(x, y_pos + dy, line_color);
                }
            }
        }
    }

    // Draw sparkle/evolution effect (small stars)
    let star_color = Rgba([255, 220, 100, 255]); // Gold
    let stars = [(8u32, 20u32), (56, 15), (52, 50), (10, 48)];
    for (sx, sy) in stars {
        img.put_pixel(sx, sy, star_color);
        if sx > 0 { img.put_pixel(sx - 1, sy, star_color); }
        if sx < size - 1 { img.put_pixel(sx + 1, sy, star_color); }
        if sy > 0 { img.put_pixel(sx, sy - 1, star_color); }
        if sy < size - 1 { img.put_pixel(sx, sy + 1, star_color); }
    }

    // Green checkmark (larger)
    for i in 0..6u32 {
        let x = 38 + i;
        let y = 44 + i;
        for dx in 0..3 {
            for dy in 0..3 {
                if x + dx < size && y + dy < size {
                    img.put_pixel(x + dx, y + dy, accent_color);
                }
            }
        }
    }
    for i in 0..10u32 {
        let x = 44 + i;
        let y = 49u32.saturating_sub(i);
        for dx in 0..3 {
            for dy in 0..3 {
                if x + dx < size && y + dy < size {
                    img.put_pixel(x + dx, y + dy, accent_color);
                }
            }
        }
    }

    img
}

/// Convert RgbaImage to egui IconData
pub fn to_icon_data(img: &RgbaImage) -> IconData {
    let (width, height) = img.dimensions();
    let rgba = img.as_raw().clone();
    IconData {
        rgba,
        width,
        height,
    }
}
