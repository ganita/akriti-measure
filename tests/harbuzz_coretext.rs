// Copyright 2017 Sreejith Krishnan R <sreejith@ganita.io>

extern crate akriti_measure;
extern crate core_foundation;

use core_foundation::string::{CFString, CFStringRef};
use core_foundation::base::TCFType;

use akriti_measure::harfbuzz::*;

#[link(name = "CoreGraphics", kind = "framework")]
extern {
    fn CGFontCreateWithFontName(name: CFStringRef) -> CGFontRef;
}

#[test]
#[should_panic]
fn it_fails_to_open_invalid_font() {
    let font_name = CFString::from_static_string("Invalid Font");
    let cg_font = unsafe { CGFontCreateWithFontName(font_name.as_concrete_TypeRef()) };
    
    HBFace::from_cg_font(cg_font);
}

#[test]
fn it_opens_font() {
    let font_name = CFString::from_static_string("STIX Two Math");
    let cg_font = unsafe { CGFontCreateWithFontName(font_name.as_concrete_TypeRef()) };
    
    let face = HBFace::from_cg_font(cg_font);

    assert_eq!(face.index(), 0);
    assert_eq!(face.glyph_count(), 5248);
    assert_eq!(face.upem(), 1000);

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

#[test]
fn it_opens_arial_font() {
    let font_name = CFString::from_static_string("Arial");
    let cg_font = unsafe { CGFontCreateWithFontName(font_name.as_concrete_TypeRef()) };
    
    let face = HBFace::from_cg_font(cg_font);
    assert_eq!(face.has_ot_math_table(), false);
    assert_eq!(face.upem(), 2048);
}