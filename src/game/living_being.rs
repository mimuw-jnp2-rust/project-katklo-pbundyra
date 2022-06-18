use bevy::prelude::*;
use crate::game::{AudioHitEvent, LastDespawnedEntity};

pub struct LivingBeingHitEvent {
    pub entity: Entity,
}

pub struct LivingBeingDeathEvent {
    pub entity: Entity,
}

pub fn on_living_being_hit(
    mut living_being_hit_events: EventReader<LivingBeingHitEvent>,
    mut send_living_being_death: EventWriter<LivingBeingDeathEvent>,
    mut send_audio_hit_event: EventWriter<AudioHitEvent>,
) {
    for event in living_being_hit_events.iter() {
        send_living_being_death.send(LivingBeingDeathEvent {
            entity: event.entity,
        });
        send_audio_hit_event.send(AudioHitEvent);
    }
}

pub fn on_living_being_dead(
    mut living_being_death_events: EventReader<LivingBeingDeathEvent>,
    mut commands: Commands,
    mut last_despawned_entity: ResMut<LastDespawnedEntity>,
) {
    for event in living_being_death_events.iter() {
        if event.entity != last_despawned_entity.entity {
            commands.entity(event.entity).despawn_recursive();
            last_despawned_entity.entity = event.entity;
        }
    }
}
