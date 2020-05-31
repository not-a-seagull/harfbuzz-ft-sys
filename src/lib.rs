// This code is dual-licensed under the Apache 2.0 License and the MIT License.

//! Hand-written FFI bindings to the Harfbuzz Freetype library.

use freetype::freetype::FT_Face;
use harfbuzz_sys::{hb_destroy_func_t, hb_face_t, hb_font_t};
use std::os::raw::c_int;

#[link(name = "harfbuzz")]
extern "C" {
    /// Official harfbuzz documentation:
    ///
    /// This one creates a new hb-face for given ft-face.
    /// When the returned hb-face is destroyed, the destroy
    /// callback is called (if not NULL), with the ft-face passed
    /// to it.
    ///
    /// The client is responsible to make sure that ft-face is
    /// destroyed after hb-face is destroyed.
    ///
    /// Most often you don't want this function.  You should use either
    /// hb_ft_face_create_cached(), or hb_ft_face_create_referenced().
    /// In particular, if you are going to pass NULL as destroy, you
    /// probably should use (the more recent) hb_ft_face_create_referenced()
    /// instead.
    pub fn hb_ft_face_create(ft_face: FT_Face, destroy: hb_destroy_func_t) -> *mut hb_face_t;

    /// Official harfbuzz documentation:
    ///
    /// This version is like hb_ft_face_create(), except that it caches
    /// the hb-face using the generic pointer of the ft-face.  This means
    /// that subsequent calls to this function with the same ft-face will
    /// return the same hb-face (correctly referenced).
    ///
    /// Client is still responsible for making sure that ft-face is destroyed
    /// after hb-face is.
    pub fn hb_ft_face_create_cached(ft_face: FT_Face) -> *mut hb_face_t;

    /// Official harfbuzz documentation:
    ///
    /// This version is like hb_ft_face_create(), except that it calls
    /// FT_Reference_Face() on ft-face, as such keeping ft-face alive
    /// as long as the hb-face is.
    ///
    /// This is the most convenient version to use.  Use it unless you have
    /// very good reasons not to.
    pub fn hb_ft_create_referenced(ft_face: FT_Face) -> *mut hb_face_t;

    /// Official harfbuzz documentation:
    ///
    /// See notes on hb_ft_face_create().  Same issues re lifecycle-management
    /// apply here.  Use hb_ft_font_create_referenced() if you can.
    pub fn hb_ft_font_create(ft_face: FT_Face, destroy: hb_destroy_func_t) -> *mut hb_font_t;

    /// Official harfbuzz documentation:
    ///
    /// See notes on hb_ft_face_create_referenced() re lifecycle-management
    /// issues.
    pub fn hb_ft_font_create_referenced(ft_face: FT_Face) -> *mut hb_font_t;

    pub fn hb_ft_font_get_face(font: *mut hb_font_t) -> FT_Face;

    pub fn hb_ft_font_set_load_flags(font: *mut hb_font_t, load_flags: c_int);

    pub fn hb_ft_font_get_load_flags(font: *mut hb_font_t) -> c_int;

    /// Official harfbuzz documentation:
    ///
    /// Call when size or variations settings on underlying FT_Face change.
    pub fn hb_ft_font_changed(font: *mut hb_font_t);

    /// Official harfbuzz documentaion:
    ///
    /// Makes an hb_font_t use FreeType internally to implement font functions.
    /// Note: this internally creates an FT_Face.  Use it when you create your
    /// hb_face_t using hb_face_create().
    pub fn hb_ft_font_set_funcs(font: *mut hb_font_t);
}
