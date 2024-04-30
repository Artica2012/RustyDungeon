const ATLAS_PATH: &str = "ascii.png";

// pub fn load_assets(
//     mut commands: Commands,
//     asset_server: Res<AssetServer>,
//     mut texture_atlasses: ResMut<Assets<TextureAtlasLayout>>,
//     mut asset_list: ResMut<crate::assets::AssetList>,
// ) {
// println!("Loading Asset");
// let texture = asset_server.load(ATLAS_PATH);
// asset_list.0.push(texture.clone().untyped());
// //The Texture atlas API was completely reworked in .13.
// let layout = TextureAtlasLayout::from_grid(Vec2::splat(10.), 16, 16, None, None);
//
// let handle = texture_atlasses.add(layout);
// commands.insert_resource(GraphicsAssets {
//     atlas: handle,
//     sprite_texture: texture.clone(),
// })
// }

// pub fn load_graphic_assets(
//     mut commands: Commands,
//     rpg_sprite_handles: Res<RPGSpriteFolder>,
//     asset_server: Res<AssetServer>,
//     mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
//     loaded_folders: Res<Assets<LoadedFolder>>,
//     mut textures: ResMut<Assets<Image>>
// ){
//     let loaded_folder = loaded_folders.get(&rpg_sprite_handles.0).unwrap();
//     let layout = TextureAtlasLayout::from_grid(Vec2::splat(10.), 16, 16, None, None);
//     let handle = texture_atlases.add(layout.clone());
//     for x in loaded_folder.handles.iter(){
//         if x.path() == Option(&asset::AssetPath::parse("ascii.png")) {
//             commands.insert_resource(GraphicsAssets {
//                 atlas: handle,
//                 sprite_texture: Handle::try_from(x.clone()).unwrap(),
//             })
//         }
//     }
// }
