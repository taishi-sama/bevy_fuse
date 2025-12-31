use bevy::prelude::*;
use libc::{EACCES, ENOENT};

use crate::{
    fuser::FuserState,
    fuser_systems::helpers::{fh_to_entity, inode_to_entity},
    node::{INode, directory::RootDirectory, file::File, filehandle::FileHandle},
};

pub fn read_system(
    mut fuser_state: NonSendMut<FuserState>,
    root_entity: Res<RootDirectory>,
    nodes: Query<(&INode, Option<&File>)>,
    fhs: Query<&FileHandle>,
) {
    let root_entity = root_entity.0;
    if let Some(ref mut state) = fuser_state.0 {
        while let Ok((data, reply)) = state.read.try_recv() {
            trace!("Recieved 1 message!");
            let e = inode_to_entity(data.ino, root_entity);
            if let Ok((_metadata, f)) = nodes.get(e) {
                let fh = fh_to_entity(data.fh);
                if let Ok(fh_c) = fhs.get(fh) {
                    if fh_c.parent == e && fh_c.read {
                        let file_size = f.map(|x| x.get_size()).unwrap_or(0);
                        let read_size = data
                            .size
                            .min(file_size.saturating_sub(data.offset as u64) as u32);
                        if let Some(f) = f
                            && read_size != 0
                        {
                            reply.data(
                                &f.content[data.offset as usize
                                    ..(data.offset as u64 + read_size as u64) as usize],
                            );
                        } else {
                            reply.data(&[])
                        }
                    } else {
                        reply.error(EACCES);
                    }
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
