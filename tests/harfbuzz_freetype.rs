extern crate akriti_measure;
extern crate freetype_sys;

use std::ptr;
use std::ffi;

use akriti_measure::harfbuzz::*;

#[test]
#[should_panic]
fn it_fails_to_open_invalid_font() {
    HBFace::from_freetype_font(ptr::null_mut());
}

#[test]
fn it_opens_freetype_font() {
    let face = open_ft_font("STIX2Math.otf") as FTFontRef;
    let face = HBFace::from_freetype_font(face);

    assert_eq!(face.index(), 0);
    assert_eq!(face.glyph_count(), 5248);
    assert_eq!(face.upem(), 1000);
    assert_eq!(face.glyph_index('c' as u32).unwrap(), 257);
    assert_eq!(face.glyph_index(909909), None);

    assert_eq!(face.italics_correction(face.glyph_index('𝐼' as u32).unwrap()), 43);
    assert_eq!(face.top_accent_attachment(face.glyph_index('𝐼' as u32).unwrap()), 238);
    assert!(face.is_glyph_extended_shape(face.glyph_index(0x5b).unwrap()));
    assert!(!face.is_glyph_extended_shape(face.glyph_index('a' as u32).unwrap()));

    assert_eq!(face.has_ot_math_table(), true);

    assert_eq!(face.glyph_kerning_top_right(face.glyph_index('A' as u32).unwrap(), 300), -17);
    assert_eq!(face.glyph_kerning_top_right(face.glyph_index('A' as u32).unwrap(), 400), -63);
    assert_eq!(face.glyph_kerning_top_right(face.glyph_index('A' as u32).unwrap(), 200), 0);

    let mut variants = face.glyph_variants(face.glyph_index('√' as u32).unwrap(), HBDirection::TTB);
    assert_eq!(variants.len(), 4);
    assert_eq!(variants.next().unwrap().glyph_index(), 1657);
    assert_eq!(variants.next().unwrap().glyph_index(), 1658);
    assert_eq!(variants.next().unwrap().glyph_index(), 1659);
    assert_eq!(variants.next().unwrap().glyph_index(), 1660);
    assert_eq!(variants.next(), None);

    let parts = face.glyph_assembly(face.glyph_index('√' as u32).unwrap(), HBDirection::TTB);
    assert_eq!(parts.italics_correction(), 0);
    let parts = parts.parts();
    assert_eq!(parts[0].start_connector_length(), 192);
    assert_eq!(parts[0].end_connector_length(), 192);
    assert_eq!(parts[0].full_advance(), 1829);
    assert_eq!(parts[0].is_extender(), false);

    assert_eq!(parts[1].start_connector_length(), 624);
    assert_eq!(parts[1].end_connector_length(), 624);
    assert_eq!(parts[1].full_advance(), 625);
    assert_eq!(parts[1].is_extender(), true);

    assert_eq!(parts[2].start_connector_length(), 528);
    assert_eq!(parts[2].end_connector_length(), 0);
    assert_eq!(parts[2].full_advance(), 616);
    assert_eq!(parts[2].is_extender(), false);

    assert_eq!(face.glyph_assembly(face.glyph_index('a' as u32).unwrap(), HBDirection::TTB).len(), 0);
}

