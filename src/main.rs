pub mod fuser;
pub mod fuser_systems;
pub mod links;
pub mod node;
pub mod pan_camera;
pub mod startup;

use avian2d::{PhysicsPlugins, prelude::Gravity};
use bevy::prelude::*;
use clap::{Arg, ArgAction, crate_version};

use crate::{
    fuser::{FuseParams, FuserState},
    fuser_systems::fuser_systems_plugin,
    links::links_plugin,
    pan_camera::PanCameraPlugin,
    startup::startup_plugin,
};

fn main() {
    let matches = clap::builder::Command::new("bevy_fuse")
        .version(crate_version!())
        .author("Alexandra Mikhaylova")
        .arg(
            Arg::new("mount-point")
                .long("mount-point")
                .value_name("MOUNT_POINT")
                .required(true)
                .help("Act as a client, and mount FUSE at given path"),
        )
        .arg(
            Arg::new("auto-unmount")
                .long("auto-unmount")
                .action(ArgAction::SetTrue)
                .help("Automatically unmount FUSE when process exits"),
        )
        .get_matches();

    let fuse_params = FuseParams {
        mountpoint: matches
            .get_one::<String>("mount-point")
            .unwrap()
            .to_string()
            .into(),
        auto_unmount: false,
    };
    App::new()
        .insert_resource(fuse_params)
        .init_non_send_resource::<FuserState>()
        .add_plugins(DefaultPlugins)
        .add_plugins(PanCameraPlugin)
        .add_plugins(PhysicsPlugins::default())
        .insert_resource(Gravity::ZERO)
        .add_plugins(startup_plugin)
        .add_plugins(links_plugin)
        .add_plugins(fuser_systems_plugin)
        .run();
}
