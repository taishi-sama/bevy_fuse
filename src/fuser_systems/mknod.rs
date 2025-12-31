use avian2d::prelude::{AngularDamping, Collider, DistanceJoint, LinearDamping, RigidBody};
use bevy::prelude::*;
use libc::{EEXIST, ENOENT, ENOTDIR};

use crate::{
    fuser::FuserState,
    fuser_systems::{
        TTL,
        helpers::{entity_to_inode, inode_to_entity},
    },
    node::{
        INode,
        directory::{Directory, INodeStorage, RootDirectory, StoredIn},
        file::File,
    },
};

pub fn mknod_system(
    mut fuser_state: NonSendMut<FuserState>,
    root_entity: Res<RootDirectory>,
    parent: Query<(
        &INode,
        Option<&Directory>,
        Option<&INodeStorage>,
        &GlobalTransform,
    )>,
    nodes: Query<(&INode, &Name, Option<&File>)>,
    mut c: Commands,
) {
    let root_entity = root_entity.0;
    if let Some(ref mut state) = fuser_state.0 {
        while let Ok((data, reply)) = state.mknod.try_recv() {
            trace!("Recieved 1 message!");
            let parent_entity = inode_to_entity(data.parent, root_entity);
            if let Ok((metadata, dir, content, gt)) = parent.get(parent_entity) {
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
                    if found.is_some() {
                        reply.error(EEXIST);
                        continue;
                    }
                    let file = File::default();
                    let inode = INode::new_file();
                    let new_entity = c
                        .spawn((
                            Sprite::from_color(Color::linear_rgb(0.1, 0.5, 0.5), vec2(15.0, 15.0)),
                            Transform::from_translation(
                                (gt.translation().xy() + vec2(10.0, 0.0)).extend(0.0),
                            ),
                            file.clone(),
                            inode.clone(),
                            Name::new(data.name.to_string_lossy().to_string()),
                            RigidBody::Dynamic,
                            Collider::rectangle(10.0, 10.0),
                            StoredIn {
                                parent: parent_entity,
                            },
                            LinearDamping(0.8),
                            AngularDamping(0.8),
                        ))
                        .id();
                    let _joint = c.spawn((
                        DistanceJoint::new(parent_entity, new_entity).with_limits(5.0, 55.0),
                        ChildOf(new_entity),
                    ));
                    let attr = inode
                        .get_file_attrb_obj(entity_to_inode(new_entity, root_entity), Some(&file));
                    reply.entry(&TTL, &attr, 0);
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
