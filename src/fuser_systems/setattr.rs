use bevy::prelude::*;
use libc::{EACCES, ENOENT, ENOSYS};

use crate::{
    fuser::FuserState,
    fuser_systems::{
        TTL,
        helpers::{fh_to_entity, inode_to_entity},
    },
    node::{INode, directory::RootDirectory, file::File, filehandle::FileHandle},
};

pub fn setattr_system(
    mut fuser_state: NonSendMut<FuserState>,
    root_entity: Res<RootDirectory>,
    mut nodes: Query<(&mut INode, Option<&mut File>)>,
    fhs: Query<&FileHandle>,
) {
    let root_entity = root_entity.0;
    if let Some(ref mut state) = fuser_state.0 {
        while let Ok((data, reply)) = state.setattr.try_recv() {
            trace!("Recieved 1 message!");
            let e = inode_to_entity(data.ino, root_entity);
            if let Ok((metadata, mut f)) = nodes.get_mut(e) {
                if let Some(size) = data.size {
                    if let Some(fh) = data.fh {
                        if let Ok(file_handler) = fhs.get(fh_to_entity(fh))
                            && file_handler.write
                        {
                            if let Some(ref mut f) = f {
                                f.truncate(size);
                            }
                        } else {
                            reply.error(EACCES);
                            continue;
                        }
                    }
                    if let Some(ref mut f) = f {
                        f.truncate(size);
                    }
                }
                if let Some(_) = data.gid {
                    reply.error(ENOSYS);
                    continue;
                }
                if let Some(_) = data.uid {
                    reply.error(ENOSYS);
                    continue;
                }
                if let Some(_) = data.mode {
                    reply.error(ENOSYS);
                    continue;
                }
                if let Some(_) = data.atime {
                    reply.error(ENOSYS);
                    continue;
                }
                if let Some(_) = data.mtime {
                    reply.error(ENOSYS);
                    continue;
                }
                if let Some(_) = data.ctime {
                    reply.error(ENOSYS);
                    continue;
                }
                reply.attr(&TTL, &metadata.get_file_attrb_obj_mut(data.ino, f));
            } else {
                trace!("Entry not found!");
                reply.error(ENOENT);
            }
        }
    };
}
