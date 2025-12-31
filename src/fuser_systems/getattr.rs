use bevy::prelude::*;
use libc::ENOENT;

use crate::{
    fuser::FuserState,
    fuser_systems::{TTL, helpers::inode_to_entity},
    node::{INode, directory::RootDirectory, file::File},
};

pub fn getattr_system(
    mut fuser_state: NonSendMut<FuserState>,
    root_entity: Res<RootDirectory>,
    nodes: Query<(&INode, Option<&File>)>,
) {
    let root_entity = root_entity.0;
    if let Some(ref mut state) = fuser_state.0 {
        while let Ok((data, reply)) = state.getattr.try_recv() {
            trace!("Recieved 1 message!");
            let e = inode_to_entity(data.ino, root_entity);
            if let Ok((metadata, file)) = nodes.get(e) {
                trace!("Found entry, replying...! {metadata:?}");
                reply.attr(&TTL, &metadata.get_file_attrb_obj(data.ino, file));
            } else {
                trace!("Entry not found!");
                reply.error(ENOENT);
            }
        }
    };
}
