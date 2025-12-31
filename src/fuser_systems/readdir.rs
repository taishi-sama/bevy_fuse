use std::ffi::OsStr;

use bevy::prelude::*;
use libc::{ENOENT, ENOTDIR};

use crate::{
    fuser::FuserState,
    fuser_systems::helpers::{entity_to_inode, inode_to_entity},
    node::{
        INode,
        directory::{Directory, INodeStorage, RootDirectory},
    },
};

pub fn readdir_system(
    mut fuser_state: NonSendMut<FuserState>,
    root_entity: Res<RootDirectory>,
    nodes: Query<(
        &INode,
        Option<&Directory>,
        Option<&INodeStorage>,
        Option<&Name>,
    )>,
) {
    let root_entity = root_entity.0;
    if let Some(ref mut state) = fuser_state.0 {
        while let Ok((data, mut reply)) = state.readdir.try_recv() {
            trace!("Recieved 1 message!");
            let e = inode_to_entity(data.ino, root_entity);
            if let Ok((metadata, dir, content, _name)) = nodes.get(e) {
                trace!("Found entry, replying...! {metadata:?}");
                if let Some(_dir) = dir
                    && let Some(content) = content
                {
                    for (index, entry) in content.iter().skip(data.offset as usize).enumerate() {
                        let inode = entity_to_inode(entry, root_entity);
                        let (entry_md, _, _, entry_name) = nodes.get(entry).unwrap();
                        let buffer_full: bool = reply.add(
                            inode,
                            data.offset + index as i64 + 1,
                            entry_md.kind,
                            OsStr::new(
                                &entry_name.unwrap().as_str(), //Fix me, do something sane if file lacks name
                            ),
                        );

                        if buffer_full {
                            break;
                        }
                    }

                    reply.ok();
                } else {
                    reply.error(ENOTDIR);
                }
            } else {
                trace!("Entry not found!");
                reply.error(ENOENT);
            }
        }
    };
}
