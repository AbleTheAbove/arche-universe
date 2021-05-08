pub const CHUNKSIZE: usize = 128;
pub type Layer = [[Tile; CHUNKSIZE]; CHUNKSIZE];

#[derive(Debug)]
pub struct World {
    pub data: [Layer; 3],
}
#[derive(Debug, Clone, Copy)]
pub enum Tile {
    Air,
    Stone,
}

fn holder() {
    let layer0 = [[Tile::Air; CHUNKSIZE]; CHUNKSIZE];
    let layer1 = [[Tile::Air; CHUNKSIZE]; CHUNKSIZE];
    let layer2 = [[Tile::Air; CHUNKSIZE]; CHUNKSIZE];

    let x = Box::new(World {
        data: [layer0, layer1, layer2],
    });
}
