use std::ops::Neg;

use bevy::asset::LoadState;
use bevy::prelude::*;
use bevy::utils::petgraph::visit::Walker;

use crate::states::MainState;

pub struct AssetPlugin;

impl Plugin for AssetPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AssetList>()
            .add_systems(Startup, check_asset_loading.in_set(MainState::LoadAssets));
        //This is updated after the old one was depreciated in .11: https://bevyengine.org/learn/migration-guides/0-10-to-0-11/
    }
}

#[derive(Default, Resource)]
pub struct AssetList(pub Vec<UntypedHandle>); //HandleUntyped was changed to UntypedHandle in .12: https://bevyengine.org/learn/migration-guides/0-11-to-0-12/

pub fn check_asset_loading(
    asset_server: Res<AssetServer>,
    asset_list: Res<AssetList>,
    mut next_state: ResMut<NextState<MainState>>,
) {
    // todo!("There is a better way to do this with events.")
    let mut loaded = false; // Had to rewrite this setion, as the entire API was changed in .12: https://bevyengine.org/learn/migration-guides/0-11-to-0-12/

    for asset in asset_list.0.iter() {
        let id = asset.id();
        loaded = asset_server.is_loaded_with_dependencies(asset.id());
    }
    if loaded {
        next_state.set(MainState::Game);
    } else {
        error!("asset loading error")
    }
}
