use std::{default, marker::PhantomData};

use bevy::{
    ecs::{component::Immutable, relationship},
    prelude::*,
    sprite::Anchor,
};

pub trait GetValue {
    fn get(&self) -> f32;
}

pub trait IntoStatusBar: Send + Sync + 'static {
    type GetValue: GetValue + Component;
    type GetMaxValue: GetValue + Component;
}

#[derive(Component)]
#[relationship_target(relationship=StatusBarOf)]
pub struct StatusBarRef(Vec<Entity>);

#[derive(Component)]
#[relationship(relationship_target=StatusBarRef)]
pub struct StatusBarOf(pub Entity);

#[derive(Component)]
#[require(StatusBarOf = enforce_exists!(StatusBarOf), Sprite = enforce_exists!(Sprite))]
#[component(immutable)]
pub struct StatusBar {
    size: Vec2,
}

impl StatusBar {
    pub fn new(size: Vec2) -> Self {
        Self { size }
    }
}

#[derive(Component)]
#[require(StatusBar = enforce_exists!(StatusBar))]
#[component(immutable)]
pub struct StatusBarType<T: IntoStatusBar>(PhantomData<T>);

impl<T: IntoStatusBar> Default for StatusBarType<T> {
    fn default() -> Self {
        Self(Default::default())
    }
}

#[derive(Component)]
#[require(StatusBar = enforce_exists!(StatusBar))]
#[component(immutable)]
pub struct StatusBarBackground(pub Sprite);

#[derive(Component)]
#[require(StatusBar = enforce_exists!(StatusBar))]
#[component(immutable)]
pub struct StatusBarBorder(pub Sprite);

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
    tr: Trigger<OnInsert, StatusBar>,
    mut status_bars: Query<
        (
            &mut Sprite,
            &StatusBar,
            &StatusBarOf,
            Option<&StatusBarBackground>,
            Option<&StatusBarBorder>,
        ),
        With<StatusBarType<T>>,
    >,
    targets: Query<(&T::GetMaxValue, &T::GetValue)>,
    mut commands: Commands,
) {
    let e = tr.target();
    let (mut sprite, bar, tatget, bg, border) = status_bars.get_mut(e).unwrap();
    let (max_val, val) = targets.get(tatget.0).unwrap();

    let progress_size = bar.size.with_x(bar.size.x * val.get() / max_val.get());
    sprite.custom_size = Some(progress_size);
    sprite.anchor = Anchor::CenterLeft;

    if let Some(bg) = bg {
        commands.entity(e).with_child((
            Transform::from_xyz(0.0, 0.0, -1.0),
            Sprite {
                custom_size: Some(bar.size),
                anchor: Anchor::CenterLeft,
                ..bg.0.clone()
            },
        ));
    }

    if let Some(border) = border {
        commands.entity(e).with_child((
            Transform::from_xyz(0.0, 0.0, 1.0),
            Sprite {
                custom_size: Some(bar.size),
                anchor: Anchor::CenterLeft,
                ..border.0.clone()
            },
        ));
    }
}

fn handle_change<T: IntoStatusBar>(
    tr: Trigger<OnInsert, (T::GetValue, T::GetMaxValue)>,
    targets: Query<(&T::GetMaxValue, &T::GetValue, &StatusBarRef)>,
    mut status_bars: Query<(&mut Sprite, &StatusBar)>,
) {
    let target = tr.target();
    let Ok((max_val, val, bar_ref)) = targets.get(target) else {
        return;
    };

    let Some(bar_entity) = bar_ref.0.iter().find(|bar| status_bars.contains(**bar)) else {
        return;
    };

    let (mut sprite, bar) = status_bars.get_mut(*bar_entity).unwrap();
    let progress_size = bar.size.with_x(bar.size.x * val.get() / max_val.get());

    sprite.custom_size = Some(progress_size);
}

fn handle_remove<T: IntoStatusBar>(
    tr: Trigger<OnRemove, T::GetValue>,
    targets: Query<&StatusBarRef>,
    mut status_bars: Query<(&mut Sprite, &StatusBar)>,
) {
    let target = tr.target();
    let Ok(bar_ref) = targets.get(target) else {
        return;
    };

    let Some(bar_entity) = bar_ref.0.iter().find(|bar| status_bars.contains(**bar)) else {
        return;
    };

    let (mut sprite, bar) = status_bars.get_mut(*bar_entity).unwrap();
    let progress_size = bar.size.with_x(0.0);

    sprite.custom_size = Some(progress_size);
}
