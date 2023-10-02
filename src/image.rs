pub struct Image {
    // Image formats: png, gif, jpg, bmp
    format: Option<String>,
    source: String,
    trans: String, // normally: Color
    width: u32,
    height: u32
}
