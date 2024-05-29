use bevy::prelude::*;
use crate::SpriteSheetRust;

pub(crate) struct RobotPlugin;

impl Plugin for RobotPlugin{
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_robot);
    }
}

fn spawn_robot(mut commands: Commands, sprite: Res<SpriteSheetRust>) {
    let mut robot = TextureAtlasSprite::new(13);
    robot.custom_size = Some(Vec2::splat(500.0));

    commands.spawn(SpriteSheetBundle {
        texture_atlas: sprite.0.clone(),
        sprite: robot,
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..Default::default()
    }).insert(Name::new("Robot"));
}