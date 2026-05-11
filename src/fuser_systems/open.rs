use bevy::{color::palettes::css, prelude::*};
use fuser::{Errno, FopenFlags, OpenAccMode};

use crate::{
    fuser::FuserState,
    fuser_systems::helpers::{entity_to_fh, inode_to_entity},
    node::{INode, directory::RootDirectory, filehandle::FileHandle},
};
const FMODE_EXEC: i32 = 0x20;

pub fn open_system(
    mut fuser_state: NonSendMut<FuserState>,
    root_entity: Res<RootDirectory>,
    nodes: Query<&INode>,
    mut commands: Commands,
) {
    let root_entity = root_entity.0;
    if let Some(ref mut state) = fuser_state.0 {
        while let Ok((data, reply)) = state.open.try_recv() {
            trace!("Received 1 message!");
            let flags = data.flags;
            let (_access_mask, read, write) = match flags.acc_mode() {
                OpenAccMode::O_RDONLY => {
                    // Behavior is undefined, but most filesystems return EACCES
                    if flags.0 & libc::O_TRUNC != 0 {
                        reply.error(Errno::EACCES);
                        break;
                    }
                    if flags.0 & FMODE_EXEC != 0 {
                        // Open is from internal exec syscall
                        (libc::X_OK, true, false)
                    } else {
                        (libc::R_OK, true, false)
                    }
                }
                OpenAccMode::O_WRONLY => (libc::W_OK, false, true),
                OpenAccMode::O_RDWR => (libc::R_OK | libc::W_OK, true, true),
            };
            let e = inode_to_entity(data.ino, root_entity);
            if let Ok(_metadata) = nodes.get(e) {
                let fh = commands.spawn((
                    FileHandle {
                        parent: e,
                        read,
                        write,
                    },
                    ChildOf(e),
                    Sprite::from_color(css::WHEAT, vec2(5.0, 5.0)),
                    Transform::from_xyz(0.0, 11.0, 1.0),
                ));
                let fh = entity_to_fh(fh.id());
                trace!("File descriptor created! fh: {fh}");
                reply.opened(fh, FopenFlags::empty());
            } else {
                trace!("Entry not found!");
                reply.error(Errno::ENOENT);
            }
        }
    };
}
