use std::collections::VecDeque;

use bevy::prelude::*;

#[derive(Component)]
pub struct PathAnimator(pub VecDeque<Vec3>);
