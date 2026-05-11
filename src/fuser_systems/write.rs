use std::io::Write;

use bevy::prelude::*;
use fuser::{Errno, FileType};

use crate::{
    fuser::FuserState,
    fuser_systems::helpers::{fh_to_entity, inode_to_entity},
    node::{INode, directory::RootDirectory, file::File, filehandle::FileHandle},
};

pub fn write_system(
    mut fuser_state: NonSendMut<FuserState>,
    root_entity: Res<RootDirectory>,
    mut nodes: Query<(&INode, Option<&mut File>)>,
    fhs: Query<&FileHandle>,
) {
    let root_entity = root_entity.0;
    if let Some(ref mut state) = fuser_state.0 {
        while let Ok((data, reply)) = state.write.try_recv() {
            trace!("Received 1 message!");
            let e = inode_to_entity(data.ino, root_entity);
            if let Ok((metadata, f)) = nodes.get_mut(e) {
                let fh = fh_to_entity(data.fh);
                if let Ok(fh_c) = fhs.get(fh) {
                    if fh_c.parent == e && fh_c.write {
                        if metadata.kind == FileType::Directory {
                            reply.error(Errno::EISDIR)
                        } else if let Some(mut f) = f {
                            let write_offset_start: usize = data.offset.try_into().unwrap();
                            let write_offset_end = write_offset_start + data.data.len();
                            let orrd_write_offset_start = write_offset_start.min(f.content.len());
                            let orrd_write_offset_end = write_offset_end.min(f.content.len());
                            let orrd_len = orrd_write_offset_end - orrd_write_offset_start;
                            if orrd_len != 0 {
                                f.content[orrd_write_offset_start..orrd_write_offset_end]
                                    .copy_from_slice(&data.data[..orrd_len]);
                            }
                            f.content.write(&data.data[orrd_len..]).unwrap();
                            reply.written(data.data.len() as u32);
                        } else {
                            trace!("this is not file!");
                            reply.error(Errno::ENOSYS);
                        }
                    } else {
                        reply.error(Errno::EACCES);
                    }
                } else {
                    trace!("Handle not found!");
                    reply.error(Errno::ENOENT);
                }
            } else {
                trace!("Entry not found!");
                reply.error(Errno::ENOENT);
            }
        }
    };
}
