use std::u32;

use bevy::{ecs::entity::Entities, prelude::*};

use crate::{fuser::FuserState, node::INode};

pub fn statfs_system(
    mut fuser_state: NonSendMut<FuserState>,
    files: Query<&INode>,
    entities: &Entities,
) {
    if let Some(ref mut state) = fuser_state.0 {
        if let Ok((_data, reply)) = state.statfs.try_recv() {
            let count = files.count() as u64;
            let ffree = u32::MAX as u64 - 1 - entities.total_count() as u64;
            reply.statfs(0, 10000000, 10000000, count, ffree, 512, 1024, 0);
        }
    };
}
