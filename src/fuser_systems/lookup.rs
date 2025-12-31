use bevy::prelude::*;
use libc::{ENOENT, ENOTDIR};

use crate::{
    fuser::FuserState,
    fuser_systems::{
        TTL,
        helpers::{entity_to_inode, inode_to_entity},
    },
    node::{
        INode,
        directory::{Directory, INodeStorage, RootDirectory},
        file::File,
    },
};

pub fn lookup_system(
    mut fuser_state: NonSendMut<FuserState>,
    root_entity: Res<RootDirectory>,
    parent: Query<(&INode, Option<&Directory>, Option<&INodeStorage>)>,
    nodes: Query<(&INode, &Name, Option<&File>)>,
) {
    let root_entity = root_entity.0;
    if let Some(ref mut state) = fuser_state.0 {
        while let Ok((data, reply)) = state.lookup.try_recv() {
            trace!("Recieved 1 message!");
            let e = inode_to_entity(data.parent, root_entity);
            if let Ok((metadata, dir, content)) = parent.get(e) {
                trace!("Found entry, replying...! {metadata:?}");
                if let Some(_dir) = dir
                    && let Some(content) = content
                {
                    let mut found = None;
                    for n in content.iter() {
                        let (md, name, file) = nodes.get(n).unwrap();
                        if let Some(s) = data.name.to_str()
                            && s == name.as_str()
                        {
                            found = Some((md, n, file));
                            break;
                        }
                    }
                    if let Some((md, n, file)) = found {
                        trace!("Found entry, replying...! {md:?}");
                        let ino = entity_to_inode(n, root_entity);
                        reply.entry(&TTL, &md.get_file_attrb_obj(ino, file), 0);
                    } else {
                        trace!("Entry not found!");
                        reply.error(ENOENT);
                    }
                } else {
                    reply.error(ENOTDIR);
                }
                // reply.attr(&TTL, &metadata.get_file_attrb(data));
            } else {
                trace!("Entry not found!");
                reply.error(ENOENT);
            }
        }
    };
}
