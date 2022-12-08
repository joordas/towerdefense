use bevy::prelude::*;

use crate::components::{Bullet, Lifetime};


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


#[derive(Default)]
pub struct BulletPlugin;


impl Plugin for BulletPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_system(move_bullets)
      .add_system(bullet_despawn);
  }
}
