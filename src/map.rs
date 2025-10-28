use std::mem;

use crate::{constants, types};

/// The display mode for the background of a tile
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DataModeBackground {
    /// Display the light level of the tile
    Light,
    /// Display the transparency value of the tile
    Transparency,
}

impl DataModeBackground {
    pub const COUNT: usize = 2;

    /// The id to the mode in a list of all modes
    pub fn id(&self) -> usize {
        return match self {
            Self::Light => 0,
            Self::Transparency => 1,
        };
    }

    /// Constructs a new data mode from an id
    ///
    /// # Parameters
    ///
    /// id: The id to construct from
    pub fn from_id(id: usize) -> Self {
        return match id.clamp(0, Self::COUNT - 1) {
            0 => Self::Light,
            1 => Self::Transparency,
            _ => panic!("DataModeBackground::from_id has not been updated"),
        };
    }

    /// Gets the next mode
    pub fn next(&self) -> Self {
        return Self::from_id((self.id() + 1) % Self::COUNT);
    }

    /// Gets the previous mode
    pub fn prev(&self) -> Self {
        return Self::from_id((self.id() + (Self::COUNT - 1)) % Self::COUNT);
    }

    /// Constructs a new list of the color maps for all modes
    ///
    /// # Parameters
    ///
    /// transparency: The color map for transparency mode
    ///
    /// light: The color map for light mode
    pub fn new_color_map_collection(
        transparency: types::ColorMap,
        light: types::ColorMap,
    ) -> [types::ColorMap; Self::COUNT] {
        return [light, transparency];
    }
}

/// All data for a single sun ray
#[derive(Clone, Copy, Debug, PartialEq)]
struct SunTile {
    /// The intensity of the sun
    intensity: f64,
}

impl SunTile {
    /// Constructs a new sun tile
    ///
    /// # Parameters
    ///
    /// intensity: The intensity of the tile
    fn new(intensity: f64) -> Self {
        return Self { intensity };
    }

    /// Converts the sun tile to shader compatible data
    fn get_data(&self) -> InstanceTile {
        return InstanceTile {
            color_value: self.intensity as f32,
        };
    }
}

/// A single tile for the map
#[derive(Clone, Copy, Debug, PartialEq)]
struct Tile {
    /// The light transparency of this tile
    transparency: f64,
    /// The light level of this tile
    light: f64,
}

impl Tile {
    /// Constructs a new empty tile
    fn new() -> Self {
        return Self {
            transparency: 1.0,
            light: 0.0,
        };
    }

    /// Calculates the next state of the tile
    ///
    /// # Parameters
    ///
    /// map_settings: The settings for the map
    ///
    /// neighbohrs: References to all the neighbohrs of this til
    fn forward(&self, map_settings: &MapSettings, neighbors: &TileNeighbors) -> Self {
        // Set the new transparency
        let transparency = map_settings.transparency;

        // Set the new light level
        let light_right = match neighbors.up_right {
            Neighbor::Empty => 0.0,
            Neighbor::Tile(tile) => tile.light * tile.transparency,
            Neighbor::SunTile(tile) => tile.intensity,
        };
        let light_left = match neighbors.up_left {
            Neighbor::Empty => 0.0,
            Neighbor::Tile(tile) => tile.light * tile.transparency,
            Neighbor::SunTile(tile) => tile.intensity,
        };
        let light = 0.5 * (light_right + light_left);

        return Self {
            transparency,
            light,
        };
    }

    /// Converts the tile to shader compatible data
    ///
    /// mode: The mode to display
    pub fn get_data_background(&self, mode: &DataModeBackground) -> InstanceTile {
        let value = match mode {
            DataModeBackground::Transparency => self.transparency,
            DataModeBackground::Light => self.light,
        };

        return InstanceTile {
            color_value: value as f32,
        };
    }
}

/// References for all the neighbors of a single tile
#[derive(Clone, Copy, Debug, PartialEq)]
struct TileNeighbors<'a> {
    /// The tile to the right
    right: Neighbor<'a>,
    /// The tile to the up-right
    up_right: Neighbor<'a>,
    /// The tile to the up-left
    up_left: Neighbor<'a>,
    /// The tile to the left
    left: Neighbor<'a>,
    /// The tile to the down-left
    down_left: Neighbor<'a>,
    /// The tile to the down-right
    down_right: Neighbor<'a>,
}

