use avian2d::prelude::DistanceJoint;
use bevy::{color::palettes::css, prelude::*};

use crate::node::{INode, directory::INodeStorage, file::File};

pub fn links_plugin(app: &mut App) {
    app.add_systems(Update, (visualize_links, update_links));
}

pub fn visualize_links(
    dirs: Query<(&GlobalTransform, &INodeStorage)>,
    children: Query<(&GlobalTransform,)>,
    mut gizmos: Gizmos,
) {
    for (t, s) in dirs.iter() {
        for c in s.collection() {
            let Ok((tc,)) = children.get(*c) else {
                continue;
            };
            gizmos.line_2d(t.translation().xy(), tc.translation().xy(), css::DARK_RED);
        }
    }
}

pub fn update_links(
    dirs: Query<(&INode, &INodeStorage)>,
    children: Query<(Option<&INodeStorage>, Option<&File>, &Children)>,
    mut springs: Query<&mut DistanceJoint>,
) {
    for (_, d) in dirs.iter() {
        for n in d.iter() {
            if let Ok((internal_storage, _is_file, children)) = children.get(n) {
                for spring in children.iter() {
                    if let Ok(mut joint) = springs.get_mut(spring) {
                        joint.limits.min = 15.0
                            * (1.0
                                + d.iter().count() as f32 / 10.0
                                + internal_storage.map(|x| x.iter().count()).unwrap_or(0) as f32
                                    / 5.5);
                        joint.limits.max = 20.0
                            * (1.0
                                + d.iter().count() as f32 / 10.0
                                + internal_storage.map(|x| x.iter().count()).unwrap_or(0) as f32
                                    / 5.5);
                    }
                }
            }
        }
    }
}
