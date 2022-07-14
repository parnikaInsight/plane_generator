use bevy::{prelude::*, window::PresentMode};

mod mouse;//must include this in main, along with mod.rs, to find mod in crate
use mouse::mouse_events;

mod geometry;
use geometry::my_plane;

mod camera;
use camera::pan_orbit;

fn main() {
    println!("Hello, world!");
    let mut app = bevy::app::App::new(); //new vs empty //bevy::App has more trait implementations than bevy_app
    app.add_plugin(bevy::core::CorePlugin::default())
    //Events
        //.add_event::<mouse_events::MyCursorMoved>()

    //Resources
        //Window: event loops, changing contexts
        .insert_resource(Msaa { samples: 4 }) //remove jaggedness
        .insert_resource(WindowDescriptor { //must come before DefaultPlugins
            title: "I am a window!".to_string(),
            width: 1600.0, //sub w/ scr_width but change from u32 to f32
            height: 1000.0,
            present_mode: PresentMode::Fifo,
            ..default()
        })

    //Plugins
        .add_plugins(DefaultPlugins)
        .add_plugin(bevy::input::InputPlugin::default())

    //Systems
        //.add_system(mouse_events::print_mouse_events_system)
        .add_startup_system(my_plane::setup_plane)
        .add_startup_system(pan_orbit::spawn_camera)
        .add_system(pan_orbit::pan_orbit_camera)
        .run();
    //code never reaches here
}
