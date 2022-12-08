use bevy::{core_pipeline::clear_color::ClearColorConfig, prelude::*, render::view::RenderLayers};
use bevy::utils::FloatOrd;
use bevy_inspector_egui::WorldInspectorPlugin;
use simula_action::ActionPlugin;
use simula_video::rt;
use bevy_egui::EguiPlugin;
use simula_camera::{flycam::*, orbitcam::*};

pub const HEIGHT: f32 = 720.0;
pub const WIDTH: f32 = 1280.0;

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Tower {
    shooting_timer: Timer,
    bullet_offset: Vec3,
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Lifetime {
    timer: Timer,
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Target {
    speed: f32,
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Health {
    value: i32,
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Bullet {
    direction: Vec3,
    speed: f32,
}

fn main() {
    let mut app = App::new();

    app.insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.2)))
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
        .add_plugin(EguiPlugin)
        .add_plugin(ActionPlugin)
        .add_plugin(OrbitCameraPlugin)
        .add_plugin(FlyCameraPlugin)
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_scene)
        .add_system(tower_shooting)
        .add_system(move_targets)
        .add_system(bullet_despawn)
        .add_system(move_bullets)
        .add_system(target_death)
        .add_system(bullet_collison)
        .add_startup_system(setup)
        .add_plugin(WorldInspectorPlugin::new())
        .run();
}

fn spawn_camera(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
  let rt_image = images.add(rt::common_render_target_image(UVec2 { x: 256, y: 256 }));

  commands
      .spawn(Camera3dBundle {
          transform: Transform::from_xyz(0.0, 2.0, -10.0)
              .looking_at(Vec3::new(0.0, 1.0, 0.0), Vec3::Y),
          ..default()
      })
      .insert(RenderLayers::all())
      .with_children(|parent| {
          let mut _child = parent.spawn(Camera3dBundle {
              camera_3d: Camera3d {
                  clear_color: ClearColorConfig::Custom(Color::BLACK),
                  ..default()
              },
              camera: Camera {
                  priority: -1,
                  target: bevy::render::camera::RenderTarget::Image(rt_image.clone()),
                  ..default()
              },
              ..default()
          });
      })
      .insert(FlyCamera::default());
}

fn spawn_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
            material: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
            // transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..Default::default()
        })
        .insert(Name::new("Floor"));

    // middle tower
    let middle_tower_transform = Transform::from_xyz(0.0, 0.5, 0.0);
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.2, 0.8, 0.2).into()),
            transform: middle_tower_transform,
            ..Default::default()
        })
        .insert(Tower {
            shooting_timer: Timer::from_seconds(1.0, TimerMode::Repeating),
            bullet_offset: Vec3::new(0.0, 0.0, 0.0),
        })
        .insert(Name::new("Tower"))
        .insert(Health { value: 3 });
    // .insert(Target {
    //   speed: 0.3,
    // });

    // target 1
    let target_1_transform = Transform::from_xyz(-4.0, 0.2, 1.5);

    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 0.4 })),
            material: materials.add(Color::rgb(0.9, 0.2, 0.2).into()),
            transform: target_1_transform,
            ..Default::default()
        })
        // .insert(Tower {
        //   shooting_timer: Timer::from_seconds(1.0, TimerMode::Repeating),
        // })
        .insert(Name::new("Target"))
        .insert(Health { value: 3 })
        .insert(Target { speed: 0.3 });

    // target 2
    let target_2_transform = Transform::from_xyz(-5.0, 0.2, 1.5);

    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 0.4 })),
            material: materials.add(Color::rgb(0.9, 0.2, 0.2).into()),
            transform: target_2_transform,
            ..Default::default()
        })
        // .insert(Tower {
        //   shooting_timer: Timer::from_seconds(1.0, TimerMode::Repeating),
        // })
        .insert(Name::new("Target"))
        .insert(Health { value: 3 })
        .insert(Target { speed: 0.3 });

    // spawn light
    commands
        .spawn(PointLightBundle {
            transform: Transform::from_xyz(4.0, 8.0, 4.0),
            ..Default::default()
        })
        .insert(Name::new("Light"));
}

fn move_targets(mut targets: Query<(&Target, &mut Transform)>, time: Res<Time>) {
    for (target, mut transform) in &mut targets {
        transform.translation.x += target.speed * time.delta_seconds();
    }
}

fn tower_shooting(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    targets: Query<&GlobalTransform, With<Target>>,
    mut towers: Query<(Entity, &mut Tower, &GlobalTransform)>,
    time: Res<Time>,
) {
    for (tower_ent, mut tower, transform) in &mut towers {
        tower.shooting_timer.tick(time.delta());
        {
            if tower.shooting_timer.just_finished() {
                let bullet_spawn: Vec3 = transform.translation() + tower.bullet_offset;

                let direction: Option<Vec3> = targets
                    .iter()
                    .min_by_key(|target_transform| {
                        FloatOrd(Vec3::distance(target_transform.translation(), bullet_spawn))
                    })
                    .map(|closest_target| closest_target.translation() - bullet_spawn);

                if let Some(direction) = direction {
                    commands.entity(tower_ent).with_children(|commands| {

                        let spawn_transform = Transform::from_translation(tower.bullet_offset);
                        commands
                            .spawn(PbrBundle {
                                mesh: meshes.add(Mesh::from(shape::UVSphere {
                                    radius: 0.1,
                                    ..Default::default()
                                })),
                                material: materials.add(Color::rgb(0.2, 0.2, 0.8).into()),
                                transform: spawn_transform,
                                ..Default::default()
                            })
                            .insert(Lifetime {
                                timer: Timer::from_seconds(5.5, TimerMode::Once),
                            })
                            .insert(Name::new("Bullet"))
                            .insert(Bullet {
                                direction: direction,
                                speed: 5.5,
                            });
                    });
                }
            }
        }
    }
}

fn move_bullets(mut bullets: Query<(&Bullet, &mut Transform)>, time: Res<Time>) {
    for (bullet, mut transform) in &mut bullets {
        transform.translation += bullet.direction.normalize() * bullet.speed * time.delta_seconds();
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

fn bullet_collison(
    mut commands: Commands,
    bullets: Query<(Entity, &GlobalTransform), With<Bullet>>,
    mut targets: Query<(&mut Health, &Transform), With<Target>>,
) {
  for (bullet, bullet_transform) in &bullets {
    for (mut health, target_transform) in &mut targets {
      if Vec3::distance(bullet_transform.translation(), target_transform.translation) < 0.2 {
        health.value -= 1;
        commands.entity(bullet).despawn_recursive();
        break;
      }
    }
  }
}

fn target_death(mut commands: Commands, targets: Query<(Entity, &Health)>) {
    for (entity, health) in &targets {
        if health.value <= 0 {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn setup(mut commands: Commands) {}