#[ignore]
#[test]
fn it_read_metrics_correctly() {
    let face = open_ft_font("STIX2Math.otf") as FTFontRef;
    let face = HBFace::from_freetype_font(face);

    assert_eq!(face.script_percent_scale_down(), 70);
    assert_eq!(face.script_script_percent_scale_down(), 55);
    assert_eq!(face.delimited_sub_formula_min_height(), 1325);
    assert_eq!(face.display_operator_min_height(), 1800);
    assert_eq!(face.math_leading(), 150);
    assert_eq!(face.axis_height(), 258);
    assert_eq!(face.accent_base_height(), 480);
    assert_eq!(face.flattened_accent_base_height(), 656);

    assert_eq!(face.subscript_shift_down(), 210);
    assert_eq!(face.subscript_top_max(), 368);
    assert_eq!(face.subscript_baseline_drop_min(), 160);
    assert_eq!(face.superscript_shift_up(), 360);
    assert_eq!(face.superscript_shift_up_cramped(), 252);
    assert_eq!(face.superscript_bottom_min(), 120);
    assert_eq!(face.superscript_baseline_drop_max(), 230);
    assert_eq!(face.sub_superscript_gap_min(), 150);
    assert_eq!(face.superscript_bottom_max_with_subscript(), 380);
    assert_eq!(face.space_after_script(), 40);

    assert_eq!(face.upper_limit_gap_min(), 135);
    assert_eq!(face.upper_limit_baseline_rise_min(), 300);
    assert_eq!(face.lower_limit_gap_min(), 135);
    assert_eq!(face.lower_limit_baseline_drop_min(), 670);

    assert_eq!(face.stack_top_shift_up(), 470);
    assert_eq!(face.stack_top_display_style_shift_up(), 780);
    assert_eq!(face.stack_bottom_shift_down(), 385);
    assert_eq!(face.stack_bottom_display_style_shift_down(), 690);
    assert_eq!(face.stack_gap_min(), 150);
    assert_eq!(face.stack_display_style_gap_min(), 300);
    assert_eq!(face.stretch_stack_top_shift_up(), 800);
    assert_eq!(face.stretch_stack_bottom_shift_down(), 590);
    assert_eq!(face.stretch_stack_gap_above_min(), 68);
    assert_eq!(face.stretch_stack_gap_below_min(), 68);

    assert_eq!(face.fraction_numerator_shift_up(), 585);
    assert_eq!(face.fraction_numerator_display_style_shift_up(), 640);
    assert_eq!(face.fraction_denominator_shift_down(), 585);
    assert_eq!(face.fraction_denominator_display_style_shift_down(), 640);
    assert_eq!(face.fraction_numerator_gap_min(), 68);
    assert_eq!(face.fraction_num_display_style_gap_min(), 150);
    assert_eq!(face.fraction_rule_thickness(), 68);
    assert_eq!(face.fraction_denominator_gap_min(), 68);
    assert_eq!(face.fraction_denominator_display_style_gap_min(), 150);
    assert_eq!(face.skewed_fraction_horizontal_gap(), 350);
    assert_eq!(face.skewed_fraction_vertical_gap(), 68);

    assert_eq!(face.overbar_vertical_gap(), 175);
    assert_eq!(face.overbar_rule_thickness(), 68);
    assert_eq!(face.overbar_extra_ascender(), 68);
    assert_eq!(face.underbar_vertical_gap(), 175);
    assert_eq!(face.underbar_rule_thickness(), 68);
    assert_eq!(face.underbar_extra_descender(), 68);

    assert_eq!(face.radical_vertical_gap(), 85);
    assert_eq!(face.radical_display_style_vertical_gap(), 170);
    assert_eq!(face.radical_rule_thickness(), 68);
    assert_eq!(face.radical_extra_ascender(), 68);
    assert_eq!(face.radical_kern_before_degree(), 65);
    assert_eq!(face.radical_kern_after_degree(), -335);
    assert_eq!(face.radical_degree_bottom_raise_percent(), 55);

    assert_eq!(face.min_connector_overlap_vertical(), 100);
    assert_eq!(face.min_connector_overlap_horizontal(), 100);
}

fn open_ft_font(name: &str) -> freetype_sys::FT_Face {
    unsafe {
        let mut library = ptr::null_mut();
        let error = freetype_sys::FT_Init_FreeType(&mut library);
        assert!(error == (freetype_sys::FT_Err_Ok as i32));
        let mut face = ptr::null_mut();
        let path = ffi::CString::new(format!("{}/tests/fonts/{}", env!("CARGO_MANIFEST_DIR"), name)).unwrap();
        let error = freetype_sys::FT_New_Face(library, path.as_ptr(), 0, &mut face);
        assert!(error == (freetype_sys::FT_Err_Ok as i32));
        let error = freetype_sys::FT_Set_Pixel_Sizes(face, 0, 15);
        assert!(error == (freetype_sys::FT_Err_Ok as i32));
        return face;
    }
}