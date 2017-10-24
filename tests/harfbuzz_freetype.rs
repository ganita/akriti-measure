// Copyright 2017 Sreejith Krishnan R <sreejith@ganita.io>

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

    assert_eq!(face.ascent(), 768);
    assert_eq!(face.descent(), -256);

    let positions = face.measure(String::from("Test"), &HBDirection::LTR);
    assert_eq!(positions.width(), 1638);
    assert_eq!(positions.height(), 0);
}

#[test]
fn it_read_metrics_correctly() {
    let face = open_ft_font("STIX2Math.otf") as FTFontRef;
    let face = HBFace::from_freetype_font(face);

    assert_eq!(face.script_percent_scale_down(), 70);
    assert_eq!(face.script_script_percent_scale_down(), 55);
}

fn open_ft_font(name: &str) -> freetype_sys::FT_Face {
    unsafe {
        let mut library = ptr::null_mut();
        let error = freetype_sys::FT_Init_FreeType(&mut library);
        assert_eq!(error, (freetype_sys::FT_Err_Ok as i32));
        let mut face = ptr::null_mut();
        let path = ffi::CString::new(format!("{}/tests/fonts/{}", env!("CARGO_MANIFEST_DIR"), name)).unwrap();
        let error = freetype_sys::FT_New_Face(library, path.as_ptr(), 0, &mut face);
        assert_eq!(error, (freetype_sys::FT_Err_Ok as i32));
        let error = freetype_sys::FT_Set_Pixel_Sizes(face, 0, 15);
        assert_eq!(error, (freetype_sys::FT_Err_Ok as i32));
        return face;
    }
}