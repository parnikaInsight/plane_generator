use dolly::prelude::*;
use glam::{Mat4, Vec3};

use crate::basics::transform::Transform;

const CAMERA_FOV_DEGREES: f32 = 45.0;

pub struct MainCamera {
    pub camera: Camera, //rendering?
    pub camera_rig: CameraRig, //camera positioning
}

impl MainCamera {
    pub fn new(aspect_ratio: f32) -> Self {
        let camera_rig = CameraRig::builder()
            .with(YawPitch::new().yaw_degrees(45.0).pitch_degrees(-30.0)) //rotation
            .with(Position::new(dolly::glam::Vec3::ZERO))
            .with(Smooth::new_position_rotation(1.0, 1.0))
            .with(Arm::new(dolly::glam::Vec3::Z * 9.0)) //offset
            .build();
        let camera = Camera::new(
            Transform::identity(), //scale = 1, no rotation or translation
            CAMERA_FOV_DEGREES.to_radians(), //field of view
            aspect_ratio, // width to height 
        );
        Self { camera, camera_rig }
    }
}

pub struct Camera {
    pub transform: Mat4, //Mat4 for translation
    pub perspective_projection: Mat4, // TODO: hm... it really doesnt need to be a part of the camera
     //like OpenGL's gluPerspective — set up a perspective projection matrix (fov, aspect, distances)
}

impl Camera {
    pub fn new(transform: Transform, fov: f32, aspect_ratio: f32) -> Self {
        // convert between glam 13.0 and glam 18.0
        let transform = Mat4::from_cols_array(&transform.compute_matrix().to_cols_array());

        Self {
            transform,
            perspective_projection: Mat4::perspective_rh_gl(fov, aspect_ratio, 0.1, 100.0),
        }
    }

    pub fn world_to_camera_view(&self) -> Mat4 {
        self.transform.inverse() //inverse of matrix gives camera view ??
    }

    pub fn position(&self) -> Vec3 {
        let (_, _, t) = self.transform.to_scale_rotation_translation();
        t
    }
}
