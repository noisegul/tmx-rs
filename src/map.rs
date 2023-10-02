use imagelayer::ImageLayer;
use layer::Layer;
use objectgroup::ObjectGroup;
use properties::Property;
use tileset::Tileset;

pub struct Map {
    pub version: String,

    // Possible orientations:
    // Orthogonal, Isometric, Staggered, Hexagonal
    pub orientation: Orientation,

    // Possible Renderorders:
    // RightDown, RightUp, LeftDown, Leftup
    pub renderorder: RenderOrder, // normally: RenderOrder
    pub width: u32,
    pub height: u32,
    pub tilewidth: u32,
    pub tileheight: u32,
    pub hexsidelength: Option<u32>,

    // Staggeraxis x or y
    pub staggeraxis: Option<String>,

    // Staggerindex even or odd
    pub staggerindex: Option<String>,
    pub backgroundcolor: Option<String>,
    pub nextobjectid: Option<u32>,
    pub tileset: Vec<Tileset>,
    pub layer: Vec<Layer>,
    pub objectgroup: Vec<ObjectGroup>,
    pub imagelayer: Vec<ImageLayer>,
    pub property: Vec<Property>
}

pub enum Orientation {
    Orthogonal,
    Isometric,
    Staggered,
    Hexagonal
}

pub enum RenderOrder {
    RightDown,
    RightUp,
    LeftDown,
    LeftUp
}
