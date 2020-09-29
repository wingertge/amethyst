use crate::Entity;
use amethyst_core::ecs::{Component, DenseVecStorage};

/// Mask this entity to the entity specified. Its rendering cannot extend past the bounds of `to`.
#[derive(Clone, Copy, Debug)]
pub struct Mask {
    /// The entity to constrain this to. Does nothing if the entity has no `UiTransform`cargo +stable fmt --all
    pub to: Entity,
}

impl Component for Mask {
    type Storage = DenseVecStorage<Self>;
}
