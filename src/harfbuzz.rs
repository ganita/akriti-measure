// Copyright 2017 Sreejith Krishnan R <sreejith@ganita.io>

use std::ffi::{CStr, CString};
use std::ptr;
use std::cmp::PartialEq;
use std::slice;

use ::harfbuzz_sys;

pub fn hb_version_string() -> String {
    let version = unsafe { CStr::from_ptr(harfbuzz_sys::hb_version_string()) };
    String::from(version.to_str().expect("Harbuzz not linked"))
}

pub struct HBFace {
    face: *mut harfbuzz_sys::hb_face_t,
    font: *mut harfbuzz_sys::hb_font_t,
}

#[cfg(any(target_os="ios", target_os="macos"))]
pub enum CGFont {}
#[cfg(any(target_os="ios", target_os="macos"))]
pub type CGFontRef = *mut CGFont;

pub enum FTFont {}
pub type FTFontRef = *mut FTFont;

#[derive(Debug)]
pub struct HBGlyphVariant {
    data: harfbuzz_sys::hb_ot_math_glyph_variant_t
}

impl PartialEq for HBGlyphVariant {
    fn eq(&self, other: &HBGlyphVariant) -> bool {
        self.glyph_index() == other.glyph_index() && self.advance() == other.advance()
    }
}

impl HBGlyphVariant {
    fn new(data: harfbuzz_sys::hb_ot_math_glyph_variant_t) -> HBGlyphVariant {
        HBGlyphVariant {
            data
        }
    }

    pub fn glyph_index(&self) -> u32 {
        self.data.glyph
    }

    pub fn advance(&self) -> i32 {
        self.data.advance
    }
}

pub enum HBDirection {
    LTR,
    RTL,
    TTB,
    BTT
}

impl HBDirection {
    fn to_hb_dir(&self) -> harfbuzz_sys::hb_direction_t {
        match *self {
            HBDirection::LTR => harfbuzz_sys::hb_direction_t::HB_DIRECTION_LTR,
            HBDirection::RTL => harfbuzz_sys::hb_direction_t::HB_DIRECTION_RTL,
            HBDirection::TTB => harfbuzz_sys::hb_direction_t::HB_DIRECTION_TTB,
            HBDirection::BTT => harfbuzz_sys::hb_direction_t::HB_DIRECTION_BTT,
        }
    }
}

pub struct HBGlyphVariantIter<'a> {
    size: u32,
    index: u32,
    glyph_index: u32,
    direction: HBDirection,
    face: &'a HBFace
}

impl<'a> Iterator for HBGlyphVariantIter<'a> {
    type Item = HBGlyphVariant;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.size {
            return None;
        }

        let mut variants: [harfbuzz_sys::hb_ot_math_glyph_variant_t; 1] = [harfbuzz_sys::hb_ot_math_glyph_variant_t { glyph: 0, advance: 0 }];
        unsafe {
            harfbuzz_sys::hb_ot_math_get_glyph_variants(
                self.face.font,
                self.glyph_index,
                self.direction.to_hb_dir(),
                self.index,
                &mut 1,
                variants.as_mut_ptr(),
            )
        };
        self.index = self.index+1;

        Some(HBGlyphVariant::new(variants[0]))
    }
}

impl<'a> HBGlyphVariantIter<'a> {
    fn new(face: &'a HBFace, glyph_index: u32, direction: HBDirection) -> HBGlyphVariantIter<'a> {
        let num_variants = unsafe {
            harfbuzz_sys::hb_ot_math_get_glyph_variants(
                face.font,
                glyph_index,
                direction.to_hb_dir(),
                0,
                &mut 0,
                ptr::null_mut(),
            )
        };

        HBGlyphVariantIter {
            size: num_variants,
            index: 0,
            glyph_index,
            direction,
            face
        }
    }

    pub fn len(&self) -> u32 {
        self.size
    }
}

#[derive(Debug)]
pub struct HBGlyphPart {
    data: harfbuzz_sys::hb_ot_math_glyph_part_t
}


impl HBGlyphPart {
    pub fn glyph_index(&self) -> u32 {
        self.data.glyph
    }

    pub fn start_connector_length(&self) -> i32 {
        self.data.start_connector_length
    }

    pub fn end_connector_length(&self) -> i32 {
        self.data.end_connector_length
    }

    pub fn full_advance(&self) -> i32 {
        self.data.full_advance
    }

