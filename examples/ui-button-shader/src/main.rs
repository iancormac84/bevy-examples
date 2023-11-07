//! Demonstrates the use of [`UiMaterials`](UiMaterial) and how to change material values

use bevy::prelude::*;
use bevy::reflect::TypePath;
use bevy::render::render_resource::*;
use bevy_shader_utils::ShaderUtilsPlugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::RgbaLinear {
            red: 0.25,
            green: 0.24,
            blue: 0.28,
            alpha: 1.,
        }))
        .add_plugins((DefaultPlugins, ShaderUtilsPlugin))
        .add_plugins(
            UiMaterialPlugin::<CustomUiMaterial>::default(),
        )
        .add_plugins(
            UiMaterialPlugin::<HeartUiMaterial>::default(),
        )
        .add_systems(Startup, setup)
        .add_systems(Update, (update, update_time))
        .run();
}

fn update(
    time: Res<Time>,
    mut ui_materials: ResMut<Assets<CustomUiMaterial>>,
) {
    for (_, material) in ui_materials.iter_mut() {
        // rainbow color effect
        let new_color = Color::hsl(
            (time.elapsed_seconds() * 60.0) % 360.0,
            1.,
            0.5,
        );
        material.color = new_color.into();
    }
}

fn update_time(
    time: Res<Time>,
    mut ui_materials: ResMut<Assets<HeartUiMaterial>>,
) {
    for (_, material) in ui_materials.iter_mut() {
        material.time = time.elapsed_seconds();
    }
}

fn setup(
    mut commands: Commands,
    mut ui_materials: ResMut<Assets<CustomUiMaterial>>,
    mut heart_ui_materials: ResMut<Assets<HeartUiMaterial>>,
) {
    // Camera so we can see UI
    commands.spawn(Camera2dBundle::default());

    commands
        .spawn(NodeBundle {
            // background_color: Color::RED.into(),
            style: Style {
                display: Display::Grid,
                grid_template_columns: vec![
                    RepeatedGridTrack::flex(10, 1.),
                ],
                grid_template_rows: vec![
                    RepeatedGridTrack::px(50, 25.),
                ],
                column_gap: Val::Px(2.),
                width: Val::Px(300.),
                height: Val::Px(10.0),
                padding: UiRect::all(Val::Px(20.)),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            for i in 0..18 {
                parent
                    .spawn(NodeBundle {
                        // background_color: Color::RED.into(),
                        style: Style {
                            aspect_ratio: Some(1.),
                            width: Val::Percent(100.),
                            height: Val::Auto,
                            align_items: AlignItems::Center,
                            justify_content:
                                JustifyContent::Center,
                            ..default()
                        },
                        ..default()
                    })
                    .with_children(|parent| {
                        parent.spawn(MaterialNodeBundle {
                            style: Style {
                                aspect_ratio: Some(1.),
                                width: Val::Percent(100.),
                                height: Val::Percent(100.),
                                ..default()
                            },
                            material: heart_ui_materials
                                .add(HeartUiMaterial {
                                    color: Color::WHITE
                                        .into(),
                                    time: 0.,
                                    fill_level: -(((i
                                        as f32
                                        % 18.)
                                        / 18.)
                                        * 2.0
                                        - 1.0),
                                    offset: i as f32 * 10.,
                                }),
                            ..default()
                        });
                    });
            }
        });
    commands
        .spawn(NodeBundle {
            // background_color: Color::RED.into(),
            style: Style {
                position_type: PositionType::Absolute,
                right: Val::Px(0.),
                bottom: Val::Px(0.),
                aspect_ratio: Some(1.),
                margin: UiRect::all(Val::Px(50.)),
                width: Val::Percent(50.),
                height: Val::Auto,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(MaterialNodeBundle {
                style: Style {
                    aspect_ratio: Some(1.),
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    ..default()
                },
                material: heart_ui_materials.add(
                    HeartUiMaterial {
                        color: Color::WHITE.into(),
                        time: 0.,
                        fill_level: 0.,
                        offset: 0.,
                    },
                ),
                ..default()
            });
        });
}

#[derive(AsBindGroup, Asset, TypePath, Debug, Clone)]
struct CustomUiMaterial {
    #[uniform(0)]
    color: Vec4,
}

impl UiMaterial for CustomUiMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/ui_button.wgsl".into()
    }
}

#[derive(AsBindGroup, Asset, TypePath, Debug, Clone)]
struct HeartUiMaterial {
    #[uniform(0)]
    color: Vec4,
    #[uniform(0)]
    time: f32,
    #[uniform(0)]
    fill_level: f32,
    #[uniform(0)]
    offset: f32,
}

impl UiMaterial for HeartUiMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/heart.wgsl".into()
    }
}
