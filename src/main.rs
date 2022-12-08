use bevy::prelude::*;
use simula_camera::{flycam::*, orbitcam::*};
use bevy_inspector_egui::WorldInspectorPlugin;
use std::f32::consts::PI;

pub const HEIGHT: f32 = 720.0;
pub const WIDTH: f32 = 1280.0;


#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Tower {
  shooting_timer: Timer,
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Lifetime {
  timer: Timer,
}


fn main() {
    let mut app = App::new();

    app
    .insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.2)))
    .add_plugins(DefaultPlugins.set(WindowPlugin {
      window: WindowDescriptor {
        width: WIDTH,
        height: HEIGHT,
        resizable: false,
        ..default()
      },
      ..default()
    }))
    .register_type::<Tower>()
    .add_startup_system(spawn_camera)
    .add_startup_system(spawn_scene)
    .add_system(tower_shooting)
    .add_system(bullet_despawn)
    // .add_plugin<(OrbitCameraPlugin)
    // .add_plugin(FlyCameraPlugin)>
    .add_startup_system(setup)
    .add_plugin(WorldInspectorPlugin::new())
    .run();
}


fn spawn_camera(mut commands: Commands) {
  commands.spawn(Camera3dBundle {
    transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ..Default::default()
  });
}

fn spawn_scene(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
  commands.spawn(PbrBundle {
    mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
    material: materials.add(Color::rgb(0.8, 0.2, 0.2).into()),
    // transform: Transform::from_xyz(0.0, 0.0, 0.0),
    ..Default::default()
  }).insert(Name::new("Floor"));

  commands.spawn(
    PbrBundle {
      mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
      material: materials.add(Color::rgb(0.2, 0.8, 0.2).into()),
      transform: Transform::from_xyz(0.0, 0.5, 0.0),
      ..Default::default()
    }
  )
  .insert(Tower {
    shooting_timer: Timer::from_seconds(1.0, TimerMode::Repeating),
  })
  .insert(Name::new("Tower"));

  // spawn light
  commands.spawn(PointLightBundle {
    transform: Transform::from_xyz(4.0, 8.0, 4.0),
    ..Default::default()
  }).insert(Name::new("Light"));

}


fn tower_shooting(
  mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
  mut towers: Query<(&mut Tower)>,
  time: Res<Time>,
) {
  for mut tower in &mut towers {
    tower.shooting_timer.tick(time.delta()); {
      if tower.shooting_timer.just_finished() {

        let spawn_transform = Transform::from_xyz(0.0, 0.7, 0.6).with_rotation(Quat::from_rotation_y(-PI / 2.0));

        commands.spawn(PbrBundle {
          mesh: meshes.add(Mesh::from(shape::UVSphere { radius: 0.1, ..Default::default() })),
          material: materials.add(Color::rgb(0.2, 0.2, 0.8).into()),
          transform: spawn_transform,
          ..Default::default()
        })
        .insert(Lifetime {
          timer: Timer::from_seconds(0.5, TimerMode::Once),
        })
        .insert(Name::new("Bullet"));
      }
    }
  }
}

fn bullet_despawn(
  mut commands: Commands,
  mut bullets: Query<(Entity, &mut Lifetime)>,
  time: Res<Time>,
) {
  for (entity, mut lifetime) in &mut bullets {
    lifetime.timer.tick(time.delta());
    if lifetime.timer.just_finished() {
      commands.entity(entity).despawn_recursive();
    }
  }
}


// 3d camera with orbit controls
fn setup(mut commands: Commands,

) {


}
