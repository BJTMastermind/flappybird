/*
MIT License

Copyright (c) 2025 BJTMastermind

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/

use bevy::{prelude::*, window::PrimaryWindow};

use crate::FlappybirdState;

use super::WorldSpeed;

#[derive(Component)]
pub struct Sky;

#[derive(Resource)]
struct SkyOffset(pub f32);

fn move_sky(
    time: Res<Time>,
    speed: Res<WorldSpeed>,
    mut sky_query: Query<&mut Transform, With<Sky>>,
    mut offset: ResMut<SkyOffset>,
) {
    let delta_x = (speed.0 / 3.) * time.delta_seconds();
    for mut transform in sky_query.iter_mut() {
        transform.translation.x -= delta_x;
    }

    offset.0 += delta_x;
}

fn despawn_and_spawn_sky(
    mut commands: Commands,
    mut query: Query<(Entity, &Transform), With<Sky>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    mut offset: ResMut<SkyOffset>,
) {
    let window = window_query.get_single().unwrap();
    let texture_handle = asset_server
        .get_handle("embedded://sprites/world/day-sky.png")
        .unwrap();
    let texture_width = 144.0; // Adjust based on your texture width
    let texture_scale = Vec3::splat(3.); // Adjust based on your texture scale

    let effective_width = texture_width * texture_scale.x;

    let mut sky_entities: Vec<(Entity, &Transform)> = query.iter_mut().collect();
    sky_entities.sort_by(|a, b| a.1.translation.x.partial_cmp(&b.1.translation.x).unwrap());

    // Despawn entities that have moved out of the left boundary
    for (entity, transform) in sky_entities.iter() {
        if transform.translation.x <= -effective_width / 2. {
            println!("Despawning entity");
            commands.entity(*entity).despawn();
            // Spawn new ground entities on the right if needed
            let rightmost_x = sky_entities
                .last()
                .map_or(-window.width() / 2., |(_, transform)| {
                    transform.translation.x
                });
            let new_x = rightmost_x + effective_width - offset.0;
            commands.spawn((
                SpriteBundle {
                    texture: texture_handle.clone(),
                    transform: Transform {
                        translation: Vec3::new(new_x, window.height() / 2., 0.),
                        scale: texture_scale,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                Sky,
            ));
        }
    }

    offset.0 = 0.;
}

pub struct SkyPlugin;
impl Plugin for SkyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SkyOffset(0.)).add_systems(
            Update,
            (move_sky, despawn_and_spawn_sky).run_if(not(in_state(FlappybirdState::GameOver))),
        );
    }
}
