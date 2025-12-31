use avian2d::prelude::{AngularDamping, Collider, DistanceJoint, LinearDamping, RigidBody};
use bevy::prelude::*;

use crate::{
    fuser::{FuseParams, FuserState, construct_fusecs},
    node::{
        directory::{Directory, RootDirectory, RootDirectoryMarker, StoredIn},
        file::File,
    },
    pan_camera::PanCamera,
};

pub fn startup_plugin(app: &mut App) {
    app.add_systems(Startup, (startup, start_fuse).chain());
}
pub fn startup(mut c: Commands) {
    c.spawn((Camera2d, PanCamera::default()));

    let parent_entity = c
        .spawn((
            Sprite::from_color(Color::linear_rgb(0.05, 0.05, 0.05), vec2(20.0, 20.0)),
            Transform::from_xyz(0.0, 0.0, 0.0),
            RootDirectoryMarker,
            Directory {},
            Name::new("Root"),
            RigidBody::Static,
            Collider::rectangle(20.0, 20.0),
        ))
        .id();
    c.insert_resource(RootDirectory(parent_entity));
    for i in 0..4 {
        let child_entity = c
            .spawn((
                Sprite::from_color(Color::linear_rgb(0.1, 0.5, 0.5), vec2(15.0, 15.0)),
                Transform::from_xyz(30.0, i as f32 * 30.0 - 2.0 * 30.0, 0.0),
                File {
                    content: b"Hello World!".to_vec(),
                    ..File::default()
                },
                StoredIn {
                    parent: parent_entity,
                },
                Name::new(format!("random_file{i}.txt")),
                RigidBody::Dynamic,
                Collider::rectangle(10.0, 10.0),
                LinearDamping(0.8),
                AngularDamping(0.8),
            ))
            .id();
        let _joint = c.spawn((
            DistanceJoint::new(parent_entity, child_entity).with_limits(25.0, 55.0),
            ChildOf(child_entity),
        ));
    }
}

pub fn start_fuse(fuse_conf: Res<FuseParams>, mut fuser_state: NonSendMut<FuserState>) {
    let fuser = construct_fusecs(&fuse_conf);
    fuser_state.0 = Some(fuser)
}
