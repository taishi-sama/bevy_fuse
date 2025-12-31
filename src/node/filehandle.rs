use bevy::prelude::*;

#[derive(Debug, Component)]
#[relationship(relationship_target = FileHandleTarget)]
pub struct FileHandle {
    #[relationship]
    pub parent: Entity,
    pub read: bool,
    pub write: bool,
}

#[derive(Component)]
#[relationship_target(relationship = FileHandle, linked_spawn)]
pub struct FileHandleTarget(Vec<Entity>);