impl<'a> TileNeighbors<'a> {
    /// Gets all the neighbohrs for a single tile
    ///
    /// # Parameters
    ///
    /// tiles: The list of tiles forming the grid in column first, left to right, top down order
    ///
    /// size: The size of the grid
    ///
    /// pos: The position of the tile to get neighbors for
    fn new(tiles: &'a [Tile], sun: &'a [SunTile], size: &types::ISize, pos: &TilePos) -> Self {
        let right = match pos.right(size) {
            TilePosNeighbor::Valid(pos) => Neighbor::Tile(&tiles[pos.to_index(size)]),
            TilePosNeighbor::Invalid(_) => Neighbor::Empty,
        };
        let up_right = match pos.up_right(size) {
            TilePosNeighbor::Valid(pos) => Neighbor::Tile(&tiles[pos.to_index(size)]),
            TilePosNeighbor::Invalid(_) => Neighbor::SunTile(&sun[pos.pos.x as usize]),
        };
        let up_left = match pos.up_left(size) {
            TilePosNeighbor::Valid(pos) => Neighbor::Tile(&tiles[pos.to_index(size)]),
            TilePosNeighbor::Invalid(_) => Neighbor::SunTile(&sun[pos.pos.x as usize]),
        };
        let left = match pos.left(size) {
            TilePosNeighbor::Valid(pos) => Neighbor::Tile(&tiles[pos.to_index(size)]),
            TilePosNeighbor::Invalid(_) => Neighbor::Empty,
        };
        let down_left = match pos.down_left(size) {
            TilePosNeighbor::Valid(pos) => Neighbor::Tile(&tiles[pos.to_index(size)]),
            TilePosNeighbor::Invalid(_) => Neighbor::Empty,
        };
        let down_right = match pos.down_right(size) {
            TilePosNeighbor::Valid(pos) => Neighbor::Tile(&tiles[pos.to_index(size)]),
            TilePosNeighbor::Invalid(_) => Neighbor::Empty,
        };

        return Self {
            right,
            up_right,
            up_left,
            left,
            down_left,
            down_right,
        };
    }
}

