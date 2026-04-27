///Will most likely rewrite this in the future considering I copied this from C++ code which has an
///entirely different style which isn't idiomatic Rust.
pub mod allocator;
pub mod atlas;
pub mod cont_comb;
pub mod distances;
pub mod edge_cache;
pub mod edge_coloring;
pub mod edge_select;
pub mod shape_distance_finder;
use crate::cont_comb::SimpleContourCombiner;
use crate::edge_coloring::edge_coloring_simple;
pub use allocator::*;
pub use atlas::*;
pub use edge_select::*;
use image::{ImageBuffer, Rgb};
use math::calc::median;
use parser::TtfFont;
pub use shape_distance_finder::*;
use std::{collections::HashMap, time::Instant};
const CROSS_THRESHOLD: f64 = 3.0;
pub use math::lalg::Vec2;
///Example code for generating a texture atlas.
pub fn entry() -> Atlas<char, Rgb<u8>, ShelfAllocator> {
    let mut font = TtfFont::new("../jet.ttf").unwrap();
    let atlas_allocator = ShelfAllocator::new(512, 512);
    let mut texture_atlas: Atlas<char, Rgb<u8>, ShelfAllocator> =
        Atlas::new(1024, 1024, atlas_allocator, 4);
    let target_font_px = 32;
    let dmax_px = 1.0;
    let now = Instant::now();
    for _ in 0..1_000 {
        for ch in '!'..'~' {
            let mut seed = 0;
            let gid = font.lookup(ch as u32).unwrap();
            let mut shape = font.assemble_glyf(gid as u16).unwrap();
            // shape.normalize();
            edge_coloring_simple(&mut shape, CROSS_THRESHOLD.sin(), &mut seed);
            let glyph = font.glyf.get_glyf(gid as u16).unwrap().clone();
            let bounds = glyph.get_header();
            let mut sdf: ShapeDistanceFinder<SimpleContourCombiner<MultiDistanceSelector>> =
                ShapeDistanceFinder::new(shape);
            // scale = pixels per font unit
            let scale = target_font_px as f64 / font.head.units_per_em as f64;

            // convert dMAX from pixels to font units
            let dmax = dmax_px / scale;
            let distance_range = 2.0 * dmax;
            let max_color = 255.0;

            let width = bounds.x_max - bounds.x_min;
            let height = bounds.y_max - bounds.y_min;

            let pixel_width = (width as f64 * scale).ceil().max(1.0) as u32;
            let pixel_height = (height as f64 * scale).ceil().max(1.0) as u32;

            let mut output_image: ImageBuffer<Rgb<u8>, Vec<u8>> =
                ImageBuffer::new(pixel_width, pixel_height);

            for py in 0..pixel_height {
                for px in 0..pixel_width {
                    let gx = bounds.x_min as f64 + (px as f64 + 0.5) / scale;
                    let gy = bounds.y_min as f64 + (py as f64 + 0.5) / scale;
                    let p = Vec2 { x: gx, y: gy };
                    let distance = sdf.distance(p);

                    // clamp to [-dmax, +dmax]
                    let clamped_r = distance.r.clamp(-dmax, dmax);
                    let clamped_g = distance.g.clamp(-dmax, dmax);
                    let clamped_b = distance.b.clamp(-dmax, dmax);

                    // distanceColor(d) = ((d / (2*dmax)) + 0.5) * 255
                    let r_0_255 = ((clamped_r / distance_range + 0.5) * max_color)
                        .clamp(0.0, 255.0)
                        .round() as u8;

                    let g_0_255 = ((clamped_g / distance_range + 0.5) * max_color)
                        .clamp(0.0, 255.0)
                        .round() as u8;

                    let b_0_255 = ((clamped_b / distance_range + 0.5) * max_color)
                        .clamp(0.0, 255.0)
                        .round() as u8;
                    let pixel = Rgb([r_0_255, g_0_255, b_0_255]);
                    output_image.put_pixel(px, pixel_height - 1 - py, pixel);
                }
            }
            // texture_atlas.add_image(ch, &output_image).unwrap();
            // output_image.save(format!("./res/{}.png", ch)).unwrap();
        }
    }
    println!("Took {} ms to complete", now.elapsed().as_millis());
    texture_atlas.image.save("../texture_atlas.png").unwrap();
    texture_atlas
}
///Returns a HashMap of chars and the corresponding image.
pub fn no_atlas(
    glyphs: &[char],
    font: &mut TtfFont,
    target_font_px: u16,
) -> HashMap<char, ImageBuffer<Rgb<u8>, Vec<u8>>> {
    let mut map = HashMap::new();
    let mut seed = 0;
    for ch in glyphs {
        let gid = font.lookup(*ch as u32).unwrap();
        let mut shape = font.assemble_glyf(gid as u16).unwrap();
        edge_coloring_simple(&mut shape, CROSS_THRESHOLD.sin(), &mut seed);
        let glyph = font.glyf.get_glyf(gid as u16).unwrap().clone();
        let bounds = glyph.get_header();
        let mut sdf: ShapeDistanceFinder<SimpleContourCombiner<MultiDistanceSelector>> =
            ShapeDistanceFinder::new(shape);
        let scale = target_font_px as f64 / font.head.units_per_em as f64;
        let width = bounds.x_max - bounds.x_min;
        let height = bounds.y_max - bounds.y_min;
        let pixel_width = (width as f64 * scale).ceil().max(1.0) as u32;
        let pixel_height = (height as f64 * scale).ceil().max(1.0) as u32;
        let mut output_image: ImageBuffer<Rgb<u8>, Vec<u8>> =
            ImageBuffer::new(pixel_width, pixel_height);
        for py in 0..pixel_height {
            for px in 0..pixel_width {
                let gx = bounds.x_min as f64 + px as f64 / scale;
                let gy = bounds.y_min as f64 + py as f64 / scale;
                let p = Vec2 { x: gx, y: gy };
                let distance = sdf.distance(p);
                let clamped_r = distance.r.clamp(-127.0, 128.0);
                let clamped_g = distance.g.clamp(-127.0, 128.0);
                let clamped_b = distance.b.clamp(-127.0, 128.0);
                let r_0_255 = (clamped_r + 127.0).round() as u8;
                let g_0_255 = (clamped_g + 127.0).round() as u8;
                let b_0_255 = (clamped_b + 127.0).round() as u8;
                let res = median(r_0_255, g_0_255, b_0_255);
                let pixel = Rgb([res; 3]);
                output_image.put_pixel(px, py, pixel);
            }
        }
        map.insert(*ch, output_image);
    }
    map
}

