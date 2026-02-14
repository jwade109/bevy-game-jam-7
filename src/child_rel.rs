use bevy::prelude::*;

#[derive(Component, Debug)]
#[relationship(relationship_target = DuckChildren)]
pub struct DuckParent(pub Entity);

#[derive(Component, Debug)]
#[relationship_target(relationship = DuckParent)]
pub struct DuckChildren(Vec<Entity>);

#[derive(Component, Debug)]
#[relationship(relationship_target = FollowedBy)]
pub struct Following(pub Entity);

#[derive(Component, Debug)]
#[relationship_target(relationship = Following)]
pub struct FollowedBy(Vec<Entity>);
