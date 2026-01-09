//! Icon export tool - saves icon as PNG files
//! Run with: cargo run --bin export_icon

use image::RgbaImage;

fn main() {
    println!("📦 Exporting icons...");

    let assets_dir = std::path::Path::new("assets");
    if !assets_dir.exists() {
        std::fs::create_dir_all(assets_dir).expect("Failed to create assets directory");
    }

    // Generate and save 64x64 icon
    let icon_64 = generate_icon_64();
    icon_64.save("assets/icon_64.png").expect("Failed to save 64x64 icon");
    println!("✅ Saved: assets/icon_64.png");

    // Generate and save 128x128 icon
    let icon_128 = generate_icon_128();
    icon_128.save("assets/icon_128.png").expect("Failed to save 128x128 icon");
    println!("✅ Saved: assets/icon_128.png");

    // Generate and save 256x256 icon
    let icon_256 = generate_icon_256();
    icon_256.save("assets/icon_256.png").expect("Failed to save 256x256 icon");
    println!("✅ Saved: assets/icon_256.png");

    println!("✨ All icons exported!");
}

fn generate_icon_64() -> RgbaImage {
    generate_icon(64)
}

fn generate_icon_128() -> RgbaImage {
    generate_icon(128)
}

fn generate_icon_256() -> RgbaImage {
    generate_icon(256)
}

fn generate_icon(size: u32) -> RgbaImage {
    use image::Rgba;

    let mut img = RgbaImage::new(size, size);
    let scale = size as f32 / 64.0;

    // Colors
    let bg_color = Rgba([59, 130, 246, 255]);         // Blue
    let clipboard_color = Rgba([255, 255, 255, 255]); // White
    let line_color = Rgba([180, 200, 240, 255]);      // Light blue
    let accent_color = Rgba([34, 197, 94, 255]);      // Green
    let shadow_color = Rgba([40, 100, 200, 255]);     // Darker blue
    let star_color = Rgba([255, 220, 100, 255]);      // Gold

    // Fill background with rounded corners
    let corner_radius = 12.0 * scale;
    for y in 0..size {
        for x in 0..size {
            let in_corner = |cx: f32, cy: f32| -> bool {
                let dx = (x as f32 - cx).abs();
                let dy = (y as f32 - cy).abs();
                dx * dx + dy * dy > corner_radius * corner_radius
            };

            let cr = corner_radius as u32;
            let is_corner =
                (x < cr && y < cr && in_corner(corner_radius, corner_radius)) ||
                (x >= size - cr && y < cr && in_corner((size - cr - 1) as f32, corner_radius)) ||
                (x < cr && y >= size - cr && in_corner(corner_radius, (size - cr - 1) as f32)) ||
                (x >= size - cr && y >= size - cr && in_corner((size - cr - 1) as f32, (size - cr - 1) as f32));

            if !is_corner {
                img.put_pixel(x, y, bg_color);
            }
        }
    }

    // Helper to scale coordinates
    let s = |v: u32| -> u32 { (v as f32 * scale) as u32 };
    let sf = |v: f32| -> u32 { (v * scale) as u32 };

    // Shadow for clipboard
    for y in s(18)..s(56) {
        for x in s(16)..s(50) {
            if x < size && y < size {
                img.put_pixel(x, y, shadow_color);
            }
        }
    }

    // Main clipboard body
    for y in s(16)..s(54) {
        for x in s(14)..s(48) {
            if x < size && y < size {
                img.put_pixel(x, y, clipboard_color);
            }
        }
    }

    // Clipboard clip (top center)
    for y in s(10)..s(20) {
        for x in s(22)..s(42) {
            if x < size && y < size {
                img.put_pixel(x, y, clipboard_color);
            }
        }
    }
    // Clip hole
    for y in s(12)..s(18) {
        for x in s(26)..s(38) {
            if x < size && y < size {
                img.put_pixel(x, y, bg_color);
            }
        }
    }

    // Draw text lines
    let line_height = sf(3.0);
    for line in 0..4u32 {
        let y_pos = s(24) + s(line * 7);
        let line_width = if line == 3 { s(20) } else { s(28) };
        for x in s(18)..(s(18) + line_width) {
            for dy in 0..line_height {
                if x < size && y_pos + dy < size {
                    img.put_pixel(x, y_pos + dy, line_color);
                }
            }
        }
    }

    // Draw sparkle/evolution effect (stars)
    let stars = [(8u32, 20u32), (56, 15), (52, 50), (10, 48)];
    let star_size = sf(2.0).max(1);
    for (sx, sy) in stars {
        let sx = s(sx);
        let sy = s(sy);
        for dx in 0..star_size {
            for dy in 0..star_size {
                if sx + dx < size && sy + dy < size {
                    img.put_pixel(sx + dx, sy + dy, star_color);
                }
                if sx >= dx && sy + dy < size {
                    img.put_pixel(sx - dx, sy + dy, star_color);
                }
                if sx + dx < size && sy >= dy {
                    img.put_pixel(sx + dx, sy - dy, star_color);
                }
            }
        }
    }

    // Green checkmark (larger)
    let check_thickness = sf(3.0).max(2);
    for i in 0..sf(6.0) {
        let x = s(38) + i;
        let y = s(44) + i;
        for dx in 0..check_thickness {
            for dy in 0..check_thickness {
                if x + dx < size && y + dy < size {
                    img.put_pixel(x + dx, y + dy, accent_color);
                }
            }
        }
    }
    for i in 0..sf(10.0) {
        let x = s(44) + i;
        let y = s(49).saturating_sub(i);
        for dx in 0..check_thickness {
            for dy in 0..check_thickness {
                if x + dx < size && y + dy < size {
                    img.put_pixel(x + dx, y + dy, accent_color);
                }
            }
        }
    }

    img
}