    pub fn is_extender(&self) -> bool {
        self.data.flags == harfbuzz_sys::hb_ot_math_glyph_part_flags_t::HB_MATH_GLYPH_PART_FLAG_EXTENDER
    }
}

#[derive(Debug)]
pub struct HBGlyphAssembly {
    parts: Vec<HBGlyphPart>,
    italics_correction: i32,
}

impl HBGlyphAssembly {
    pub fn len(&self) -> usize {
        self.parts.len()
    }

    pub fn italics_correction(&self) -> i32 {
        self.italics_correction
    }

    pub fn parts(&self) -> &Vec<HBGlyphPart> {
        &self.parts
    }
}

#[derive(Debug)]
pub struct HBGlyphPosition {
    x_advance: i32,
    y_advance: i32,
    x_offset: i32,
    y_offset: i32,
}

impl HBGlyphPosition {
    fn new(data: &harfbuzz_sys::hb_glyph_position_t) -> HBGlyphPosition {
        HBGlyphPosition {
            x_advance: data.x_advance,
            y_advance: data.y_advance,
            x_offset: data.x_offset,
            y_offset: data.y_offset
        }
    }

    pub fn x_advance(&self) -> i32 {
        self.x_advance
    }

    pub fn y_advance(&self) -> i32 {
        self.y_advance
    }

    pub fn x_offset(&self) -> i32 {
        self.x_offset
    }

    pub fn y_offset(&self) -> i32 {
        self.y_offset
    }
}

#[derive(Debug)]
pub struct HBGlyphPositions {
    positions: Vec<HBGlyphPosition>,
    width: i32,
    height: i32,
}

impl HBGlyphPositions {
    fn new(positions: Vec<HBGlyphPosition>) -> HBGlyphPositions {
        let mut width = 0;
        let mut height = 0;
        for pos in &positions {
            width += pos.x_advance();
            height += pos.y_advance();
        }
        width += positions[positions.len()-1].x_offset();
        height += positions[positions.len()-1].y_offset();
        HBGlyphPositions { positions, width, height }
    }

    pub fn width(&self) -> i32 {
        self.width
    }

    pub fn height(&self) -> i32 {
        self.height
    }

    pub fn positions(&self) -> &Vec<HBGlyphPosition> {
        &self.positions
    }
}

impl HBFace {

    #[cfg(any(target_os="ios", target_os="macos"))]
    pub fn from_cg_font(cg_font: CGFontRef) -> HBFace {
        if cg_font.is_null() {
            panic!("Attempt to create font with null CGFontRef");
        }

        let face = unsafe { hb_coretext_face_create(cg_font) };
        let font = unsafe { harfbuzz_sys::hb_font_create(face) };
        HBFace {
            face,
            font
        }
    }

    pub fn from_freetype_font(ft_ref: FTFontRef) -> HBFace {
        if ft_ref.is_null() {
            panic!("Attempt to create font with null FTFontRef");
        }

        let face = unsafe { hb_ft_face_create_referenced(ft_ref) };
        let font = unsafe { hb_ft_font_create_referenced(ft_ref) };
        HBFace {
            face,
            font
        }
    }

    pub fn index(&self) -> u32 {
        return unsafe { harfbuzz_sys::hb_face_get_index(self.face) }
    }

    pub fn upem(&self) -> u32 {
        return unsafe { harfbuzz_sys::hb_face_get_upem(self.face) }
    }

    pub fn glyph_count(&self) -> u32 {
        return unsafe { harfbuzz_sys::hb_face_get_glyph_count(self.face) }
    }

    pub fn glyph_index(&self, unicode: u32) -> Option<u32> {
        unsafe {
            let mut glyph: harfbuzz_sys::hb_codepoint_t = 0;
            let have_glyph = harfbuzz_sys::hb_font_get_nominal_glyph(self.font, unicode, &mut glyph);
            if have_glyph != 0 {
                return Some(glyph);
            }
            return None
        }
    }

    pub fn ascent(&self) -> i32 {
        self.extends().ascender
    }

    pub fn descent(&self) -> i32 {
        self.extends().descender
    }

