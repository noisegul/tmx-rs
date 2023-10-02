# tmx-rs

A small parsing library for *.tmx files.
The TMX Map Format describes tile based maps, and is produced by the
[Tiled Map Editor](www.mapeditor.org).

### Example
```rust
use tmx::map::Map;

fn main() {
    let kmap = Map::from_file("assets/kmap.tmx");

    println!("{:?}", kmap);
    println!("{:?}", kmap.layer[0]);
    println!("{:?}", kmap.layer[0].name);
}
```
