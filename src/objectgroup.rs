use properties::Property;

pub struct ObjectGroup {
    name: String,
    color: String, // `Color` in the future
    // x
    // y
    // width
    // height
    opacity: f32,
    visible: u8,
    offsetx: u32,
    offsety: u32,

    // Draworder index or topdown
    draworder: String,

    properties: Vec<Property>,
    objects: Vec<Object>
}

pub struct Object {
    id: u32,
    name: String,
    obj_type: String,
    x: f32,
    y: f32,
    width: u32,
    height: u32,
    rotation: f32,

    // gid use is currently very unclear
    // gid:

    visible: u8,

    properties: Vec<Property>,
    ellipse: Ellipse,
    polygon: Polygon,
    polyline: Polyline,

    // `image` is listed as a possible content, but the nature
    // of this is unclear, so it won't be included for now
    // image:
}

pub struct Ellipse;

pub struct Polygon {
    /// A list of x,y coordinates in px
    /// TODO: Revisit the choice to use (x: i32, y: i32)
    points: Vec<(i32, i32)>
}

pub struct Polyline {
    /// A list of x,y coordinates in px
    /// TODO: Revisit the choice to use (x: i32, y: i32)
    points: Vec<(i32, i32)>
}
