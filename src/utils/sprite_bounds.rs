use bevy::prelude::*;

/// Get if some point is inside a sprite's bounding box.
pub fn point_in_sprite_bounds(point: Vec2, sprite: &Sprite, transform: &GlobalTransform) -> bool {
    if let Some(custom_size) = sprite.custom_size {
        let half_size = custom_size / 2.0;
        let bounding_box =
            Rect::from_center_half_size(transform.translation().truncate(), half_size);
        if point.x >= bounding_box.min.x
            && point.x <= bounding_box.max.x
            && point.y >= bounding_box.min.y
            && point.y <= bounding_box.max.y
        {
            true
        } else {
            false
        }
    }else {
        false
    }
}
