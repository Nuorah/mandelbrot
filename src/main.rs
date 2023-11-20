use bevy::{
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::*,
    reflect::TypePath,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle},
    window::WindowResized,
};

const MOVE_SPEED: f32 = 200.0;
const MOUSE_MOVE_SPEED: f32 = 100.0;
const ZOOM_SPEED: f32 = 2.5;

#[derive(Component, Debug)]
struct Mandelbrot;

#[derive(Component, Debug)]
struct MouseOrigin {
    x: f32,
    y: f32,
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: (1920., 1080.).into(),
                    ..default()
                }),
                ..default()
            }),
            Material2dPlugin::<CustomMaterial>::default(),
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, (on_resize, move_camera))
        .run();
}

// Setup a simple 2d scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
) {
    // camera
    commands.spawn(Camera2dBundle::default());

    // quad
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
            transform: Transform::default().with_scale(Vec3::splat(1080.)),
            material: materials.add(CustomMaterial {
                zoom: 0.005,
                center: Vec2::new(0.0, 0.0),
                epsilon: 1000.0,
            }),
            ..default()
        },
        Mandelbrot,
        MouseOrigin { x: 0.0, y: 0.0 },
    ));
}

fn on_resize(
    mut q: Query<&mut Transform, With<Mandelbrot>>,
    mut resize_reader: EventReader<WindowResized>,
) {
    let mut mandelbrot = q.single_mut();
    for e in resize_reader.read() {
        mandelbrot.scale = Vec3::splat(e.height);
    }
}

fn move_camera(
    mut windows: Query<&mut Window>,
    keyboard_input: Res<Input<KeyCode>>,
    mut mouse_wheel: EventReader<MouseWheel>,
    mouse: Res<Input<MouseButton>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
    mut mouse_origin: Query<&mut MouseOrigin>,
    time: Res<Time>,
) {
    let mut window = windows.single_mut();
    let mut origin = mouse_origin.single_mut();

    for ev in mouse_wheel.read() {
        match ev.unit {
            MouseScrollUnit::Line => {
                for material in materials.iter_mut() {
                    material.1.zoom -= ZOOM_SPEED * material.1.zoom * time.delta_seconds() * ev.y;
                }
            }
            MouseScrollUnit::Pixel => {}
        }
    }

    if mouse.just_pressed(MouseButton::Left) {
        origin.x = window.cursor_position().map_or(0.0, |p| p.x);
        origin.y = window.cursor_position().map_or(0.0, |p| p.y);
    }

    if mouse.pressed(MouseButton::Left) {
        window.cursor.visible = false;
        for material in materials.iter_mut() {
            material.1.center.x -= MOUSE_MOVE_SPEED
                * (window.cursor_position().map_or(0.0, |p| p.x) - origin.x)
                * material.1.zoom
                * time.delta_seconds();
            material.1.center.y += MOUSE_MOVE_SPEED
                * (window.cursor_position().map_or(0.0, |p| p.y) - origin.y)
                * material.1.zoom
                * time.delta_seconds();
        }
        origin.x = window.cursor_position().map_or(0.0, |p| p.x);
        origin.y = window.cursor_position().map_or(0.0, |p| p.y);
    }

    if mouse.just_released(MouseButton::Left) {
        window.cursor.visible = true;
    }

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
            material.1.zoom;
        }
    }
    if keyboard_input.pressed(KeyCode::A) {
        for material in materials.iter_mut() {
            material.1.zoom -= ZOOM_SPEED * material.1.zoom * time.delta_seconds();
            //material.1.epsilon = material.1.zoom * (1000.0 / 0.005);
            material.1.zoom;
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
