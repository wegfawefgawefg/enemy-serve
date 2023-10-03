use glam::{UVec2, Vec2};
use legion::world::SubWorld;
pub use legion::*;
use rand::{
    rngs::{StdRng, ThreadRng},
    Rng,
};
use raylib::prelude::{Color, Vector2};

use crate::{
    components::{CColor, CTransform, ChaseMouse, Wander},
    rendering::{DrawCommand, RenderCommandBuffer},
};

// system to make all enemies go to player

// system to damage player if touching player

// system to control InputControlled entities

// render system
/* fetch position and sprite entities, and just blit them with a fixed size with the given position */
#[system]
#[read_component(CTransform)]
#[read_component(CColor)]
pub fn entity_render(ecs: &SubWorld, #[resource] render_command_buffer: &mut RenderCommandBuffer) {
    render_command_buffer.clear();
    <(&CTransform, &CColor)>::query()
        .iter(ecs)
        .for_each(|(transform, ccolor)| {
            render_command_buffer.push(DrawCommand::ColoredSquare {
                pos: transform.pos.as_uvec2(),
                color: ccolor.color,
            });
        });
}

// wander system
const WANDER_RANGE: f32 = 1.0;
#[system]
#[write_component(CTransform)]
pub fn wander(ecs: &mut SubWorld, #[resource] rng: &mut StdRng) {
    let mut query = <&mut CTransform>::query().filter(component::<Wander>());
    for ctransform in query.iter_mut(ecs) {
        let dx = rng.gen_range(-WANDER_RANGE..WANDER_RANGE);
        let dy = rng.gen_range(-WANDER_RANGE..WANDER_RANGE);
        ctransform.pos += Vec2::new(dx, dy);
    }
}

#[system]
#[write_component(CTransform)]
pub fn chase_mouse(ecs: &mut SubWorld, #[resource] mouse_pos: &Vec2) {
    let mut query = <&mut CTransform>::query().filter(component::<ChaseMouse>());
    for ctransform in query.iter_mut(ecs) {
        let pos = ctransform.pos;
        let delta = *mouse_pos - pos;
        ctransform.pos += delta.normalize() * Vec2::ONE;
    }
}
