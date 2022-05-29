#![feature(derive_default_enum)]
mod animation_tree;
mod character;
mod common;
mod components;

use crate::animation_tree::AnimationTreePlugin;
use crate::character::CharacterPlugin;
use crate::components::InputVector;
use benimator::AnimationPlugin;
use bevy::prelude::*;
use bevy::winit::WinitSettings;
use bevy_inspector_egui::{InspectableRegistry, WorldInspectorPlugin};
use bevy_inspector_egui_rapier::InspectableRapierPlugin;
use bevy_parallax::{LayerData, ParallaxCameraComponent, ParallaxPlugin, ParallaxResource};
use bevy_prototype_debug_lines::DebugLinesPlugin;
use bevy_rapier2d::prelude::*;

fn main() {
    App::new()
        .insert_resource(WinitSettings::game())
        .insert_resource(WindowDescriptor {
            title: "RPG".to_string(),
            width: 320. * common::SCALE,
            height: 180. * common::SCALE,
            resizable: true,
            ..default()
        })
        .insert_resource(ParallaxResource {
            layer_data: vec![
                LayerData {
                    speed: 1.0,
                    path: "World/GrassBackground.png".to_string(),
                    tile_size: Vec2::new(64., 64.),
                    position: Vec2::ZERO,
                    cols: 1,
                    rows: 1,
                    scale: 1.0,
                    z: 0.0,
                    ..Default::default()
                },
                LayerData {
                    speed: 1.0,
                    path: "World/GrassBackground.png".to_string(),
                    tile_size: Vec2::new(64., 64.),
                    position: Vec2::new(0., 64.),
                    cols: 1,
                    rows: 1,
                    scale: 1.0,
                    z: 0.0,
                    ..Default::default()
                },
                LayerData {
                    speed: 1.0,
                    path: "World/GrassBackground.png".to_string(),
                    tile_size: Vec2::new(64., 64.),
                    position: Vec2::new(0., -64.),
                    cols: 1,
                    rows: 1,
                    scale: 1.0,
                    z: 0.0,
                    ..Default::default()
                },
            ],
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(DebugLinesPlugin::default())
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(InspectableRapierPlugin)
        .add_plugin(WorldInspectorPlugin::default())
        .add_plugin(ParallaxPlugin)
        .add_plugin(AnimationPlugin::default())
        .add_plugin(AnimationTreePlugin)
        .add_plugin(GamePlugin)
        .add_plugin(CharacterPlugin)
        .run();
}

struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        let mut registry = app.world.get_resource_mut::<InspectableRegistry>().unwrap();

        registry.register::<InputVector>();

        app.add_startup_system(spawn_camera)
            .add_startup_system(set_gravity);
    }
}

fn spawn_camera(mut commands: Commands) {
    // Camera
    let mut orthographic_camera = OrthographicCameraBundle::new_2d();
    orthographic_camera.orthographic_projection.scale = 1. / common::SCALE;

    commands
        .spawn_bundle(orthographic_camera)
        .insert(ParallaxCameraComponent);
}

fn set_gravity(mut rapier_config: ResMut<RapierConfiguration>) {
    rapier_config.gravity = Vec2::ZERO;
}
