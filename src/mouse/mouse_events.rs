use bevy::{
    input::mouse::{MouseButtonInput, MouseMotion, MouseWheel},
    prelude::*,
};

#[derive(Debug)]
pub struct MyCursorMoved {
    pub pos: Vec2,
    pub delta: Vec2,
}

/// This system prints out all mouse events as they come in
pub fn print_mouse_events_system(
    buttons: Res<Input<MouseButton>>,
    mut mouse_button_input_events: EventReader<MouseButtonInput>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut cursor_moved_events: EventReader<MyCursorMoved>,
    mut mouse_wheel_events: EventReader<MouseWheel>,
) {
    for event in mouse_button_input_events.iter() {
        info!("{:?}", event);
    }

    // for event in mouse_motion_events.iter() {
    //     info!("{:?}", event);
    // }

    for event in cursor_moved_events.iter() {
        info!("{:?}", event);
        println!("hettt");
    }

    for event in mouse_wheel_events.iter() {
        info!("{:?}", event);
    }
}