// The angle of the sun above the horizon (alpha) for a planet is given by:
//
// cos(alpha) = (cos(beta) * cos(psi) + cos(phi) * tan(theta) * sin(beta)) / sqrt(1 + cos^2(phi) * tan^2(theta))
//
// theta: Tilt of the planet (0: equator is in orbital plane)
// beta: Latitude of location on planet
// phi: Time of year (0: summer for positive beta)
// psi: Time of day (0: noon)

use super::InstanceTile;

mod state;
pub(super) use state::State;

mod tile;
pub(super) use tile::Tile;

mod intensity;
pub use intensity::{Intensity, IntensityPlanet};

mod position;
