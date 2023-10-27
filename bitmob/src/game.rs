use crate::*;

pub use image::{GenericImage, RgbaImage};

const BASE_COLOR: Color = WHITE;

pub struct Player {
    pub position: Vec2,
    pub move_timer: f32,
    pub footsteps_timer: f32,
    pub moved: bool,
    pub is_dead: bool,
}

impl Player {
    pub fn new() -> Self {
        Self {
            position: vec2(100.0, 60.0),
            move_timer: 0.0,
            footsteps_timer: 0.0,
            moved: false,
            is_dead: false,
        }
    }
}

pub struct Bullet {
    pub position: Vec2,
    pub velocity: Vec2,
}

pub struct GameState {
    pub walls: Grid<bool>,
    pub pixels: Grid<Color>,
    pub lighting: Grid<f32>,
    pub max_light_value: f32,
    pub image: DynamicImage,
    pub handle: TextureHandle,

    pub bullet: Option<Bullet>,
    pub player: Player,
    pub mobs: Vec<Mob>,

    pub is_monster_visible: bool,
    pub start_time: f32,
    pub map_change_timer: f32,
}

impl GameState {
    pub fn new(c: &mut EngineContext) -> Self {
        let ratio = screen_width() / screen_height();

        info!("ratio: {}", ratio);

        srand(8);

        let width = 200;
        let height = (width as f32 / ratio) as i32;

        let pixels = Grid::filled_with(width, height, |_x, _y| BLACK.alpha(0.0));
        let walls = Grid::filled_with(width, height, |_x, _y| coin_toss(0.13));
        let lighting = Grid::new(width, height, 0.0);

        let image = DynamicImage::ImageRgba8(RgbaImage::new(
            pixels.width() as u32,
            pixels.height() as u32,
        ));

        let handle = c
            .texture_creator
            .borrow_mut()
            .handle_from_image("img", &image);

        Self {
            pixels,
            walls,
            lighting,
            max_light_value: 1.0,
            image,
            handle,
            player: Player::new(),
            bullet: None,
            mobs: vec![],
            is_monster_visible: false,
            start_time: 0.0,
            map_change_timer: 10.0,
        }
    }
}

pub fn game_update(state: &mut GameState, c: &mut EngineContext) {
    c.texture_creator
        .borrow_mut()
        .update_texture(&state.image, state.handle);

    for (x, y, val) in state.pixels.iter() {
        state
            .image
            .put_pixel(x as u32, y as u32, val.to_image_rgba());
    }

    let viewport = main_camera().world_viewport();

    gameplay(state, c);

    if state.player.is_dead {
        draw_text(
            "YOU DIED",
            screen_to_world(vec2(10.0, 10.0)),
            WHITE,
            TextAlign::TopLeft,
        );
        draw_text(
            "press R to restart",
            screen_to_world(vec2(10.0, 30.0)),
            WHITE,
            TextAlign::TopLeft,
        );

        if is_key_pressed(KeyCode::R) {
            state.player.is_dead = false;
            state.map_change_timer = 0.0;
        }
    }

    draw_sprite_ex(
        state.handle,
        Vec2::ZERO,
        WHITE,
        0,
        DrawTextureParams {
            dest_size: Some(viewport.as_world_size()),
            ..Default::default()
        },
    );
}

pub fn gameplay(state: &mut GameState, c: &mut EngineContext) {
    let viewport = main_camera().world_viewport();

    let img_dims = vec2(state.image.width() as f32, state.image.height() as f32);

    let mouse_pct = mouse_world() / viewport + 0.5;
    let mouse_coord = (img_dims * mouse_pct).as_ivec2();

    let wall_size = 8;

    for (x, y, wall) in state.walls.iter() {
        if *wall {
            blit_at(
                &mut state.image,
                texture_id("wall"),
                vec2(x as f32, y as f32) * wall_size as f32,
                None,
                Some(isplat(wall_size)),
                BASE_COLOR.alpha(0.5),
                false,
                false,
            );
        }
    }

    let light_at = state.player.position;

    for (_, _, val) in state.lighting.iter_mut() {
        *val *= 1.0 - 10.0 * delta();
    }

    let ray_count = calculate_lighting(state, c, wall_size, mouse_coord, light_at);

    let mut max_light = 0.0;

    for (_, _, val) in state.lighting.iter() {
        if *val > max_light {
            max_light = *val;
        }
    }

    for (x, y, val) in state.lighting.iter() {
        let alpha = tone_map_intensity(*val / ray_count as f32 * 1000.0);

        let color = if state.player.is_dead {
            BLACK
        } else {
            Color::new(alpha, alpha, alpha, 1.0)
        };

        state
            .image
            .put_pixel(x as u32, y as u32, color.to_image_rgba());
    }

    player_update(state, c, mouse_coord);
    mobs_update(state, c, mouse_coord);

    if state.lighting.is_valid(mouse_coord) {
        state.image.put_pixel(
            mouse_coord.x as u32,
            mouse_coord.y as u32,
            RED.to_image_rgba(),
        );
    }
}

pub fn tone_map_intensity(value: f32) -> f32 {
    value / (1.0 + value)
}
