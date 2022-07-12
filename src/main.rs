use bevy::{prelude::*, window::PresentMode};
use bevy_config_cam::*;

mod mouse;
use mouse::mouse_events;

mod geometry;
use geometry::plane;

mod basics;
use basics::transform;

mod camera;
use camera::perspective;
use camera::camera_structure::MainCamera;
use camera::main_camera;

// settings
const SCR_WIDTH: u32 = 1600;
const SCR_HEIGHT: u32 = 1200;

fn main() {
    println!("Hello, world!");
    let mut app = bevy::app::App::new(); //new vs empty //bevy::App has more trait implementations than bevy_app
    app.add_plugin(bevy::core::CorePlugin::default())
    //Events
        .add_event::<mouse_events::MyCursorMoved>()

    //Resources
        //Window: event loops, changning contexts
        .insert_resource(Msaa { samples: 4 }) //remove jaggedness
        .insert_resource(WindowDescriptor { //must come before DefaultPlugins
            title: "I am a window!".to_string(),
            width: 600.0, //sub w/ scr_width but change from u32 to f32
            height: 400.0,
            present_mode: PresentMode::Fifo,
            ..default()
        })
        //.insert_resource(MainCamera::new(SCR_WIDTH as f32 / SCR_HEIGHT as f32))

    //Plugins
        .add_plugins(DefaultPlugins)
        .add_plugin(bevy::input::InputPlugin::default())
        .add_plugin(ConfigCam)
        .insert_resource(MovementSettings {
            sensitivity: 0.00015, // default: 0.00012
            speed: 12.0,          // default: 12.0
            ..Default::default()
        })
        .insert_resource(PlayerSettings {
            pos: Vec3::new(2., 0., 0.),
            player_asset: "models/craft_speederA.glb#Scene0",
            ..Default::default()
        })

    //Systems
        //.add_system(mouse_events::print_mouse_events_system)
        .add_startup_system(plane::setup_plane)
        //.add_startup_system(perspective::setup_camera)

       // .add_system(main_camera::main_camera_update) //not working yet ^works
        .run();
    //code never reaches here
}

