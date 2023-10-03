use glam::{IVec2, UVec2, Vec2};
pub use legion::*;
use rand::{rngs::ThreadRng, Rng};
use raylib::prelude::*;

use crate::{
    components::{CColor, CTransform, ChaseMouse, Wander},
    rendering::{execute_render_command_buffer, RenderCommandBuffer},
    systems::{chase_mouse, chase_mouse_system, entity_render_system, wander_system},
};

pub const FRAMES_PER_SECOND: u32 = 60;

pub struct State {
    pub running: bool,
    pub time_since_last_update: f32,

    pub ecs: World,
    pub resources: Resources,
    pub schedule: Schedule,
}

impl State {
    pub fn new() -> Self {
        let mut ecs = World::default();

        // spawn some entities that have Transform and a ccolor
        let mut rng: ThreadRng = rand::thread_rng();
        for _ in 0..1024 {
            let x = rng.gen_range(0.0..100.0); // Replace with your own range
            let y = rng.gen_range(0.0..100.0); // Replace with your own range

            ecs.push((
                CTransform { pos: Vec2 { x, y } },
                CColor {
                    color: Color {
                        r: rng.gen_range(0..255),
                        g: rng.gen_range(0..255),
                        b: rng.gen_range(0..255),
                        a: 255,
                    },
                },
                Wander,
                ChaseMouse,
            ));
        }

        for _ in 0..1000 {
            let x = rng.gen_range(0.0..100.0); // Replace with your own range
            let y = rng.gen_range(0.0..100.0); // Replace with your own range

            ecs.push((
                CTransform { pos: Vec2 { x, y } },
                CColor {
                    color: Color {
                        r: rng.gen_range(0..255),
                        g: rng.gen_range(0..255),
                        b: rng.gen_range(0..255),
                        a: 255,
                    },
                },
                Wander,
            ));
        }

        for _ in 0..1000 {
            let x = rng.gen_range(0.0..100.0); // Replace with your own range
            let y = rng.gen_range(0.0..100.0); // Replace with your own range

            ecs.push((
                CTransform { pos: Vec2 { x, y } },
                CColor {
                    color: Color {
                        r: rng.gen_range(0..255),
                        g: rng.gen_range(0..255),
                        b: rng.gen_range(0..255),
                        a: 255,
                    },
                },
                ChaseMouse,
            ));
        }

        let mut resources = Resources::default();
        let schedule = Schedule::builder()
            // .add_system(player_input::player_input_system())
            // .add_system(map_render::map_render_system())
            .add_system(wander_system())
            .add_system(chase_mouse_system())
            .add_system(entity_render_system())
            .build();

        Self {
            running: true,
            time_since_last_update: 0.0,

            ecs,
            resources,
            schedule,
        }
    }
}

pub fn process_events_and_input(rl: &mut RaylibHandle, state: &mut State) {
    if rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_ESCAPE) {
        state.running = false;
    }
}

pub fn step(rl: &mut RaylibHandle, rlt: &mut RaylibThread, state: &mut State) {
    let mouse_pos_rl = rl.get_mouse_position();
    let mouse_pos = Vec2::new(mouse_pos_rl.x, mouse_pos_rl.y);
    state.resources.insert(mouse_pos);

    state.schedule.execute(&mut state.ecs, &mut state.resources);
}

pub fn draw(state: &State, d: &mut RaylibTextureMode<RaylibDrawHandle>) {
    d.draw_text("ECS!", 12, 12, 12, Color::WHITE);
    let mouse_pos = d.get_mouse_position();
    d.draw_circle(mouse_pos.x as i32, mouse_pos.y as i32, 6.0, Color::GREEN);

    let mut render_command_buffer = state.resources.get_mut::<RenderCommandBuffer>().unwrap();
    execute_render_command_buffer(d, &mut render_command_buffer);

    // let angle = d.get_time() as f32;

    // let center = Vec2::new(d.get_screen_width() as f32, d.get_screen_height() as f32) / 2.0;
    // let offset = center / 4.0;

    // for i in 0..3 {
    //     let rot = glam::Mat2::from_angle(angle + i as f32 * 90.0);
    //     let rect_pos_rotated = rot * offset + center;

    //     let size =
    //         (((d.get_time() as f32 + i as f32 * 1.0) * 2.0).sin() + 1.0) / 2.0 * offset.y + 4.0;
    //     d.draw_rectangle(
    //         rect_pos_rotated.x as i32,
    //         rect_pos_rotated.y as i32,
    //         size as i32,
    //         size as i32,
    //         Color::RED,
    //     );
    // }
}
