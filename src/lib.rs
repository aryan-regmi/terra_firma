pub mod game;
pub mod screens;
pub mod utils;

pub mod prelude {
    pub use crate::game::*;
    pub use crate::screens::*;
    pub use crate::utils::*;
}

// TODO: Make everything private unless necessary! (`pub(crate)` instead of `pub` if needed)
