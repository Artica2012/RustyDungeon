use bevy::asset::LoadedFolder;
use bevy::prelude::*;

use crate::graphics::GraphicsAssets;
use crate::states::MainState;

const ATLAS_PATH: &str = "sprites/ascii.png";

pub struct AssetPlugin;

impl Plugin for AssetPlugin {
    fn build(&self, app: &mut App) {
        // app.init_resource::<AssetList>()
        //     .add_systems(Update, check_asset_loading.in_set(MainState::LoadAssets));
        //This is updated after the old one was depreciated in .11: https://bevyengine.org/learn/migration-guides/0-10-to-0-11/
        app.init_resource::<AssetList>()
            .init_resource::<RPGSpriteFolder>()
            .add_systems(OnEnter(MainState::LoadAssets), load_textures)
            .add_systems(
                Update,
                check_textures.run_if(in_state(MainState::LoadAssets)),
            );
    }
}

#[derive(Resource, Default)]
pub struct RPGSpriteFolder(pub(crate) Handle<LoadedFolder>);

pub fn load_textures(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut asset_list: ResMut<AssetList>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    println!("Loading Textures");
    commands.insert_resource((RPGSpriteFolder(asset_server.load_folder(("sprites")))));

    let texture = asset_server.load(ATLAS_PATH);
    asset_list.0.push(texture.clone().untyped());
    //The Texture atlas API was completely reworked in .13.
    let layout = TextureAtlasLayout::from_grid(Vec2::splat(10.), 16, 16, None, None);

    let handle = texture_atlases.add(layout);
    commands.insert_resource(GraphicsAssets {
        atlas: handle,
        sprite_texture: texture.clone(),
    })
}

fn check_textures(
    mut next_state: ResMut<NextState<MainState>>,
    rpgsprite_folder: Res<RPGSpriteFolder>,
    mut events: EventReader<AssetEvent<LoadedFolder>>,
) {
    for event in events.read() {
        println!("{:?}", event);
        if event.is_loaded_with_dependencies(&rpgsprite_folder.0) {
            println!("Advancing State to: Game");
            next_state.set(MainState::Game)
        }
    }
}

#[derive(Default, Resource)]
pub struct AssetList(pub Vec<UntypedHandle>); //HandleUntyped was changed to UntypedHandle in .12: https://bevyengine.org/learn/migration-guides/0-11-to-0-12/

// pub fn check_asset_loading(
//     asset_server: Res<AssetServer>,
//     asset_list: Res<AssetList>,
//     mut next_state: ResMut<NextState<MainState>>,
//     mut events: EventReader<AssetEvent<Image>>,
// ) {
//     // todo!("There is a better way to do this with events.")
//     // println!("Waiting for an event");
//     // let mut loaded = false; // Had to rewrite this setion, as the entire API was changed in .12: https://bevyengine.org/learn/migration-guides/0-11-to-0-12/
//     //
//     // for asset in asset_list.0.iter() {
//     //     let id = asset.id();
//     //     loaded = asset_server.is_loaded_with_dependencies(asset.id());
//     // }
//     // if loaded {
//     //     next_state.set(MainState::Game);
//     // } else {
//     //     error!("asset loading error")
//     // }
//
//     for event in events.read(){
//         match event {
//             AssetEvent::LoadedWithDependencies{id:_}=>{
//                 next_state.set(MainState::Game)
//             }
//             _ => {println!("{:?}", event)}
//         }
//         // if let AssetEvent::LoadedWithDependencies {id:_} = event {
//         //     println!("Asset Loaded");
//         //     next_state.set(MainState::Game);
//         // } else {
//         //     println!("Asset Not Loaded");
//         // }
//     }
// }