///Single use.
pub fn oneshot(
    glyphs: &[char],
    font: &mut TtfFont,
    target_font_px: u16,
    dims: (u32, u32),
) -> Result<Atlas<char, Rgb<u8>, MaxRectsAllocator>, &'static str> {
    let allocator = MaxRectsAllocator::new(dims.0, dims.1);
    let mut atlas: Atlas<char, Rgb<u8>, MaxRectsAllocator> =
        Atlas::new(dims.0, dims.1, allocator, 0);
    let mut seed = 0;
    for ch in glyphs {
        let gid = font.lookup(*ch as u32).unwrap();
        let mut shape = font.assemble_glyf(gid as u16).unwrap();
        edge_coloring_simple(&mut shape, CROSS_THRESHOLD.sin(), &mut seed);
        let glyph = font.glyf.get_glyf(gid as u16).unwrap().clone();
        let bounds = glyph.get_header();
        let mut sdf: ShapeDistanceFinder<SimpleContourCombiner<MultiDistanceSelector>> =
            ShapeDistanceFinder::new(shape);
        let scale = target_font_px as f64 / font.head.units_per_em as f64;
        let width = bounds.x_max - bounds.x_min;
        let height = bounds.y_max - bounds.y_min;
        let pixel_width = (width as f64 * scale).ceil().max(1.0) as u32;
        let pixel_height = (height as f64 * scale).ceil().max(1.0) as u32;
        let mut output_image: ImageBuffer<Rgb<u8>, Vec<u8>> =
            ImageBuffer::new(pixel_width, pixel_height);
        for py in 0..pixel_height {
            for px in 0..pixel_width {
                let gx = bounds.x_min as f64 + px as f64 / scale;
                let gy = bounds.y_min as f64 + py as f64 / scale;
                let p = Vec2 { x: gx, y: gy };
                let distance = sdf.distance(p);

                let clamped_r = distance.r.clamp(-127.0, 128.0);
                let clamped_g = distance.g.clamp(-127.0, 128.0);
                let clamped_b = distance.b.clamp(-127.0, 128.0);
                let r_0_255 = (clamped_r + 127.0).round() as u8;
                let g_0_255 = (clamped_g + 127.0).round() as u8;
                let b_0_255 = (clamped_b + 127.0).round() as u8;
                let pixel = Rgb([r_0_255, g_0_255, b_0_255]);
                output_image.put_pixel(px, py, pixel);
            }
        }
        atlas.stage(*ch, output_image);
    }
    let flush = atlas.flush();
    if !flush.failed.is_empty() {
        Err("Not enough space was allocated for ")
    } else {
        Ok(atlas)
    }
}
