use std::marker::PhantomData;

use bevy::{ecs::schedule::ScheduleLabel, prelude::*};

#[derive(Component)]
struct DespawnMarker;

#[derive(Event)]
pub struct DespawnDelayed;

#[derive(Default)]
pub struct DespawnPlugin<T: ScheduleLabel>(PhantomData<T>);

impl<T: ScheduleLabel + Default> Plugin for DespawnPlugin<T> {
    fn build(&self, app: &mut App) {
        app.add_observer(handle_despawn_entity)
            .add_systems(T::default(), despawn);
    }
}

fn handle_despawn_entity(tr: Trigger<DespawnDelayed>, mut commands: Commands) {
    commands.entity(tr.target()).insert(DespawnMarker);
}

fn despawn(qs: Query<Entity, With<DespawnMarker>>, mut commands: Commands) {
    for e in qs {
        commands.entity(e).try_despawn();
    }
}
