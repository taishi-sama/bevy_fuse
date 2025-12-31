pub mod getattr;
pub mod helpers;
pub mod lookup;
pub mod mkdir;
pub mod mknod;
pub mod open;
pub mod read;
pub mod readdir;
pub mod release;
pub mod setattr;
pub mod statfs;
pub mod write;

use std::time::Duration;

use bevy::app::{App, FixedUpdate};

use crate::fuser_systems::{
    getattr::getattr_system, lookup::lookup_system, mkdir::mkdir_system, mknod::mknod_system,
    open::open_system, read::read_system, readdir::readdir_system, release::release_system,
    setattr::setattr_system, statfs::statfs_system, write::write_system,
};

const TTL: Duration = Duration::from_secs(1); // 1 second

pub fn fuser_systems_plugin(app: &mut App) {
    app.add_systems(
        FixedUpdate,
        (
            getattr_system,
            readdir_system,
            statfs_system,
            lookup_system,
            open_system,
            release_system,
            read_system,
            write_system,
            setattr_system,
            mkdir_system,
            mknod_system,
        ),
    );
}
