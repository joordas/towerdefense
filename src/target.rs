use bevy::prelude::*;

use crate::components::GameState;
pub use crate::components::{Health, Target, TargetDeathEvent, Tower};

fn move_targets(mut targets: Query<(&Target, &mut Transform)>, time: Res<Time>) {
    for (target, mut transform) in &mut targets {
        transform.translation.x += target.speed * time.delta_seconds();
    }
}

fn target_death(
    mut commands: Commands,
    targets: Query<(Entity, &Health)>,
    mut death_event_writer: EventWriter<TargetDeathEvent>,
) {
    for (entity, health) in &targets {
        if health.value <= 0 {
            death_event_writer.send(TargetDeathEvent);
            commands.entity(entity).despawn_recursive();
        }
    }
}

#[derive(Default)]
pub struct TargetPlugin;

impl Plugin for TargetPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Tower>()
            .register_type::<Target>()
            .register_type::<Health>()
            .add_event::<TargetDeathEvent>()
            .add_system_set(
                SystemSet::on_update(GameState::InGame)
                    .with_system(move_targets)
                    .with_system(target_death),
            );
    }
}
