use properties::Property;
use objectgroup::ObjectGroup;
use image::Image;

pub struct Tileset {
    firstgid: u32,
    source: Option<String>,
    name: String,
    tilewidth: u32,
    tileheight: u32,
    spacing: Option<u32>,
    margin: Option<u32>,
    tilecount: u32,
    columns: u32,

    tileoffset: Option<Vec<TileOffset>>,
    property: Option<Vec<Property>>,
    image: Option<Vec<Image>>,
    terraintype: Option<Vec<Terrain>>,
    tile: Option<Vec<Tile>>
}

pub struct TileOffset {
    x: f32,
    y: f32
}

pub struct Terrain {
    name: String,
    tile: i32,
    properties: Property
}

pub struct Tile {
    id: u32,
    terrain: Terrain,
    probability: i32,

    properties: Vec<Property>,
    images: Vec<Image>,
    objectgroups: Vec<ObjectGroup>,
    animation: Vec<Frame>
}

pub struct Frame {
    tileid: i32,
    duration: u32
}
