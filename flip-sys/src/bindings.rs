/* automatically generated by rust-bindgen 0.65.1 */

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FlipImage {
    _unused: [u8; 0],
}
extern "C" {
    pub fn flip_image_new(width: u32, height: u32, data: *const u8) -> *mut FlipImage;
}
extern "C" {
    pub fn flip_image_free(image: *mut FlipImage);
}
extern "C" {
    pub fn flip_image_flip(
        error_map: *mut FlipImage,
        reference_image: *mut FlipImage,
        test_image: *mut FlipImage,
    );
}