use bevy::ecs::entity::Entity;
use fuser::FUSE_ROOT_ID;

use crate::fuser::{Fh, Inode};

pub fn entity_to_inode(entity: Entity, root_entity: Entity) -> Inode {
    if entity == root_entity {
        FUSE_ROOT_ID
    } else {
        entity.to_bits()
    }
}

pub fn inode_to_entity(ino: Inode, root_entity: Entity) -> Entity {
    if ino == FUSE_ROOT_ID {
        root_entity
    } else {
        Entity::from_bits(ino)
    }
}

pub fn entity_to_fh(entity: Entity) -> Fh {
    entity.to_bits()
}

pub fn fh_to_entity(ino: Fh) -> Entity {
    Entity::from_bits(ino)
}
