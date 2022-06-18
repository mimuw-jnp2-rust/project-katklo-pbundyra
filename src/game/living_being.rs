use bevy::prelude::*;

pub struct LivingBeingHitEvent {
    pub entity: Entity,
}

pub struct LivingBeingDeathEvent {
    pub entity: Entity,
}

pub fn on_living_being_hit(
    mut living_being_hit_events: EventReader<LivingBeingHitEvent>,
    mut send_living_being_death: EventWriter<LivingBeingDeathEvent>,
) {
    for event in living_being_hit_events.iter() {
        send_living_being_death.send(LivingBeingDeathEvent {
            entity: event.entity,
        })
    }
}

pub fn on_living_being_dead(
    mut living_being_death_events: EventReader<LivingBeingDeathEvent>,
    mut commands: Commands,
) {
    for event in living_being_death_events.iter() {
        /*
         There is a simple despawn instead of despawn_recursively() as simple despawn() handles
         double despawns for a single enitity and despawn_recursively() panics.
         */
        commands.entity(event.entity).despawn();
    }
}
