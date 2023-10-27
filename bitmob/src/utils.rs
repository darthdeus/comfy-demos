use comfy::image::GenericImage;
use comfy::image::GenericImageView;

use crate::*;

pub fn blit_at(
    target: &mut DynamicImage,
    texture: TextureHandle,
    position: Vec2,
    source_rect: Option<IRect>,
    dest_size: Option<IVec2>,
    tint: Color,
    flip_v: bool,
    flip_h: bool,
) {
    let assets = ASSETS.borrow_mut();
    let image_map = assets.texture_image_map.lock();

    if let Some(image) = image_map.get(&texture) {
        let img_dims = ivec2(image.width() as i32, image.height() as i32);
        let src_rect = source_rect.unwrap_or(IRect::new(ivec2(0, 0), img_dims));
        let dest_size = dest_size.unwrap_or(img_dims);
        let size_offset = src_rect.size.as_vec2() / 2.0;

        for x in 0..dest_size.x {
            for y in 0..dest_size.y {
                let flipped_x = if flip_h { dest_size.x - 1 - x } else { x };
                let flipped_y = if flip_v { dest_size.y - 1 - y } else { y };

                let px = image.get_pixel(
                    ((flipped_x + src_rect.offset.x) as u32).clamp(0, image.width() - 1),
                    ((flipped_y + src_rect.offset.y) as u32).clamp(0, image.height() - 1),
                );

                if px.0[3] > 0 {
                    let put_at = position + vec2(x as f32, y as f32) - size_offset;
                    let put_at = put_at.as_uvec2();

                    let color = Into::<Color>::into(px) * tint;

                    if put_at.x < target.width() && put_at.y < target.height() {
                        target.put_pixel(
                            put_at.x.clamp(0, target.width() - 1),
                            put_at.y.clamp(0, target.height() - 1),
                            color.to_image_rgba(),
                        );
                    }
                }
            }
        }
    }
}
