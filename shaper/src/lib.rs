use parser::{Font, FontTypes, gsub::ligature::Ligature};
fn decode_utf8(text: &str) -> Vec<u32> {
    text.chars().map(|c| c as u32).collect()
}
///FontId is the key into a hasmap containing multiple fonts.  Allows passing in of different font
///types.  
pub struct TextRun<'a> {
    pub text: &'a str,
    pub font_id: u16,
    pub size: f32,
}
#[derive(Clone, Copy)]
pub struct GlyphInfo {
    pub glyph_id: u16,
    pub cluster: u32, // original char index
}
#[derive(Clone, Copy)]
pub struct GlyphPosition {
    pub x_advance: f32,
    pub y_advance: f32,
    pub x_offset: f32,
    pub y_offset: f32,
}
pub struct GlyphBuffer {
    pub infos: Vec<GlyphInfo>,
    pub positions: Vec<GlyphPosition>,
}
///Holds config opts, like language type, etc.
pub struct Shaper<'a> {
    font: &'a FontTypes,
}
impl<'a> Shaper<'a> {
    pub fn shape(&self, text: &str) -> GlyphBuffer {
        let codepoints = decode_utf8(text);

        let mut buffer = self.map_to_glyphs(&codepoints);
        // self.apply_gsub(&mut buffer);
        // self.apply_gpos(&mut buffer);
        buffer
    }
    fn map_to_glyphs(&self, cps: &[u32]) -> GlyphBuffer {
        let mut infos = Vec::with_capacity(cps.len());
        let mut positions = Vec::with_capacity(cps.len());

        for (i, &cp) in cps.iter().enumerate() {
            let gid = self.font.glyph_index(cp);

            infos.push(GlyphInfo {
                glyph_id: gid,
                cluster: i as u32,
            });

            positions.push(GlyphPosition {
                x_advance: self.font.h_advance(gid),
                y_advance: 0.0,
                x_offset: 0.0,
                y_offset: 0.0,
            });
        }

        GlyphBuffer { infos, positions }
    }
    fn apply_gpos(&self, buffer: &mut GlyphBuffer) {
        if let Some(gpos) = self.font.gpos() {
            for i in 0..buffer.infos.len() - 1 {
                let left = buffer.infos[i].glyph_id;
                let right = buffer.infos[i + 1].glyph_id;

                if let Some(kern) = gpos.get_pair_adjustment(left, right) {
                    buffer.positions[i].x_advance += kern;
                }
            }
        }
    }
    fn apply_gsub(&self, buffer: &mut GlyphBuffer) {
        if let Some(gsub) = self.font.gsub() {
            for lookup in &gsub.lookups {
                match lookup {
                    Lookup::Ligature(lig) => {
                        apply_ligatures(buffer, lig);
                    }
                    _ => {}
                }
            }
        }
    }
}
fn apply_ligatures(buffer: &GlyphBuffer, lig: Ligature) {}
pub struct PositionedGlyph {
    pub glyph_id: u16,
    pub x: f32,
    pub y: f32,
}
