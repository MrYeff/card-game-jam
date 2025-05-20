mod card;
mod card_slot;
use bevy::prelude::*;

use crate::{assets::AssetHandles, card::Card, card_slot::CardSlot};

trait SpriteRepr: Component + Sized {
    fn to_sprite(&self, assets: &AssetHandles) -> Sprite;

    fn handle_insert(
        tr: Trigger<OnInsert, Self>,
        cards: Query<&Self>,
        assets: Res<AssetHandles>,
        mut commands: Commands,
    ) {
        let entity = tr.target();
        let component = cards.get(entity).unwrap();
        commands.entity(entity).insert(component.to_sprite(&assets));
    }

    fn handle_remove(tr: Trigger<OnRemove, Self>, mut commands: Commands) {
        let entity = tr.target();
        commands.entity(entity).remove::<Sprite>();
    }
}

pub struct SpriteReprPlugin;

impl Plugin for SpriteReprPlugin {
    fn build(&self, app: &mut App) {
        register::<Card>(app);
        register::<CardSlot>(app);
    }
}

fn register<T: SpriteRepr>(app: &mut App) {
    app.add_observer(T::handle_insert)
        .add_observer(T::handle_remove);
}