/// The reference to a neighbor tile
#[derive(Clone, Copy, Debug, PartialEq)]
enum Neighbor<'a> {
    /// There is nothing at this tile
    Empty,
    /// This neighbor is a normal tile
    Tile(&'a Tile),
    /// This neighbor is a sun tile
    SunTile(&'a SunTile),
}

/// Describes the current state of the sun, it shines with an intensity
/// oscillating harmonically as a function of position
#[derive(Clone, Copy, Debug, PartialEq)]
struct SunState {
    /// The base intensity of the sun
    intensity: f64,
    /// The position of the sun on the map in the range [0, 1[
    position: f64,
}

impl SunState {
    /// Constructs a new sun state
    fn new() -> Self {
        return Self {
            intensity: 1.0,
            position: 0.0,
        };
    }
}

/// Describes the entire map
#[derive(Clone, Debug, PartialEq)]
pub struct Map {
    /// All the tiles in a row first, left to right, bottom to top order
    tiles: Vec<Tile>,
    /// The intensity of the sun at each column in the range 0 to 1
    sun_tiles: Vec<SunTile>,
    /// The state of the sun
    sun: SunState,
    /// The size of the grid
    size: types::ISize,
    /// The simulation settings of the map
    settings: MapSettings,
}

impl Map {
    /// Constructs a new empty map
    ///
    /// # Parameters
    ///
    /// size: The size of the map
    ///
    /// settings: The simulation settings for the map
    pub fn new(size: types::ISize, settings: MapSettings) -> Self {
        let tiles = (0..size.w * size.h).map(|_| Tile::new()).collect();
        let sun_tiles = (0..size.w).map(|_| SunTile::new(0.0)).collect();
        let sun = SunState::new();

        return Self {
            tiles,
            sun_tiles,
            sun,
            size,
            settings,
        };
    }

    /// Steps the simulation once
    pub fn step(&mut self) {
        // Update the grid
        self.tiles = self
            .tiles
            .iter()
            .enumerate()
            .map(|(index, tile)| {
                tile.forward(
                    &self.settings,
                    &TileNeighbors::new(
                        &self.tiles,
                        &self.sun_tiles,
                        &self.size,
                        &TilePos::from_index(index, &self.size),
                    ),
                )
            })
            .collect();

        // Update the sun state
        self.sun.position = (self.sun.position + self.settings.sun_speed) % 1.0;

        // Set the new sun tile values
        self.sun_tiles = (0..self.size.w)
            .map(|index| {
                let pos = index as f64 / self.size.w as f64;
                let dist = (pos - self.sun.position).abs();
                let dist = if dist > 0.5 { 1.0 - dist } else { dist };

                let intensity = self.sun.intensity * (dist * constants::MATH_PI).cos();

                return SunTile::new(intensity);
            })
            .collect();
    }

    /// Retrieves the grid layout of the map
    pub fn get_grid_layout(&self) -> GridLayout {
        return GridLayout {
            n_columns: self.size.w,
        };
    }

    /// Retrieves the size of the map
    pub fn get_size(&self) -> &types::ISize {
        return &self.size;
    }

    /// Retrieves the simulation settings for the map
    pub fn get_settings(&self) -> &MapSettings {
        return &self.settings;
    }

    /// Converts all tiles to shader compatible data
    ///
    /// # Parameters
    ///
    /// mode: The mode for displaying the background
    pub fn get_tile_data_background(&self, mode: &DataModeBackground) -> Vec<InstanceTile> {
        return self
            .tiles
            .iter()
            .map(|tile| tile.get_data_background(mode))
            .collect();
    }

    /// Converts all sun tiles to shader compatible data
    pub fn get_sun_data(&self) -> Vec<InstanceTile> {
        return self.sun_tiles.iter().map(|tile| tile.get_data()).collect();
    }
}

/// All basic settings for a map
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MapSettings {
    /// The base transparency of any tile
    transparency: f64,
    /// The speed of the sun (increment per simulation step)
    sun_speed: f64,
}

impl MapSettings {
    /// Constructs a new default settings
    pub fn new() -> Self {
        return Self {
            transparency: 1.0,
            sun_speed: 0.0,
        };
    }

    /// Sets the transparency of the settings and returns the updated settings
    ///
    /// # Parameters
    ///
    /// transparency: The new transparency to set
    pub fn with_transparency(mut self, transparency: f64) -> Self {
        self.transparency = transparency;

        return self;
    }

    /// Sets the speed of the sun in the settings and returns the updated settings
    ///
    /// # Parameters
    ///
    /// speed: The new speed to set
    pub fn with_sun_speed(mut self, speed: f64) -> Self {
        self.sun_speed = speed;

        return self;
    }
}

/// A tile index position in the grid
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TilePos {
    /// The index of the tile
    pos: types::Index,
}

impl TilePos {
    /// Constructs a new tile position
    ///
    /// # Parameters
    ///
    /// pos: The position of the tile
    pub fn new(pos: types::Index) -> Self {
        return Self { pos };
    }

    /// Construct a tile position from a list index
    ///
    /// # Parameters
    ///
    /// index: The index of the tile in the tile list
    ///
    /// size: The size of the tile grid
    pub fn from_index(index: usize, size: &types::ISize) -> Self {
        let y = (index / size.w) as isize;
        let x = index as isize - y * size.w as isize;

        return Self {
            pos: types::Index { x, y },
        };
    }

    /// Gets the tile position right of this tile, None if it is outside the grid
    ///
    /// # Parameters
    ///
    /// size: The size of the tile grid
    pub fn right(&self, size: &types::ISize) -> TilePosNeighbor {
        let y = self.pos.y;
        let x = if self.pos.x == size.w as isize - 1 {
            0
        } else {
            self.pos.x + 1
        };

        return TilePosNeighbor::Valid(Self {
            pos: types::Index { x, y },
        });
    }

    /// Gets the tile position up-right of this tile, None if it is outside the grid
    ///
    /// # Parameters
    ///
    /// size: The size of the tile grid
    pub fn up_right(&self, size: &types::ISize) -> TilePosNeighbor {
        let y = self.pos.y - 1;
        let x = if self.pos.x % 2 == 0 {
            self.pos.x
        } else {
            if self.pos.x == size.w as isize - 1 {
                0
            } else {
                self.pos.x + 1
            }
        };
        let pos = types::Index { x, y };

        return if y == -1 {
            TilePosNeighbor::Invalid(Self { pos })
        } else {
            TilePosNeighbor::Valid(Self { pos })
        };
    }

    /// Gets the tile position up-left of this tile, None if it is outside the grid
    ///
    /// # Parameters
    ///
    /// size: The size of the tile grid
    pub fn up_left(&self, size: &types::ISize) -> TilePosNeighbor {
        let y = self.pos.y - 1;
        let x = if self.pos.x % 2 == 0 {
            if self.pos.x == 0 {
                size.w as isize - 1
            } else {
                self.pos.x - 1
            }
        } else {
            self.pos.x
        };
        let pos = types::Index { x, y };

        return if y == -1 {
            TilePosNeighbor::Invalid(Self { pos })
        } else {
            TilePosNeighbor::Valid(Self { pos })
        };
    }

    /// Gets the tile position left of this tile, None if it is outside the grid
    ///
    /// # Parameters
    ///
    /// size: The size of the tile grid
    pub fn left(&self, size: &types::ISize) -> TilePosNeighbor {
        let y = self.pos.y;
        let x = if self.pos.x == 0 {
            size.w as isize - 1
        } else {
            self.pos.x - 1
        };

        return TilePosNeighbor::Valid(Self {
            pos: types::Index { x, y },
        });
    }

    /// Gets the tile position down-left of this tile, None if it is outside the grid
    ///
    /// # Parameters
    ///
    /// size: The size of the tile grid
    pub fn down_left(&self, size: &types::ISize) -> TilePosNeighbor {
        let y = self.pos.y + 1;
        let x = if self.pos.x % 2 == 0 {
            if self.pos.x == 0 {
                size.w as isize - 1
            } else {
                self.pos.x - 1
            }
        } else {
            self.pos.x
        };
        let pos = types::Index { x, y };

        return if y == size.h as isize {
            TilePosNeighbor::Invalid(Self { pos })
        } else {
            TilePosNeighbor::Valid(Self { pos })
        };
    }

    /// Gets the tile position down-right of this tile, None if it is outside the grid
    ///
    /// # Parameters
    ///
    /// size: The size of the tile grid
    pub fn down_right(&self, size: &types::ISize) -> TilePosNeighbor {
        let y = self.pos.y + 1;
        let x = if self.pos.x % 2 == 0 {
            self.pos.x
        } else {
            if self.pos.x == size.w as isize - 1 {
                0
            } else {
                self.pos.x + 1
            }
        };
        let pos = types::Index { x, y };

        return if y == size.h as isize {
            TilePosNeighbor::Invalid(Self { pos })
        } else {
            TilePosNeighbor::Valid(Self { pos })
        };
    }

    /// Fixes a tile position by bounding it inside the grid in y and wrapping it in x
    ///
    /// # Parameters
    ///
    /// size: The size of the tile grid
    pub fn fix_pos(&self, size: &types::ISize) -> Self {
        let y = self.pos.y.clamp(0, size.h as isize - 1);
        let x_pre = self.pos.x % size.w as isize;
        let x = if x_pre < 0 {
            x_pre + size.w as isize
        } else {
            x_pre
        };

        return Self {
            pos: types::Index { x, y },
        };
    }

    /// Converts the tile position into an index in the tile list
    ///
    /// # Parameters
    ///
    /// size: The size of the tile grid
    pub fn to_index(&self, size: &types::ISize) -> usize {
        return (self.pos.y * size.w as isize + self.pos.x) as usize;
    }
}

/// Describes the tile position of a neighbor to a tile
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TilePosNeighbor {
    /// The position is inside the grid
    Valid(TilePos),
    /// The position is outside the grid
    Invalid(TilePos),
}

/// All data for instancing a tile
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct InstanceTile {
    /// The value to draw at this tile
    pub color_value: f32,
}

impl InstanceTile {
    /// Creates the vertex buffer description for the tile instance
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        return wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<InstanceTile>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[wgpu::VertexAttribute {
                offset: 0,
                shader_location: 1,
                format: wgpu::VertexFormat::Float32,
            }],
        };
    }
}

/// All data for the layout of the grid
#[derive(Copy, Clone, Debug)]
pub struct GridLayout {
    /// The number of columns in the grid
    pub n_columns: usize,
}

impl GridLayout {
    /// Constructs the shader compatible version off a grid layout
    pub fn get_data(&self) -> UniformGridLayout {
        return UniformGridLayout {
            n_columns: self.n_columns as u32,
        };
    }
}

/// All data for the layout of the grid
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct UniformGridLayout {
    // The base color to scale
    pub n_columns: u32,
}
