lazy_static! {
    static ref RENDER_COMMAND_BUFFER: Mutex<Vec<DrawCommand>> = Mutex::new(Vec::new());
}

pub fn add_render_command(command: DrawCommand) {
    let mut buffer = RENDER_COMMAND_BUFFER.lock().unwrap();
    buffer.push(command);
}

pub fn clear_render_command_buffer() {
    let mut buffer = RENDER_COMMAND_BUFFER.lock().unwrap();
    buffer.clear();
}

#[derive(Clone)]
pub enum DrawCommand {
    ClearScreen,
    ColoredSquare { pos: UVec2, color: Color },
}

pub fn execute_render_command_buffer(rl: &mut RaylibHandle, thread: &RaylibThread) {
    let mut d = rl.begin_drawing(thread);

    let mut buffer = RENDER_COMMAND_BUFFER.lock().unwrap();
    buffer.sort_by_key(|&(layer, _)| layer);

    for (_, command) in buffer.iter() {
        match command {
            DrawCommand::ClearScreen => {
                d.clear_background(Color::WHITE);
            }
            DrawCommand::ColoredSquare { pos, color } => {
                let rectangle = Rectangle::new(pos.x as f32, pos.y as f32, 50.0, 50.0);
                d.draw_rectangle_rec(rectangle, *color);
            }
        }
    }

    // Optionally clear the buffer for the next frame
    buffer.clear();
}
