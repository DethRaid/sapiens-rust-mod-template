pub mod biome;

#[cfg(target_feature = "biome")]
use biome::{get_biome_type, get_surface_type};

/// Returns the biome type that occurs at the provided climate type
/// 
/// # Parameters
/// * `climate_type` - The type of climate
#[cfg(target_feature = "biome")]
extern "C" fn spBiomeGetBiomeType(climate_type: i32) -> i32 {
    get_biome_type(climate_type)
}

/// Retrieves the biome surface type given the provided parameters
/// 
/// # Parameters
/// * `noise` - The noise parameters to use when generating the survace type
/// * `biome_types` - A list of all the possible biome types
/// * `point_normal` - The normal of the surface you want the surface type of
/// * `noise_loc` - The location to pass to the noise function
/// * `biome_type` - The type of the biome at the point you want the surface type of
/// * `vegetation_state` - The vegetation state of the point you want the surface type of
/// * `altitude` - The altitude of the point you want the surface type of
/// * `steepness` - The steepness of the point you want the surface type of
/// * `river_distance` - The distance to the nearest river, maybe?
#[cfg(target_feature = "biome")]
extern "C" fn spBiomeGetSurfaceType(noise: *mut SPNoise, biome_types: *mut SPBiomeType, point_normal: SPVec3, noise_loc: SPVec3, biome_type: i32, vegetation_state: i32, altitude: f64, steepness: f32, river_distance: f32) -> i32 {
    get_surface_type(Noise(noise), std::slice::from_)
}
