use bevy::prelude::*;

use crate::node::INode;

// Special case for root directory, its inode value would be 1 instead of EntityId
#[derive(Debug, Component)]
pub struct RootDirectoryMarker;

#[derive(Debug, Resource)]
pub struct RootDirectory(pub Entity);

#[derive(Debug, Component)]
#[require(INode::new_directory(), INodeStorage)]
pub struct Directory {}

#[derive(Debug, Component)]
#[relationship(relationship_target = INodeStorage)]
pub struct StoredIn {
    #[relationship]
    pub parent: Entity,
}

#[derive(Debug, Default, Component)]
#[relationship_target(relationship = StoredIn)]
pub struct INodeStorage(Vec<Entity>);
