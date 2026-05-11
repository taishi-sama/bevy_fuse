use bevy::ecs::entity::Entity;
use fuser::INodeNo;


use crate::fuser::{Fh, Inode};

pub fn entity_to_inode(entity: Entity, root_entity: Entity) -> Inode {
    if entity == root_entity {
        INodeNo::ROOT
    } else {
        fuser::INodeNo(entity.to_bits())
    }
}

pub fn inode_to_entity(ino: Inode, root_entity: Entity) -> Entity {
    if ino == INodeNo::ROOT {
        root_entity
    } else {
        Entity::from_bits(ino.into())
    }
}

pub fn entity_to_fh(entity: Entity) -> Fh {
    fuser::FileHandle(entity.to_bits())
}

pub fn fh_to_entity(ino: Fh) -> Entity {
    Entity::from_bits(ino.into())
}
