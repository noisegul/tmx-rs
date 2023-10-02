use properties::Property;

pub struct Layer {
    pub name: String,
    // x
    // y
    pub width: u32,
    pub height: u32,
    pub opacity: Option<f32>,
    pub visible: Option<u8>,
    pub offsetx: Option<f32>,
    pub offsety: Option<f32>,

    pub data: Vec<Data>,
    pub property: Option<Vec<Property>>
}

pub struct Data {
    pub encoding: Option<String>, // Base64, CSV, None (XML)
    pub compression: Option<String>, // gzip, zlib
    pub tile: Option<Vec<Tile>>,

    // The xml data content <a>this</a>
    //#[serde(rename="$value")]
    pub content: Option<String>,
}

// Not to be confused with the `tile` element inside a `Tileset`
pub struct Tile {
    pub gid: i32
}
