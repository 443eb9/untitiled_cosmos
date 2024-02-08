use bevy::math::{DVec2, Vec2};

macro_rules! impl_aabb2d {
    ($name:ident, $acc:ty, $unit_acc:ty) => {
        #[derive(Clone, Copy)]
        pub struct $name {
            pub min: $acc,
            pub max: $acc,
        }

        impl $name {
            pub fn new(min: $acc, max: $acc) -> Self {
                Self { min, max }
            }

            pub fn new_sep(min_x: $unit_acc, max_x: $unit_acc, min_y: $unit_acc, max_y: $unit_acc) -> Self {
                Self {
                    min: <$acc>::new(min_x, min_y),
                    max: <$acc>::new(max_x, max_y),
                }
            }
        }
    };
}

impl_aabb2d!(DAabb2d, DVec2, f64);
impl_aabb2d!(Aabb2d, Vec2, f32);
