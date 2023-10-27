use crate::*;

pub static ASSET_DIR: include_dir::Dir<'_> =
    include_dir::include_dir!("$CARGO_MANIFEST_DIR/assets");

fn base_path(path: &str) -> String {
    format!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/{}"), path)
}

pub fn load_assets() {
    init_asset_source(&ASSET_DIR, base_path);

    let textures = vec![
        ("wall", "wall.png"),
        ("player", "player.png"),
        ("gun", "gun.png"),
        ("bullet", "bullet.png"),
        ("enemy", "enemy.png"),
    ];

    load_multiple_textures(
        textures
            .iter()
            .map(|(a, b)| (a.to_string(), b.to_string()))
            .collect_vec(),
    );

    let sounds = vec![
        ("footsteps-0", "sfx/footsteps-1.ogg"),
        ("footsteps-1", "sfx/footsteps-2.ogg"),
        ("footsteps-2", "sfx/footsteps-3.ogg"),
        ("footsteps-3", "sfx/footsteps-4.ogg"),
        ("footsteps-4", "sfx/footsteps-5.ogg"),
        ("footsteps-5", "sfx/footsteps-6.ogg"),
        ("footsteps-6", "sfx/footsteps-7.ogg"),
        ("shotgun", "sfx/shotgun.ogg"),
        ("scary-music", "scary-atmo-1.ogg"),
        ("breathing", "sfx/breathing.ogg"),
        ("monster-hit", "sfx/monster-hit.ogg"),
        ("map-change", "sfx/map-change.ogg"),
        ("player-death", "sfx/monster-attack.ogg"),
    ];

    load_multiple_sounds(
        sounds
            .iter()
            .map(|(a, b)| (a.to_string(), b.to_string()))
            .collect_vec(),
    );
}
