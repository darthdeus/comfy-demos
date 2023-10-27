use crate::*;

pub fn player_update(state: &mut GameState, _c: &EngineContext, mouse_coord: IVec2) {
    let mut move_vec = Vec2::ZERO;
    let mut any_input = false;
    let delta = delta();

    if is_key_down(KeyCode::A) {
        any_input = true;
        move_vec.x -= 1.0;
    }
    if is_key_down(KeyCode::D) {
        any_input = true;
        move_vec.x += 1.0;
    }

    if is_key_down(KeyCode::W) {
        any_input = true;
        move_vec.y += 1.0;
    }
    if is_key_down(KeyCode::S) {
        any_input = true;
        move_vec.y -= 1.0;
    }

    if any_input {
        if !state.player.moved {
            play_music_id_ex(sound_id("scary-music"), PlaySoundParams { looped: true });
            state.start_time = get_time() as f32;
        }
        state.player.moved = true;

        move_vec = move_vec.normalize_or_right();
        state.player.footsteps_timer -= delta;
        let move_speed = 24.0;

        if state.player.footsteps_timer <= 0.0 {
            state.player.footsteps_timer = 0.35;
            play_sound(&format!("footsteps-{}", gen_range(0, 7)));
        }

        state.player.position += move_vec * move_speed * delta;
    } else {
        state.player.footsteps_timer = 0.0;
    }

    let enough_time_passed = get_time() as f32 - state.start_time > 3.0;

    if any_input {
        state.player.move_timer += delta;
    } else {
        state.player.move_timer = 0.0;
    }

    let animation_time = 0.3;

    if state.player.move_timer > animation_time {
        state.player.move_timer -= animation_time;
    }

    let aim_right = state.player.position.x < mouse_coord.x as f32;
    let look_down = state.player.position.y > mouse_coord.y as f32;

    let frame = (3.0 * state.player.move_timer / animation_time).floor() as i32;

    let offset = ivec2(8 * frame, if look_down { 9 } else { 0 });
    let size = ivec2(8, 9);

    blit_at(
        &mut state.image,
        texture_id("player"),
        state.player.position,
        Some(IRect::new(offset, size)),
        Some(ivec2(8, 9)),
        WHITE,
        true,
        aim_right,
    );

    let aim_off = if aim_right { 1.0 } else { -1.0 };

    if state.bullet.is_none() && state.is_monster_visible && enough_time_passed {
        blit_at(
            &mut state.image,
            texture_id("gun"),
            state.player.position + vec2(6.0 * aim_off, -1.0),
            None,
            Some(ivec2(8, 8)),
            WHITE,
            true,
            aim_right,
        );

        if is_mouse_button_pressed(MouseButton::Left) {
            play_sound("shotgun");

            state.bullet = Some(Bullet {
                position: state.player.position,
                velocity: (mouse_coord.as_vec2() - state.player.position).normalize_or_right(),
            });
        }
    }

    if let Some(bullet_pos) = state.bullet.as_ref().map(|x| x.position) {
        state.mobs.retain_mut(|mob| {
            if mob.position.distance(bullet_pos) < 6.0 {
                play_sound("monster-hit");
                state.bullet = None;
                false
            } else {
                true
            }
        });
    }

    if enough_time_passed {
        state.map_change_timer -= delta;

        if state.map_change_timer <= 0.0 {
            state.map_change_timer = random_range(6.0, 17.0);
            play_sound("map-change");

            for (x, y, val) in state.lighting.iter() {
                let val = tone_map_intensity(*val);

                if val < 0.3 {
                    // if is_mob_close {
                    //     state.walls[(x, y)] = false;
                    // } else {
                    state.walls[(x, y)] = coin_toss(random_range(0.1, 0.2));
                    // }
                }
            }
        }
    }

    if state.mobs.is_empty() && enough_time_passed {
        let mut candidates = Vec::new();

        for (x, y, val) in state.lighting.iter() {
            let val = tone_map_intensity(*val);
            if val < 0.3 {
                candidates.push(vec2(x as f32, y as f32));
            }
        }

        let position = candidates.choose().copied().unwrap_or(Vec2::ZERO);
        state.mobs.push(Mob::new(position));

        info!("Monster spawned");
    }

    if let Some(bullet) = state.bullet.as_mut() {
        bullet.position += bullet.velocity * delta * 200.0;

        blit_at(
            &mut state.image,
            texture_id("bullet"),
            bullet.position,
            None,
            Some(ivec2(8, 8)),
            WHITE,
            true,
            aim_right,
        );
    }
}
