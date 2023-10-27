use crate::*;

pub struct Mob {
    pub position: Vec2,
    pub velocity: Vec2,
    pub move_timer: f32,
    pub move_target: Vec2,

    pub attack_timer: f32,
    pub is_attacking: bool,
    pub time_visible: f32,
    pub should_despawn: bool,
}

impl Mob {
    pub fn new(position: Vec2) -> Self {
        Self {
            position,
            move_timer: 0.0,
            velocity: Vec2::ZERO,
            move_target: position,
            attack_timer: 2.0,
            time_visible: 0.0,
            is_attacking: false,
            should_despawn: false,
        }
    }
}

pub fn mobs_update(state: &mut GameState, c: &EngineContext, _mouse_coord: IVec2) {
    let was_monster_visible = state.is_monster_visible;

    state.is_monster_visible = false;

    for mob in state.mobs.iter_mut() {
        let mob_coord = mob.position.as_ivec2();

        mob.velocity = vec2(0.0, 0.1);
        mob.position += mob.velocity * c.delta;

        if mob.velocity.length() > 0.0 {
            mob.move_timer += c.delta;
        } else {
            mob.move_timer = 0.0;
        }

        let animation_time = 0.25;

        if mob.move_timer > animation_time {
            mob.move_timer -= animation_time;
        }

        let aim_right = mob.position.x < state.player.position.x;

        let _frame = ((3.0 * mob.move_timer / animation_time).floor() as i32) % 3;

        let mob_light_val_raw = *state.lighting.get_clamped_v(mob_coord);
        let mob_light_val = tone_map_intensity(mob_light_val_raw);

        if mob.is_attacking {
            mob.attack_timer -= delta();

            if mob.attack_timer <= 0.0 {
                play_sound("player-death");
                mob.should_despawn = true;
                mob.is_attacking = false;
                state.player.is_dead = true;
            }
        }

        if mob_light_val > 0.6 {
            state.is_monster_visible = true;

            mob.time_visible += delta();

            blit_at(
                &mut state.image,
                texture_id("enemy"),
                mob.position,
                None,
                Some(ivec2(12, 12)),
                WHITE,
                true,
                aim_right,
            );
        }

        if mob.time_visible >= 1.0 {
            mob.is_attacking = true;
        }
    }

    state.mobs.retain(|x| !x.should_despawn);

    if !was_monster_visible && state.is_monster_visible {
        play_sound("breathing");
    }
}
