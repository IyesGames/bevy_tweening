use bevy::prelude::*;
use bevy_inspector_egui::{Inspectable, InspectorPlugin};

use bevy_tweening::{lens::*, *};

fn main() {
    App::default()
        .insert_resource(WindowDescriptor {
            title: "TransformRotationLens".to_string(),
            width: 1400.,
            height: 600.,
            present_mode: bevy::window::PresentMode::Fifo, // vsync
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(TweeningPlugin)
        .add_plugin(InspectorPlugin::<Options>::new())
        .add_startup_system(setup)
        .add_system(update_animation_speed)
        .run();
}

#[derive(Copy, Clone, PartialEq, Inspectable)]
struct Options {
    #[inspectable(min = 0.01, max = 100.)]
    speed: f32,
}

impl Default for Options {
    fn default() -> Self {
        Self { speed: 1. }
    }
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());

    let size = 80.;

    let spacing = 1.6;
    let screen_x = 570.;
    let screen_y = 150.;
    let mut x = -screen_x;
    let mut y = screen_y;

    for ease_function in &[
        EaseFunction::QuadraticIn,
        EaseFunction::QuadraticOut,
        EaseFunction::QuadraticInOut,
        EaseFunction::CubicIn,
        EaseFunction::CubicOut,
        EaseFunction::CubicInOut,
        EaseFunction::QuarticIn,
        EaseFunction::QuarticOut,
        EaseFunction::QuarticInOut,
        EaseFunction::QuinticIn,
        EaseFunction::QuinticOut,
        EaseFunction::QuinticInOut,
        EaseFunction::SineIn,
        EaseFunction::SineOut,
        EaseFunction::SineInOut,
        EaseFunction::CircularIn,
        EaseFunction::CircularOut,
        EaseFunction::CircularInOut,
        EaseFunction::ExponentialIn,
        EaseFunction::ExponentialOut,
        EaseFunction::ExponentialInOut,
        EaseFunction::ElasticIn,
        EaseFunction::ElasticOut,
        EaseFunction::ElasticInOut,
        EaseFunction::BackIn,
        EaseFunction::BackOut,
        EaseFunction::BackInOut,
        EaseFunction::BounceIn,
        EaseFunction::BounceOut,
        EaseFunction::BounceInOut,
    ] {
        let tween = Tween::new(
            *ease_function,
            TweeningType::PingPong,
            std::time::Duration::from_secs(1),
            TransformRotationLens {
                start: Quat::IDENTITY,
                end: Quat::from_axis_angle(Vec3::Z, std::f32::consts::PI / 2.),
            },
        );

        commands
            .spawn_bundle(SpatialBundle {
                transform: Transform::from_translation(Vec3::new(x, y, 0.)),
                ..default()
            })
            .with_children(|parent| {
                parent
                    .spawn_bundle(SpriteBundle {
                        sprite: Sprite {
                            color: Color::RED,
                            custom_size: Some(Vec2::new(size, size * 0.5)),
                            ..default()
                        },
                        ..default()
                    })
                    .insert(Animator::new(tween));
            });

        y -= size * spacing;
        if y < -screen_y {
            x += size * spacing;
            y = screen_y;
        }
    }
}

fn update_animation_speed(options: Res<Options>, mut animators: Query<&mut Animator<Transform>>) {
    if !options.is_changed() {
        return;
    }

    for mut animator in animators.iter_mut() {
        animator.set_speed(options.speed);
    }
}