    fn extends(&self) -> harfbuzz_sys::hb_font_extents_t {
        let mut extends = harfbuzz_sys::hb_font_extents_t {
            ascender: 0,
            descender: 0,
            line_gap: 0,
            reserved1: 0,
            reserved2: 0,
            reserved3: 0,
            reserved4: 0,
            reserved5: 0,
            reserved6: 0,
            reserved7: 0,
            reserved8: 0,
            reserved9: 0,
        };
        unsafe {
            harfbuzz_sys::hb_font_get_h_extents(
                self.font,
                &mut extends
            );
        };

        return extends;
    }

    pub fn measure(&self, text: String, direction: &HBDirection) -> HBGlyphPositions {
        let char_len = text.chars().count() as  i32;
        let c_str = CString::new(text).unwrap();
        let buffer = unsafe { harfbuzz_sys::hb_buffer_create() };
        let hb_dir = direction.to_hb_dir();
        unsafe {
            harfbuzz_sys::hb_buffer_set_direction(buffer, hb_dir);
            harfbuzz_sys::hb_buffer_add_utf8(
                buffer,
                c_str.as_ptr(),
                char_len,
                0,
                char_len
            );
            harfbuzz_sys::hb_shape(self.font, buffer, ptr::null_mut(), 0);
        }

        let mut num_glyphs: u32 = 0;
        let glyph_positions = unsafe { harfbuzz_sys::hb_buffer_get_glyph_positions(buffer, &mut num_glyphs) };

        let glyph_positions = unsafe { slice::from_raw_parts(glyph_positions, num_glyphs as usize) };

        let mut positions = Vec::with_capacity(glyph_positions.len());
        for pos in glyph_positions {
            positions.push(HBGlyphPosition::new(pos));
        }

        unsafe {
            harfbuzz_sys::hb_buffer_destroy(buffer);
        }

        HBGlyphPositions::new(positions)

    }

    pub fn has_ot_math_table(&self) -> bool {
        return unsafe { harfbuzz_sys::hb_ot_math_has_data(self.face) } != 0;
    }

    pub fn italics_correction(&self, glyph_index: u32) -> i32 {
        return unsafe { harfbuzz_sys::hb_ot_math_get_glyph_italics_correction(self.font, glyph_index) }
    }

    pub fn top_accent_attachment(&self, glyph_index: u32) -> i32 {
        return unsafe { harfbuzz_sys::hb_ot_math_get_glyph_top_accent_attachment(self.font, glyph_index) }
    }

    pub fn is_glyph_extended_shape(&self, glyph_index: u32) -> bool {
        return unsafe { harfbuzz_sys::hb_ot_math_is_glyph_extended_shape(self.face, glyph_index) != 0 }
    }

    pub fn glyph_kerning_top_right(&self, glyph_index: u32, correction_height: i32) -> i32 {
        return self.glyph_kerning(glyph_index, correction_height, harfbuzz_sys::hb_ot_math_kern_t::HB_OT_MATH_KERN_TOP_RIGHT)
    }

    pub fn glyph_kerning_top_left(&self, glyph_index: u32, correction_height: i32) -> i32 {
        return self.glyph_kerning(glyph_index, correction_height, harfbuzz_sys::hb_ot_math_kern_t::HB_OT_MATH_KERN_TOP_LEFT)
    }

    pub fn glyph_kerning_bottom_right(&self, glyph_index: u32, correction_height: i32) -> i32 {
        return self.glyph_kerning(glyph_index, correction_height, harfbuzz_sys::hb_ot_math_kern_t::HB_OT_MATH_KERN_BOTTOM_RIGHT)
    }

    pub fn glyph_kerning_bottom_left(&self, glyph_index: u32, correction_height: i32) -> i32 {
        return self.glyph_kerning(glyph_index, correction_height, harfbuzz_sys::hb_ot_math_kern_t::HB_OT_MATH_KERN_BOTTOM_LEFT)
    }

    fn glyph_kerning(&self, glyph_index: u32, correction_height: i32, kern: harfbuzz_sys::hb_ot_math_kern_t) -> i32 {
        return unsafe { harfbuzz_sys::hb_ot_math_get_glyph_kerning(self.font, glyph_index, kern, correction_height) }
    }

