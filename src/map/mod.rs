use crate::{constants, types};

pub mod sun;

mod data_mode;
pub use data_mode::DataModeBackground;

mod tile;
pub use tile::InstanceTile;
use tile::{Tile, TileNeighbors, TilePos};

mod settings;
pub use settings::Settings;

mod grid_layout;
pub use grid_layout::{GridLayout, UniformGridLayout};

/// Describes the entire map
#[derive(Clone, Debug, PartialEq)]
pub struct Map {
    /// All the tiles in a row first, left to right, bottom to top order
    tiles: Vec<Tile>,
    /// The intensity of the sun at each column in the range 0 to 1
    sun_tiles: Vec<sun::Tile>,
    /// The state of the sun
    sun: sun::State,
    /// The size of the grid
    size: types::ISize,
    /// The simulation settings of the map
    settings: Settings,
}

impl Map {
    /// Constructs a new empty map
    ///
    /// # Parameters
    ///
    /// size: The size of the map
    ///
    /// settings: The simulation settings for the map
    pub fn new(size: types::ISize, settings: Settings) -> Self {
        let tiles = (0..size.w * size.h).map(|_| Tile::new()).collect();
        let sun_tiles = (0..size.w).map(|_| sun::Tile::new(0.0)).collect();
        let sun = sun::State::new();

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

                return sun::Tile::new(intensity);
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
    pub fn get_settings(&self) -> &Settings {
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
