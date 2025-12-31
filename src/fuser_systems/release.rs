use bevy::prelude::*;
use libc::ENOENT;

use crate::{
    fuser::FuserState,
    fuser_systems::helpers::{fh_to_entity, inode_to_entity},
    node::{INode, directory::RootDirectory},
};

pub fn release_system(
    mut fuser_state: NonSendMut<FuserState>,
    root_entity: Res<RootDirectory>,
    nodes: Query<&INode>,
    mut commands: Commands,
) {
    let root_entity = root_entity.0;
    if let Some(ref mut state) = fuser_state.0 {
        while let Ok((data, reply)) = state.release.try_recv() {
            trace!("Recieved 1 message!");
            let e = inode_to_entity(data.ino, root_entity);
            if let Ok(_metadata) = nodes.get(e) {
                let fh = fh_to_entity(data.fh);
                if let Ok(mut c) = commands.get_entity(fh) {
                    c.despawn();
                    trace!("File descriptor destroyed! fh: {fh}");
                    reply.ok();
                } else {
                    trace!("Handle not found!");
                    reply.error(ENOENT);
                }
            } else {
                trace!("Entry not found!");
                reply.error(ENOENT);
            }
        }
    };
}
