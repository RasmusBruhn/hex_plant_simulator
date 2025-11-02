use crate::types;

use super::{Tile, sun};

/// References for all the neighbors of a single tile
#[derive(Clone, Debug)]
pub struct TileNeighbors<'a> {
    /// The tile to the right
    pub right: Neighbor<'a>,
    /// The tile to the up-right
    pub up_right: Neighbor<'a>,
    /// The tile to the up-left
    pub up_left: Neighbor<'a>,
    /// The tile to the left
    pub left: Neighbor<'a>,
    /// The tile to the down-left
    pub down_left: Neighbor<'a>,
    /// The tile to the down-right
    pub down_right: Neighbor<'a>,
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
    pub fn new(
        tiles: &'a [Tile],
        sun: &'a [sun::Tile],
        size: &types::ISize,
        pos: &TilePos,
    ) -> Self {
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
#[derive(Clone, Debug)]
pub enum Neighbor<'a> {
    /// There is nothing at this tile
    Empty,
    /// This neighbor is a normal tile
    Tile(&'a Tile),
    /// This neighbor is a sun tile
    SunTile(&'a sun::Tile),
}

/// A tile index position in the grid
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TilePos {
    /// The index of the tile
    pub pos: types::Index,
}

impl TilePos {
    /// Constructs a new tile position
    ///
    /// # Parameters
    ///
    /// pos: The position of the tile
    pub fn _new(pos: types::Index) -> Self {
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
    pub fn _fix_pos(&self, size: &types::ISize) -> Self {
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
