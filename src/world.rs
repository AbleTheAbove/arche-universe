pub const CHUNKSIZE: usize = 128;
pub type Chunk = [[Tile; CHUNKSIZE]; CHUNKSIZE];

#[derive(Debug, Clone, Copy)]
pub enum Tile {
    Air,
    Stone,
}

fn holder() {
    let chunk0 = [[Tile::Air; CHUNKSIZE]; CHUNKSIZE];
    let chunk1 = [[Tile::Air; CHUNKSIZE]; CHUNKSIZE];
    let chunk2 = [[Tile::Air; CHUNKSIZE]; CHUNKSIZE];

    let x = Box::new(chunk0);
}
