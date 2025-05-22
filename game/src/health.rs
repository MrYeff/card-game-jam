use std::cmp::{max, min};

use bevy::{ecs::world::OnDespawn, prelude::*};

use crate::status_bar::{GetValue, IntoStatusBar};

#[derive(Component)]
#[component(immutable)]
pub struct MaxHealth(pub u32);

#[derive(Component)]
#[require(MaxHealth = enforce_exists!(MaxHealth))]
#[component(immutable)]
pub struct Health(u32);

#[derive(Event)]
pub struct AdjustHealth(pub i32);

impl Health {
    #[inline(always)]
    pub fn new(val: u32) -> Self {
        Health(val)
    }

    #[inline(always)]
    pub fn get(&self) -> u32 {
        self.0
    }
}

pub struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(handle_adjust_health);
    }
}

fn handle_adjust_health(
    tr: Trigger<AdjustHealth>,
    health: Query<(Option<&Health>, &MaxHealth)>,
    mut commands: Commands,
) {
    let entity = tr.target();
    let (health, max_health) = health
        .get(entity)
        .expect("called adjust health on an entity witout MaxHealth");

    let new_health = max(
        0,
        min(max_health.0 as i32, health.map_or(0, |x| x.0 as i32) + tr.0),
    ) as u32;

    if new_health == 0 {
        commands.entity(entity).remove::<Health>();
    } else {
        commands.entity(entity).insert(Health(new_health));
    }
}

impl GetValue for Health {
    fn get(&self) -> f32 {
        self.0 as f32
    }
}

impl GetValue for MaxHealth {
    fn get(&self) -> f32 {
        self.0 as f32
    }
}

impl IntoStatusBar for Health {
    type GetValue = Health;
    type GetMaxValue = MaxHealth;
}
