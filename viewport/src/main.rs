use bevy::{
    prelude::*,
    window::{PresentMode, PrimaryWindow},
};

fn main() {
    // When building for WASM, print panics to the browser console
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    let mut app = App::new();

    app.insert_resource(ClearColor(Color::rgb_u8(60, 60, 60)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                present_mode: PresentMode::Fifo,
                #[cfg(target_arch = "wasm32")]
                canvas: Some("#viewport".to_string()),
                #[cfg(target_arch = "wasm32")]
                fit_canvas_to_parent: true,
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup_system)
        .add_systems(Update, spawn_on_click_system);

    app.run();
}

fn setup_system(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    const SPACING: f32 = 20.0;
    const LENGTH: f32 = 100000.0;
    const THICKNESS: f32 = 2.0;

    // X axis
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb_u8(141, 71, 79),
            custom_size: Some(Vec2::new(LENGTH, THICKNESS)),
            ..Default::default()
        },
        ..Default::default()
    });

    // Y axis
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb_u8(112, 139, 62),
            custom_size: Some(Vec2::new(THICKNESS, LENGTH)),
            ..Default::default()
        },
        ..Default::default()
    });

    for i in 1..=100 {
        commands.spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb_u8(72, 72, 72),
                custom_size: Some(Vec2::new(LENGTH, THICKNESS)),
                ..Default::default()
            },
            transform: Transform::from_xyz(0.0, i as f32 * SPACING, 0.0),
            ..Default::default()
        });
    }

    for i in -100..=-1 {
        commands.spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb_u8(72, 72, 72),
                custom_size: Some(Vec2::new(LENGTH, THICKNESS)),
                ..Default::default()
            },
            transform: Transform::from_xyz(0.0, i as f32 * SPACING, 0.0),
            ..Default::default()
        });
    }

    for j in 1..=100 {
        commands.spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb_u8(72, 72, 72),
                custom_size: Some(Vec2::new(THICKNESS, LENGTH)),
                ..Default::default()
            },
            transform: Transform::from_xyz(j as f32 * SPACING, 0.0, 0.0),
            ..Default::default()
        });
    }

    for j in -100..=-1 {
        commands.spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb_u8(72, 72, 72),
                custom_size: Some(Vec2::new(THICKNESS, LENGTH)),
                ..Default::default()
            },
            transform: Transform::from_xyz(j as f32 * SPACING, 0.0, 0.0),
            ..Default::default()
        });
    }
}

fn spawn_on_click_system(
    mut commands: Commands,
    windows: Query<&Window, With<PrimaryWindow>>,
    mouse_input: Res<Input<MouseButton>>,
) {
    if mouse_input.just_pressed(MouseButton::Left) {
        let Ok(window) = windows.get_single() else { return; };
        let Some(pos) = window.cursor_position() else { return; };
        commands.spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb_u8(141, 71, 79),
                custom_size: Some(Vec2::new(5.0, 5.0)),
                ..Default::default()
            },
            transform: Transform::from_translation(
                (pos - Vec2::new(window.width() / 2.0, window.height() / 2.0)).extend(0.0),
            ),
            ..Default::default()
        });
    }
}
