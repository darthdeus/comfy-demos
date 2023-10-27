use crate::*;

pub fn calculate_lighting(
    state: &mut GameState,
    _c: &mut EngineContext,
    wall_size: i32,
    mouse_coord: IVec2,
    light_at: Vec2,
) -> i32 {
    let rays_per_px = 1;
    let max_len = 100;
    let mut total_steps = 0;
    let mut ray_count = 0;

    let start_time = Instant::now();

    let mouse_dir = (mouse_coord.as_vec2() - state.player.position).normalize_or_right();

    for x in 0..state.image.width() {
        for y in 0..state.image.height() {
            if coin_toss(0.85) {
                continue;
            }

            let blob = 6.0;
            let target_px = vec2(x as f32, y as f32);

            let orig_ray_dir = if coin_toss(0.2) {
                (target_px - light_at).normalize_or_right().wiggle(0.2)
            } else {
                mouse_dir.wiggle(0.7)
            };

            // let orig_ray_dir = mouse_dir;

            let light_at = light_at + random_vec(-blob, blob);
            // let orig_ray_dir = (target_px - light_at).normalize_or_right().wiggle(0.2);

            for _ in 0..1 {
                // let orig_ray_dir = (mouse_coord.as_vec2() - light_at).normalize();
                // let orig_ray_dir = random_dir();

                for _b in 0..rays_per_px {
                    let mut ray_dir = orig_ray_dir;
                    let mut px = light_at;

                    ray_count += 1;

                    let mut bounces = 0;

                    for _ in 0..max_len {
                        px += ray_dir;
                        total_steps += 1;

                        let distance = light_at.distance(px);
                        let attenuation = 1.0 / (distance + 1.0);

                        // let pct = i as f32 / max_len as f32;
                        let wall_idx = (px / wall_size as f32).ceil().as_ivec2();
                        let mut hit = false;

                        if *state.walls.get_clamped(wall_idx.x, wall_idx.y) {
                            hit = true;
                            px -= ray_dir;

                            bounces += 1;

                            // intensity = (intensity as f32 * 0.3) as i32;

                            let prev_wall_idx =
                                ((px - ray_dir) / wall_size as f32).ceil().as_ivec2();

                            if prev_wall_idx.x != wall_idx.x {
                                ray_dir.x = -ray_dir.x; // Hit was horizontal (left or right side)
                            } else if prev_wall_idx.y != wall_idx.y {
                                ray_dir.y = -ray_dir.y; // Hit was vertical (top or bottom side)
                            }

                            ray_dir = ray_dir.wiggle(0.35);
                        }

                        let px = if hit {
                            // px
                            (px - ray_dir).as_ivec2()
                        } else {
                            px.as_ivec2()
                        };

                        let intensity = attenuation * 0.3f32.powf(bounces as f32); // * 0.5f32.powf(bounces as f32);

                        if state.lighting.is_valid(px) {
                            *state.lighting.get_clamped_mut(px.x, px.y) += intensity;
                        }
                    }
                }
            }
        }
    }

    let duration = start_time.elapsed().as_secs_f32();

    let max_light_value = state
        .lighting
        .iter()
        .map(|(_, _, val)| OrderedFloat(*val))
        .max()
        .map(|x| x.0)
        .unwrap_or(1.0);

    state.max_light_value = max_light_value;

    if cfg!(feature = "dev") {
        draw_text(
            &format!(
                "rays: {}\nsteps: {}\nresolution: {}x{}\nrays per px: {}\nframe time: {:.0}ms\n",
                ray_count,
                total_steps,
                state.image.width(),
                state.image.height(),
                rays_per_px,
                duration * 1000.0,
            ),
            screen_to_world(Vec2::ZERO),
            GREEN,
            TextAlign::TopLeft,
        );
    }

    ray_count
}
