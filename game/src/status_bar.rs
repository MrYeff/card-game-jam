use std::marker::PhantomData;

use bevy::prelude::*;

pub trait GetValue {
    fn get(&self) -> f32;
}

pub trait IntoStatusBar: Send + Sync + 'static {
    type GetValue: GetValue + Component;
    type GetMaxValue: GetValue + Component;
}

#[derive(Component)]
#[relationship_target(relationship=StatusBarOf)]
pub struct StatusBar(Vec<Entity>);

#[derive(Component)]
#[relationship(relationship_target=StatusBar)]
#[require(StatusBarDir, Sprite = enforce_exists!(Sprite))]
pub struct StatusBarOf(pub Entity);

#[derive(Component, Default)]
#[component(immutable)]
pub enum StatusBarDir {
    #[default]
    Horizontal,
    Vertical,
}

#[derive(Component)]
#[component(immutable)]
struct ContainerSize(Vec2);

#[derive(Component)]
#[require(StatusBarOf = enforce_exists!(StatusBarOf))]
#[component(immutable)]
pub struct StatusBarType<T: IntoStatusBar>(PhantomData<T>);

impl<T: IntoStatusBar> Default for StatusBarType<T> {
    fn default() -> Self {
        Self(Default::default())
    }
}

pub struct StatusBarPlugin<T: IntoStatusBar>(PhantomData<T>);

impl<T: IntoStatusBar> Default for StatusBarPlugin<T> {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<T: IntoStatusBar> Plugin for StatusBarPlugin<T> {
    fn build(&self, app: &mut App) {
        app.add_observer(handle_new::<T>)
            .add_observer(handle_change::<T>)
            .add_observer(handle_remove::<T>);
    }
}

#[allow(clippy::type_complexity)]
fn handle_new<T: IntoStatusBar>(
    tr: Trigger<OnInsert, StatusBarOf>,
    mut status_bars: Query<(&mut Sprite, &StatusBarOf, &StatusBarDir), With<StatusBarType<T>>>,
    targets: Query<(&T::GetMaxValue, &T::GetValue)>,
    mut commands: Commands,
) {
    let e = tr.target();
    let (mut sprite, target, dir) = status_bars.get_mut(e).unwrap();
    let (max_val, val) = targets.get(target.0).unwrap();

    let size = sprite
        .custom_size
        .expect("Sprite should have customs size!"); // TODO retrieve actual size
    commands.entity(e).insert(ContainerSize(size));

    let frac = val.get() / max_val.get();
    let progress_size = match dir {
        StatusBarDir::Horizontal => size.with_x(size.x * frac),
        StatusBarDir::Vertical => size.with_y(size.y * frac),
    };

    sprite.custom_size = Some(progress_size);
}

fn handle_change<T: IntoStatusBar>(
    tr: Trigger<OnInsert, (T::GetValue, T::GetMaxValue)>,
    targets: Query<(&T::GetMaxValue, &T::GetValue, &StatusBar)>,
    mut status_bars: Query<(&mut Sprite, &ContainerSize, &StatusBarDir)>,
) {
    let target = tr.target();
    let Ok((max_val, val, bar_ref)) = targets.get(target) else {
        return;
    };

    let Some(bar_entity) = bar_ref.0.iter().find(|bar| status_bars.contains(**bar)) else {
        return;
    };

    let (mut sprite, size, dir) = status_bars.get_mut(*bar_entity).unwrap();

    let size = size.0;

    let frac = val.get() / max_val.get();
    let progress_size = match dir {
        StatusBarDir::Horizontal => size.with_x(size.x * frac),
        StatusBarDir::Vertical => size.with_y(size.y * frac),
    };

    sprite.custom_size = Some(progress_size);
}

fn handle_remove<T: IntoStatusBar>(
    tr: Trigger<OnRemove, T::GetValue>,
    targets: Query<&StatusBar>,
    mut status_bars: Query<(&mut Sprite, &ContainerSize, &StatusBarDir)>,
) {
    let target = tr.target();
    let Ok(bar_ref) = targets.get(target) else {
        return;
    };

    let Some(bar_entity) = bar_ref.0.iter().find(|bar| status_bars.contains(**bar)) else {
        return;
    };

    let (mut sprite, size, dir) = status_bars.get_mut(*bar_entity).unwrap();
    let size = size.0;

    let progress_size = match dir {
        StatusBarDir::Horizontal => size.with_x(0.0),
        StatusBarDir::Vertical => size.with_y(0.0),
    };

    sprite.custom_size = Some(progress_size);
}