    pub fn glyph_variants<'a>(&'a self, glyph_index: u32, direction: HBDirection) -> HBGlyphVariantIter<'a> {
        HBGlyphVariantIter::new(self, glyph_index, direction)
    }

    pub fn glyph_assembly(&self, glyph_index: u32, direction: HBDirection) -> HBGlyphAssembly {
        let mut italics_correction: i32 = 0;
        let count = unsafe {
            harfbuzz_sys::hb_ot_math_get_glyph_assembly(
                self.font,
                glyph_index,
                direction.to_hb_dir(),
                0,
                &mut 0,
                ptr::null_mut(),
                &mut italics_correction
            )
        };

        let mut vec = Vec::with_capacity(count as usize);
        let mut read: u32 = 0;
        while read < count {
            let mut data: [harfbuzz_sys::hb_ot_math_glyph_part_t; 1] = [
                harfbuzz_sys::hb_ot_math_glyph_part_t {
                    glyph: 0,
                    start_connector_length: 0,
                    end_connector_length: 0,
                    full_advance: 0,
                    flags: harfbuzz_sys::hb_ot_math_glyph_part_flags_t::HB_MATH_GLYPH_PART_FLAG_UNKNOWN,
                }
            ];
            unsafe {
                harfbuzz_sys::hb_ot_math_get_glyph_assembly(
                    self.font,
                    glyph_index,
                    direction.to_hb_dir(),
                    read,
                    &mut 1,
                    data.as_mut_ptr(),
                    &mut italics_correction
                );
            };
            vec.push(HBGlyphPart{ data: data[0] });

            read = read+1;
        }
        HBGlyphAssembly { parts: vec, italics_correction }
    }

    fn math_constant(&self, constant: harfbuzz_sys::hb_ot_math_constant_t) -> i32 {
        return unsafe { harfbuzz_sys::hb_ot_math_get_constant(self.font, constant) }
    }

    pub fn script_percent_scale_down(&self) -> i32 {
        return self.math_constant(harfbuzz_sys::hb_ot_math_constant_t::HB_OT_MATH_CONSTANT_SCRIPT_PERCENT_SCALE_DOWN);
    }
    
    pub fn script_script_percent_scale_down(&self) -> i32 {
        return self.math_constant(harfbuzz_sys::hb_ot_math_constant_t::HB_OT_MATH_CONSTANT_SCRIPT_SCRIPT_PERCENT_SCALE_DOWN);
    }
    
    pub fn delimited_sub_formula_min_height(&self) -> i32 {
        return self.math_constant(harfbuzz_sys::hb_ot_math_constant_t::HB_OT_MATH_CONSTANT_DELIMITED_SUB_FORMULA_MIN_HEIGHT);
    }
    
    pub fn display_operator_min_height(&self) -> i32 {
        return self.math_constant(harfbuzz_sys::hb_ot_math_constant_t::HB_OT_MATH_CONSTANT_DISPLAY_OPERATOR_MIN_HEIGHT);
    }
    
    pub fn math_leading(&self) -> i32 {
        return self.math_constant(harfbuzz_sys::hb_ot_math_constant_t::HB_OT_MATH_CONSTANT_MATH_LEADING);
    }
    pub fn axis_height(&self) -> i32 {
        return self.math_constant(harfbuzz_sys::hb_ot_math_constant_t::HB_OT_MATH_CONSTANT_AXIS_HEIGHT);
    }

    pub fn accent_base_height(&self) -> i32 {
        return self.math_constant(harfbuzz_sys::hb_ot_math_constant_t::HB_OT_MATH_CONSTANT_ACCENT_BASE_HEIGHT);
    }

    pub fn flattened_accent_base_height(&self) -> i32 {
        return self.math_constant(harfbuzz_sys::hb_ot_math_constant_t::HB_OT_MATH_CONSTANT_FLATTENED_ACCENT_BASE_HEIGHT);
    }

    pub fn subscript_shift_down(&self) -> i32 {
        return self.math_constant(harfbuzz_sys::hb_ot_math_constant_t::HB_OT_MATH_CONSTANT_SUBSCRIPT_SHIFT_DOWN);
    }

    pub fn subscript_top_max(&self) -> i32 {
        return self.math_constant(harfbuzz_sys::hb_ot_math_constant_t::HB_OT_MATH_CONSTANT_SUBSCRIPT_TOP_MAX);
    }

    pub fn subscript_baseline_drop_min(&self) -> i32 {
        return self.math_constant(harfbuzz_sys::hb_ot_math_constant_t::HB_OT_MATH_CONSTANT_SUBSCRIPT_BASELINE_DROP_MIN);
    }

    pub fn superscript_shift_up(&self) -> i32 {
        return self.math_constant(harfbuzz_sys::hb_ot_math_constant_t::HB_OT_MATH_CONSTANT_SUPERSCRIPT_SHIFT_UP);
    }

    pub fn superscript_shift_up_cramped(&self) -> i32 {
        return self.math_constant(harfbuzz_sys::hb_ot_math_constant_t::HB_OT_MATH_CONSTANT_SUPERSCRIPT_SHIFT_UP_CRAMPED);
    }

    pub fn superscript_bottom_min(&self) -> i32 {
        return self.math_constant(harfbuzz_sys::hb_ot_math_constant_t::HB_OT_MATH_CONSTANT_SUPERSCRIPT_BOTTOM_MIN);
    }

    pub fn superscript_baseline_drop_max(&self) -> i32 {
        return self.math_constant(harfbuzz_sys::hb_ot_math_constant_t::HB_OT_MATH_CONSTANT_SUPERSCRIPT_BASELINE_DROP_MAX);
    }

    pub fn sub_superscript_gap_min(&self) -> i32 {
        return self.math_constant(harfbuzz_sys::hb_ot_math_constant_t::HB_OT_MATH_CONSTANT_SUB_SUPERSCRIPT_GAP_MIN);
    }

    pub fn superscript_bottom_max_with_subscript(&self) -> i32 {
        return self.math_constant(harfbuzz_sys::hb_ot_math_constant_t::HB_OT_MATH_CONSTANT_SUPERSCRIPT_BOTTOM_MAX_WITH_SUBSCRIPT);
    }

    pub fn space_after_script(&self) -> i32 {
        return self.math_constant(harfbuzz_sys::hb_ot_math_constant_t::HB_OT_MATH_CONSTANT_SPACE_AFTER_SCRIPT);
    }

    pub fn upper_limit_gap_min(&self) -> i32 {
        return self.math_constant(harfbuzz_sys::hb_ot_math_constant_t::HB_OT_MATH_CONSTANT_UPPER_LIMIT_GAP_MIN);
    }

    pub fn upper_limit_baseline_rise_min(&self) -> i32 {
        return self.math_constant(harfbuzz_sys::hb_ot_math_constant_t::HB_OT_MATH_CONSTANT_UPPER_LIMIT_BASELINE_RISE_MIN);
    }

    pub fn lower_limit_gap_min(&self) -> i32 {
        return self.math_constant(harfbuzz_sys::hb_ot_math_constant_t::HB_OT_MATH_CONSTANT_LOWER_LIMIT_GAP_MIN);
    }

    pub fn lower_limit_baseline_drop_min(&self) -> i32 {
        return self.math_constant(harfbuzz_sys::hb_ot_math_constant_t::HB_OT_MATH_CONSTANT_LOWER_LIMIT_BASELINE_DROP_MIN);
    }

    pub fn stack_top_shift_up(&self) -> i32 {
        return self.math_constant(harfbuzz_sys::hb_ot_math_constant_t::HB_OT_MATH_CONSTANT_STACK_TOP_SHIFT_UP);
    }

    pub fn stack_top_display_style_shift_up(&self) -> i32 {
        return self.math_constant(harfbuzz_sys::hb_ot_math_constant_t::HB_OT_MATH_CONSTANT_STACK_TOP_DISPLAY_STYLE_SHIFT_UP);
    }

    pub fn stack_bottom_shift_down(&self) -> i32 {
        return self.math_constant(harfbuzz_sys::hb_ot_math_constant_t::HB_OT_MATH_CONSTANT_STACK_BOTTOM_SHIFT_DOWN);
    }

    pub fn stack_bottom_display_style_shift_down(&self) -> i32 {
        return self.math_constant(harfbuzz_sys::hb_ot_math_constant_t::HB_OT_MATH_CONSTANT_STACK_BOTTOM_DISPLAY_STYLE_SHIFT_DOWN);
    }

    pub fn stack_gap_min(&self) -> i32 {
        return self.math_constant(harfbuzz_sys::hb_ot_math_constant_t::HB_OT_MATH_CONSTANT_STACK_GAP_MIN);
    }

    pub fn stack_display_style_gap_min(&self) -> i32 {
        return self.math_constant(harfbuzz_sys::hb_ot_math_constant_t::HB_OT_MATH_CONSTANT_STACK_DISPLAY_STYLE_GAP_MIN);
    }

    pub fn stretch_stack_top_shift_up(&self) -> i32 {
        return self.math_constant(harfbuzz_sys::hb_ot_math_constant_t::HB_OT_MATH_CONSTANT_STRETCH_STACK_TOP_SHIFT_UP);
    }

    pub fn stretch_stack_bottom_shift_down(&self) -> i32 {
        return self.math_constant(harfbuzz_sys::hb_ot_math_constant_t::HB_OT_MATH_CONSTANT_STRETCH_STACK_BOTTOM_SHIFT_DOWN);
    }

    pub fn stretch_stack_gap_above_min(&self) -> i32 {
        return self.math_constant(harfbuzz_sys::hb_ot_math_constant_t::HB_OT_MATH_CONSTANT_STRETCH_STACK_GAP_ABOVE_MIN);
    }

    pub fn stretch_stack_gap_below_min(&self) -> i32 {
        return self.math_constant(harfbuzz_sys::hb_ot_math_constant_t::HB_OT_MATH_CONSTANT_STRETCH_STACK_GAP_BELOW_MIN);
    }

    pub fn fraction_numerator_shift_up(&self) -> i32 {
        return self.math_constant(harfbuzz_sys::hb_ot_math_constant_t::HB_OT_MATH_CONSTANT_FRACTION_NUMERATOR_SHIFT_UP);
    }

    pub fn fraction_numerator_display_style_shift_up(&self) -> i32 {
        return self.math_constant(harfbuzz_sys::hb_ot_math_constant_t::HB_OT_MATH_CONSTANT_FRACTION_NUMERATOR_DISPLAY_STYLE_SHIFT_UP);
    }

    pub fn fraction_denominator_shift_down(&self) -> i32 {
        return self.math_constant(harfbuzz_sys::hb_ot_math_constant_t::HB_OT_MATH_CONSTANT_FRACTION_DENOMINATOR_SHIFT_DOWN);
    }

    pub fn fraction_denominator_display_style_shift_down(&self) -> i32 {
        return self.math_constant(harfbuzz_sys::hb_ot_math_constant_t::HB_OT_MATH_CONSTANT_FRACTION_DENOMINATOR_DISPLAY_STYLE_SHIFT_DOWN);
    }

    pub fn fraction_numerator_gap_min(&self) -> i32 {
        return self.math_constant(harfbuzz_sys::hb_ot_math_constant_t::HB_OT_MATH_CONSTANT_FRACTION_NUMERATOR_GAP_MIN);
    }

    pub fn fraction_num_display_style_gap_min(&self) -> i32 {
        return self.math_constant(harfbuzz_sys::hb_ot_math_constant_t::HB_OT_MATH_CONSTANT_FRACTION_NUM_DISPLAY_STYLE_GAP_MIN);
    }

    pub fn fraction_rule_thickness(&self) -> i32 {
        return self.math_constant(harfbuzz_sys::hb_ot_math_constant_t::HB_OT_MATH_CONSTANT_FRACTION_RULE_THICKNESS);
    }

    pub fn fraction_denominator_gap_min(&self) -> i32 {
        return self.math_constant(harfbuzz_sys::hb_ot_math_constant_t::HB_OT_MATH_CONSTANT_FRACTION_DENOMINATOR_GAP_MIN);
    }

    pub fn fraction_denominator_display_style_gap_min(&self) -> i32 {
        return self.math_constant(harfbuzz_sys::hb_ot_math_constant_t::HB_OT_MATH_CONSTANT_FRACTION_DENOM_DISPLAY_STYLE_GAP_MIN);
    }

    pub fn skewed_fraction_horizontal_gap(&self) -> i32 {
        return self.math_constant(harfbuzz_sys::hb_ot_math_constant_t::HB_OT_MATH_CONSTANT_SKEWED_FRACTION_HORIZONTAL_GAP);
    }

    pub fn skewed_fraction_vertical_gap(&self) -> i32 {
        return self.math_constant(harfbuzz_sys::hb_ot_math_constant_t::HB_OT_MATH_CONSTANT_SKEWED_FRACTION_VERTICAL_GAP);
    }

    pub fn overbar_vertical_gap(&self) -> i32 {
        return self.math_constant(harfbuzz_sys::hb_ot_math_constant_t::HB_OT_MATH_CONSTANT_OVERBAR_VERTICAL_GAP);
    }

    pub fn overbar_rule_thickness(&self) -> i32 {
        return self.math_constant(harfbuzz_sys::hb_ot_math_constant_t::HB_OT_MATH_CONSTANT_OVERBAR_RULE_THICKNESS);
    }

    pub fn overbar_extra_ascender(&self) -> i32 {
        return self.math_constant(harfbuzz_sys::hb_ot_math_constant_t::HB_OT_MATH_CONSTANT_OVERBAR_EXTRA_ASCENDER);
    }

    pub fn underbar_vertical_gap(&self) -> i32 {
        return self.math_constant(harfbuzz_sys::hb_ot_math_constant_t::HB_OT_MATH_CONSTANT_UNDERBAR_VERTICAL_GAP);
    }

    pub fn underbar_rule_thickness(&self) -> i32 {
        return self.math_constant(harfbuzz_sys::hb_ot_math_constant_t::HB_OT_MATH_CONSTANT_UNDERBAR_RULE_THICKNESS);
    }

    pub fn underbar_extra_descender(&self) -> i32 {
        return self.math_constant(harfbuzz_sys::hb_ot_math_constant_t::HB_OT_MATH_CONSTANT_UNDERBAR_EXTRA_DESCENDER);
    }

    pub fn radical_vertical_gap(&self) -> i32 {
        return self.math_constant(harfbuzz_sys::hb_ot_math_constant_t::HB_OT_MATH_CONSTANT_RADICAL_VERTICAL_GAP);
    }

    pub fn radical_display_style_vertical_gap(&self) -> i32 {
        return self.math_constant(harfbuzz_sys::hb_ot_math_constant_t::HB_OT_MATH_CONSTANT_RADICAL_DISPLAY_STYLE_VERTICAL_GAP);
    }

    pub fn radical_rule_thickness(&self) -> i32 {
        return self.math_constant(harfbuzz_sys::hb_ot_math_constant_t::HB_OT_MATH_CONSTANT_RADICAL_RULE_THICKNESS);
    }

    pub fn radical_extra_ascender(&self) -> i32 {
        return self.math_constant(harfbuzz_sys::hb_ot_math_constant_t::HB_OT_MATH_CONSTANT_RADICAL_EXTRA_ASCENDER);
    }

    pub fn radical_kern_before_degree(&self) -> i32 {
        return self.math_constant(harfbuzz_sys::hb_ot_math_constant_t::HB_OT_MATH_CONSTANT_RADICAL_KERN_BEFORE_DEGREE);
    }

    pub fn radical_kern_after_degree(&self) -> i32 {
        return self.math_constant(harfbuzz_sys::hb_ot_math_constant_t::HB_OT_MATH_CONSTANT_RADICAL_KERN_AFTER_DEGREE);
    }

    pub fn radical_degree_bottom_raise_percent(&self) -> i32 {
        return self.math_constant(harfbuzz_sys::hb_ot_math_constant_t::HB_OT_MATH_CONSTANT_RADICAL_DEGREE_BOTTOM_RAISE_PERCENT);
    }

    pub fn min_connector_overlap(&self, direction: harfbuzz_sys::hb_direction_t) -> i32 {
        return unsafe { harfbuzz_sys::hb_ot_math_get_min_connector_overlap(self.font, direction) };
    }

    pub fn min_connector_overlap_vertical(&self) -> i32 {
        return self.min_connector_overlap(harfbuzz_sys::hb_direction_t::HB_DIRECTION_TTB);
    }

    pub fn min_connector_overlap_horizontal(&self) -> i32 {
        return self.min_connector_overlap(harfbuzz_sys::hb_direction_t::HB_DIRECTION_LTR);
    }
}

impl Drop for HBFace {
    fn drop(&mut self) {
        unsafe {
            harfbuzz_sys::hb_face_destroy(self.face);
            harfbuzz_sys::hb_font_destroy(self.font);
        }
    }
}

extern {
    #[cfg(any(target_os="ios", target_os="macos"))]    
    fn hb_coretext_face_create(reference: CGFontRef) -> *mut harfbuzz_sys::hb_face_t;

    fn hb_ft_face_create_referenced(reference: FTFontRef) -> *mut harfbuzz_sys::hb_face_t;
    fn hb_ft_font_create_referenced(reference: FTFontRef) -> *mut harfbuzz_sys::hb_font_t;
}