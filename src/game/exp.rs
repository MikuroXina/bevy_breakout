use bevy::prelude::*;

pub struct Paddle {
    pub(super) speed: f32,
}

pub struct Ball {
    pub(super) velocity: Vec3,
}

pub struct Scoreboard {
    pub(super) score: u64,
}

pub enum Collider {
    Solid,
    Scorable,
    Paddle,
}

pub fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // Cameras
    commands
        .spawn(Camera2dComponents::default())
        .spawn(UiCameraComponents::default());
    // Paddle
    commands
        .spawn(SpriteComponents {
            material: materials.add(Color::rgb(0.5, 0.5, 1.).into()),
            transform: Transform::from_translation(Vec3::new(0., -215., 0.)),
            sprite: Sprite::new(Vec2::new(120., 30.)),
            ..Default::default()
        })
        .with(Paddle { speed: 500. })
        .with(Collider::Paddle);
    // Ball
    commands
        .spawn(SpriteComponents {
            material: materials.add(Color::rgb(1., 0.5, 0.5).into()),
            transform: Transform::from_translation(Vec3::new(0., -50., 1.)),
            sprite: Sprite::new(Vec2::new(30., 30.)),
            ..Default::default()
        })
        .with(Ball {
            velocity: 400. * Vec3::new(0.5, -0.5, 0.).normalize(),
        });
    // Scoreboard
    commands.spawn(TextComponents {
        text: Text {
            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            value: "Score: ".into(),
            style: TextStyle {
                color: Color::rgb(0.5, 0.5, 1.),
                font_size: 40.,
            },
        },
        style: Style {
            position_type: PositionType::Absolute,
            position: Rect {
                top: Val::Px(5.0),
                left: Val::Px(5.0),
                ..Default::default()
            },
            ..Default::default()
        },
        ..Default::default()
    });
    setup_walls(&mut commands, &mut materials);
    setup_bricks(&mut commands, &mut materials);
}

fn setup_walls(commands: &mut Commands, materials: &mut ResMut<Assets<ColorMaterial>>) {
    struct WallAsset {
        translation: Vec3,
        size: Vec2,
    }
    let wall_material = materials.add(Color::rgb(0.8, 0.8, 0.8).into());
    let wall_thickness = 10.;
    let bounds = Vec2::new(900., 600.);
    let assets = vec![
        // Left
        WallAsset {
            translation: Vec3::new(-bounds.x() / 2., 0., 0.),
            size: Vec2::new(wall_thickness, bounds.y() + wall_thickness),
        },
        // Right
        WallAsset {
            translation: Vec3::new(bounds.x() / 2., 0., 0.),
            size: Vec2::new(wall_thickness, bounds.y() + wall_thickness),
        },
        // Bottom
        WallAsset {
            translation: Vec3::new(0., -bounds.y() / 2., 0.),
            size: Vec2::new(bounds.x() + wall_thickness, wall_thickness),
        },
        // Top
        WallAsset {
            translation: Vec3::new(0., bounds.y() / 2., 0.),
            size: Vec2::new(bounds.x() + wall_thickness, wall_thickness),
        },
    ];
    for WallAsset { translation, size } in assets {
        commands
            .spawn(SpriteComponents {
                material: wall_material.clone(),
                transform: Transform::from_translation(translation),
                sprite: Sprite::new(size),
                ..Default::default()
            })
            .with(Collider::Solid);
    }
}

fn setup_bricks(commands: &mut Commands, materials: &mut ResMut<Assets<ColorMaterial>>) {
    let (brick_rows, brick_columns, brick_spacing) = (4, 5, 20.);
    let brick_size = Vec2::new(150., 30.);
    let bricks_width = brick_columns as f32 * (brick_size.x() + brick_spacing) - brick_spacing;

    let bricks_offset = Vec3::new(-(bricks_width - brick_size.x()) / 2., 100., 0.);
    let brick_material = materials.add(Color::rgb(0.5, 0.5, 1.).into());
    for row in 0..brick_rows {
        let y_pos = row as f32 * (brick_size.y() + brick_spacing);
        for column in 0..brick_columns {
            let x_pos = column as f32 * (brick_size.x() + brick_spacing);
            let brick_pos = Vec3::new(x_pos, y_pos, 0.0) + bricks_offset;
            commands
                .spawn(SpriteComponents {
                    material: brick_material.clone(),
                    sprite: Sprite::new(brick_size),
                    transform: Transform::from_translation(brick_pos),
                    ..Default::default()
                })
                .with(Collider::Scorable);
        }
    }
}
