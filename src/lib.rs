pub(crate) mod game;
pub(crate) mod screens;
pub(crate) mod utils;

pub use game::GamePlugin;

// NOTE: use TextureAtlasLayout to get atlas for tilesets to get tile textures
//  - Get image handles like in `grey_out_game`
//  - Get TextureAtlasLayout (asset server) and TextureAtlas (query)
//  - Update the image like in `grey_out_game`, except bound the x and y by the `texture_rect`
