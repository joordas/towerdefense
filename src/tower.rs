use bevy::prelude::*;
use bevy::utils::FloatOrd;

pub use crate::components::{Bullet, GameAssets, Health, Lifetime, Target, Tower, TowerType};
use crate::physics::PhysicsBundle;
use crate::*;

fn tower_shooting(
    mut commands: Commands,
    targets: Query<&GlobalTransform, With<Target>>,
    bullet_assets: Res<GameAssets>,
    mut towers: Query<(Entity, &mut Tower, &TowerType, &GlobalTransform)>,
    time: Res<Time>,
) {
    for (tower_ent, mut tower, tower_type, transform) in &mut towers {
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
                        let (model, bullet) = tower_type.get_bullet(direction, &bullet_assets);

                        commands
                            .spawn(model)
                            .insert(Lifetime {
                                timer: Timer::from_seconds(5.5, TimerMode::Once),
                            })
                            .insert(Name::new("Bullet"))
                            .insert(bullet)
                            .insert(PhysicsBundle::moving_entity(Vec3::new(0.2, 0.2, 0.2)));
                    });
                }
            }
        }
    }
}

// fn build_tower(
//     mut commands: Commands,
//     selection: Query<(Entity, &Selection, &Transform)>,
//     keyboard: Res<Input<KeyCode>>,

//     assets: Res<GameAssets>,
// ) {
//     if keyboard.just_pressed(KeyCode::Space) {
//         for (entity, selection, transform) in &selection {
//             if selection.selected() {
//                 commands.entity(entity).despawn_recursive();
//                 spawn_tower(&mut commands, &assets, transform.translation);
//             }
//         }
//     }
// }

fn tower_button_clicked(
  interaction: Query<(&Interaction, &TowerType), Changed<Interaction>>,
  mut commands: Commands,
  selection: Query<(Entity, &Selection, &Transform)>,
  assets: Res<GameAssets>,
) {
  for (interaction, tower_type) in &interaction {
      if matches!(interaction, Interaction::Clicked) {
          for (entity, selection, transform) in &selection {
              if selection.selected() {
                  //Remove the base model/hitbox
                  commands.entity(entity).despawn_recursive();

                  spawn_tower(&mut commands, &assets, transform.translation, *tower_type);
              }
          }
      }
  }
}
fn spawn_tower(commands: &mut Commands, tower_type: TowerType, assets: &GameAssets, position: Vec3) -> Entity {
  let (model, tower) = tower_type.get_tower(&assets);

    commands
        .spawn(SpatialBundle::from_transform(Transform::from_translation(
            position,
        )))
        .insert(Name::new(format!("{:?}_Tower", tower_type)))
        .insert(tower)
        .with_children(|commands| {
            commands.spawn(SceneBundle {
                scene: model,
                transform: Transform::from_xyz(0.0, -0.8, 0.0),
                ..Default::default()
            });
        })
        .id()
}

#[derive(Default)]
pub struct TowerPlugin;

impl Plugin for TowerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(tower_shooting);
    }
}
