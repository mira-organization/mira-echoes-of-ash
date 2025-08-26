#![coverage(off)]

use bevy_rapier3d::geometry::Group;

pub const GROUP_CAMERA_COLLIDER: Group = Group::GROUP_1;
pub const GROUP_ITEMS_COLLIDER: Group = Group::GROUP_2;
pub const GROUP_CHARACTER_COLLIDER: Group = Group::GROUP_2;