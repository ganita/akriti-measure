/*
 * Copyright 2017 Sreejith Krishnan R
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
*/


use std::ptr;
use std::rc::Rc;
use std::ffi::CString;

use ::freetype_sys::*;
use ::harfbuzz::{HBFace, FTFontRef};

pub struct Freetype {
    ptr: *mut FT_LibraryRec_,
}

impl Freetype {
    pub fn new() -> Freetype {
        let mut library = ptr::null_mut();
        let error = unsafe { FT_Init_FreeType(&mut library) };
        if error != FT_Err_Ok as i32 {
            panic!("Failed to initialize freetype library. Error: {:?}", error);
        }
        Freetype { ptr: library }
    }
}

impl Drop for Freetype {
    fn drop(&mut self) {
        unsafe { FT_Done_FreeType(self.ptr) };
    }
}


pub struct FreetypeFace {
    library: Rc<Freetype>,
    ptr: *mut FT_FaceRec_,
    hb_face: HBFace,
}

impl FreetypeFace {
    pub fn new_from_file(library: Rc<Freetype>, path: &str, index: u32) -> Result<FreetypeFace, ()> {
        let mut ptr = ptr::null_mut();
        let path = CString::new(path).unwrap();

        let error = unsafe { FT_New_Face(library.ptr, path.as_ptr(),
                                         index as i64, &mut ptr) };

        if error != FT_Err_Ok as i32 {
            return Err(());
        }

        Ok(FreetypeFace {
            library,
            ptr,
            hb_face: HBFace::from_freetype_font(ptr as FTFontRef)
        })
    }

    pub fn set_size_pixels(&mut self, width: u32, height: u32) {
        unsafe { FT_Set_Pixel_Sizes(self.ptr, width, height) };
    }

    pub fn get_hb_face(&self) -> &HBFace {
        &self.hb_face
    }
}

impl Drop for FreetypeFace {
    fn drop(&mut self) {
        unsafe { FT_Done_Face(self.ptr) };
    }
}