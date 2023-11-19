use bevy::{
    prelude::*,
    reflect::TypePath,
    render::{
        render_asset::RenderAsset,
        render_resource::{AsBindGroup, ShaderRef},
    },
    sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle},
};

const MOVE_SPEED: f32 = 200.0;
const ZOOM_SPEED: f32 = 2.5;

#[derive(Component, Debug)]
struct Mandelbrot;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: (800., 600.).into(),
                    ..default()
                }),
                ..default()
            }),
            Material2dPlugin::<CustomMaterial>::default(),
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, move_camera)
        .run();
}

// Setup a simple 2d scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // camera
    commands.spawn(Camera2dBundle::default());

    // quad
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
            transform: Transform::default().with_scale(Vec3::splat(600.)),
            material: materials.add(CustomMaterial {
                zoom: 0.005,
                center: Vec2::new(0.0, 0.5),
                epsilon: 1000.0,
            }),
            ..default()
        },
        Mandelbrot,
    ));
}

fn move_camera(
    keyboard_input: Res<Input<KeyCode>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
    time: Res<Time>,
) {
    if keyboard_input.pressed(KeyCode::Left) {
        for material in materials.iter_mut() {
            material.1.center.x -= MOVE_SPEED * material.1.zoom * time.delta_seconds();
        }
    }
    if keyboard_input.pressed(KeyCode::Right) {
        for material in materials.iter_mut() {
            material.1.center.x += MOVE_SPEED * material.1.zoom * time.delta_seconds();
        }
    }
    if keyboard_input.pressed(KeyCode::Up) {
        for material in materials.iter_mut() {
            material.1.center.y += MOVE_SPEED * material.1.zoom * time.delta_seconds();
        }
    }
    if keyboard_input.pressed(KeyCode::Down) {
        for material in materials.iter_mut() {
            material.1.center.y -= MOVE_SPEED * material.1.zoom * time.delta_seconds();
        }
    }
    if keyboard_input.pressed(KeyCode::E) {
        for material in materials.iter_mut() {
            material.1.zoom += ZOOM_SPEED * material.1.zoom * time.delta_seconds();
            //material.1.epsilon = material.1.zoom * (1000.0 / 0.005);
        }
    }
    if keyboard_input.pressed(KeyCode::A) {
        for material in materials.iter_mut() {
            material.1.zoom -= ZOOM_SPEED * material.1.zoom * time.delta_seconds();
            //material.1.epsilon = material.1.zoom * (1000.0 / 0.005);
        }
    }
}

// This is the struct that will be passed to your shader
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone, Component)]
pub struct CustomMaterial {
    #[uniform(0)]
    zoom: f32,
    #[uniform(1)]
    center: Vec2,
    #[uniform(2)]
    epsilon: f32,
}

/// The Material2d trait is very configurable, but comes with sensible defaults for all methods.
/// You only need to implement functions for features that need non-default behavior. See the Material2d api docs for details!
impl Material2d for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/custom_material_2d.wgsl".into()
    }
}
