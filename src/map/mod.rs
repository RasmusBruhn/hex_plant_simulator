use crate::types;

pub mod sun;

mod data_mode;
pub use data_mode::DataModeBackground;

mod tile;
pub use tile::InstanceTile;
use tile::{Tile, TileNeighbors, TilePos};

pub mod settings;

mod grid_layout;
pub use grid_layout::{GridLayout, UniformGridLayout};

/// Describes the entire map
#[derive(Clone, Debug)]
pub struct Map<S: sun::Intensity> {
    /// All the tiles in a row first, left to right, bottom to top order
    tiles: Vec<Tile>,
    /// The intensity of the sun at each column in the range 0 to 1
    sun_tiles: Vec<sun::Tile>,
    /// The state of the sun
    sun: sun::State<S>,
    /// The size of the grid
    size: types::ISize,
    /// The simulation settings of the map
    settings: settings::Settings,
    /// The current iteration time step
    time: usize,
}

impl<S: sun::Intensity> Map<S> {
    /// Constructs a new empty map
    ///
    /// # Parameters
    ///
    /// size: The size of the map
    ///
    /// settings: The simulation settings for the map
    ///
    /// sun_intensity: The sun intensity variation
    pub fn new(size: types::ISize, settings: settings::Settings, mut sun_intensity: S) -> Self {
        // Set the map size for the sun intensities
        sun_intensity.set_size(size.w);

        let tiles = (0..size.w * size.h).map(|_| Tile::new()).collect();
        let sun_tiles = (0..size.w).map(|_| sun::Tile::new(0.0)).collect();
        let sun = sun::State::new(sun_intensity);

        return Self {
            tiles,
            sun_tiles,
            sun,
            size,
            settings,
            time: 0,
        };
    }

    /// Steps the simulation once
    pub fn step(&mut self) {
        // Set the new sun tile values
        self.sun_tiles = self.sun.get_tiles(self.time);

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

        // Update the time
        self.time += 1;
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
    pub fn get_settings(&self) -> &settings::Settings {
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
