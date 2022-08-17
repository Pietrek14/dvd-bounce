use std::f32::consts::PI;

use bevy::prelude::*;

pub const WIDTH: f32 = 720.0;
pub const HEIGHT: f32 = 480.0;

pub const SPEED: f32 = 100.0;
pub const STARTING_ANGLE: f32 = PI / 4.0;

fn camera_setup(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}

#[derive(Component)]
struct LogoDirection(Vec3);

fn calculate_position_on_circle(radius: f32, angle: f32) -> Vec3 {
    Vec3::new(radius * angle.cos(), radius * angle.sin(), 0.0)
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("logo.png"),
        transform: Transform::from_scale(Vec3 { x: 0.5, y: 0.5, z: 1.0 }),
        ..Default::default()
    }).insert(LogoDirection(calculate_position_on_circle(SPEED, STARTING_ANGLE)));
}

fn move_logo(mut query: Query<(&mut Transform, &LogoDirection)>, time: Res<Time>) {
    let mut logo = query.iter_mut().next().expect("no logo :(");

    logo.0.translation += logo.1.0 * time.delta_seconds();
}

fn bounce(mut query: Query<(&mut Transform, &Handle<Image>, &mut LogoDirection)>, assets: Res<Assets<Image>>) {
    let mut logo = query.iter_mut().next().expect("no logo :(");

    let screen_position = Vec2::new(
        logo.0.translation.x + WIDTH / 2.0,
        logo.0.translation.y + HEIGHT / 2.0
    );

    if let Some(sprite) = assets.get(logo.1) {
        let sprite_size = Vec2::new(
            sprite.texture_descriptor.size.width as f32 * logo.0.scale.x,
            sprite.texture_descriptor.size.height as f32 * logo.0.scale.y
        );

        // TODO: Change color on collision
        // Also simplify this shit code

        if screen_position.x - sprite_size.x / 2.0 <= 0.0 {
            logo.2.0.x *= -1.0;
            logo.0.translation.x = (sprite_size.x - WIDTH) / 2.0;
        }
        
        if screen_position.x + sprite_size.x / 2.0 >= WIDTH {
            logo.2.0.x *= -1.0;
            logo.0.translation.x = (WIDTH - sprite_size.x) / 2.0;
        }
        
        if screen_position.y - sprite_size.y / 2.0 <= 0.0 {
            logo.2.0.y *= -1.0;
            logo.0.translation.y = (sprite_size.y - HEIGHT) / 2.0;
        }
        
        if screen_position.y + sprite_size.y / 2.0 >= HEIGHT {
            logo.2.0.y *= -1.0;
            logo.0.translation.y = (HEIGHT - sprite_size.y) / 2.0;
        }
    }
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "DVD Bounce".to_string(),
            width: WIDTH,
            height: HEIGHT,
            resizable: false,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(camera_setup)
        .add_startup_system(setup)
        .add_system(move_logo)
        .add_system(bounce)
        .run();
}
