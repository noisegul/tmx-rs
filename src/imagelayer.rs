use properties::Property;
use image::Image;

pub struct ImageLayer {
    name: String,
    offsetx: f32,
    offsety: f32,
    // x
    // y
    // width
    // height
    opacity: f32,
    visible: u8,

    properties: Vec<Property>,
    images: Vec<Image>
}


