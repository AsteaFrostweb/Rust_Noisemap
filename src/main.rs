//use statmenets
use bevy::{prelude::*, render::render_resource::Buffer, transform::commands, asset::LoadState, time};
use libnoise::*;
use noise::*;

//Module definitions
mod noise;

// Consts
const NOISE_MAP_WIDTH: usize = 100;
const NOISE_MAP_HEIGHT: usize = 100;
const NOISE_MAP_DEPTH: usize = 1000;
#[derive(Resource)]
struct FlowState
{
    time_since_change: f32,
    change_interval: f32,
    z_position: f32,
    speed: f32
}


fn main() {
    
    let _ = App::new()
    .add_plugins(DefaultPlugins)
    .add_systems(Startup, setup_from_load)
    .add_systems(Update, update_sprite)
    .run();
}

fn setup_from_load(mut commands: Commands, asset_server: Res<AssetServer>, mut texture_atlases: ResMut<Assets<TextureAtlas>>)
{
    commands.spawn(Camera2dBundle::default());

    let noise_map = NoiseMap3D::load(String::from("test.noise3d")).unwrap();   
    let noise_flow: FlowState = FlowState
     {
         speed: 1.0,
         z_position: 0.0,
         time_since_change: 0.0,
         change_interval: 0.033,
     };

    let img = noise_map.get_slice_z(75).to_image();
    // Create texture from the image data
    let image_handle = asset_server.add(img);   

    
    // Create a sprite bundle
    commands.spawn(SpriteBundle {
        sprite: Sprite{ color: Color::WHITE, flip_x: false, flip_y: false, anchor: bevy::sprite::Anchor::Center,custom_size: Some(Vec2{x: 1000.0,y: 1000.0}), ..Default::default()},
        texture: image_handle.clone(),
        ..Default::default()
    });
      
    commands.insert_resource(noise_map);    
    commands.insert_resource(noise_flow);

}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut texture_atlases: ResMut<Assets<TextureAtlas>>)
{
    commands.spawn(Camera2dBundle::default());

    let perlin_source = Perlin::new(3127812567548);
    //generating a NoiseValues struct for the noise generator
    let perlin_values = NoiseValues
    {
        octaves: 2,
        scale: 0.075,
        persistence: 0.5,
        lacunarity: 2.0,
    };
    //generating the noise generator obj
    let noise_gen = NoiseGenerator3D
    {
        source: perlin_source,
        values: perlin_values,
    };    

    let mut noise_map: NoiseMap3D = NoiseMap3D::from_values(NOISE_MAP_WIDTH, NOISE_MAP_HEIGHT, NOISE_MAP_DEPTH);
    noise_map.populate_from_perlin(&noise_gen);

    let _ = noise_map.save(String::from("test.noise3d"));

   let noise_flow: FlowState = FlowState
    {
        speed: 0.5,
        z_position: 0.0,
        time_since_change: 0.0,
        change_interval: 0.33,
    };


    let img = noise_map.get_slice_z(75).to_image();

    // Create texture from the image data
    let image_handle = asset_server.add(img);   

    
    // Create a sprite bundle
    commands.spawn(SpriteBundle {
        sprite: Sprite{ color: Color::WHITE, flip_x: false, flip_y: false, anchor: bevy::sprite::Anchor::Center,custom_size: Some(Vec2{x: 1000.0,y: 1000.0}), ..Default::default()},
        texture: image_handle.clone(),
        ..Default::default()
    });
      
    commands.insert_resource(noise_map);    
    commands.insert_resource(noise_flow);

}

fn update_sprite(
    mut commands: Commands,
    mut query: Query<(&mut Handle<Image>, Entity)>,
    //mut sprites_query: Query<&mut Sprite>,
    noise_map: Res<NoiseMap3D>,
    asset_server: Res<AssetServer>,
    mut noise_flow: ResMut<FlowState>,
    time: Res<Time>,
) {
    noise_flow.time_since_change += time.delta_seconds_f64() as f32;
    if(noise_flow.time_since_change < noise_flow.change_interval)
    {
        return;
    }else
    {
        noise_flow.time_since_change = 0.0;
    }

    noise_flow.z_position += noise_flow.speed;
    
    println!("Moving z slice to: {}", noise_flow.z_position as i32);

    // Get the image and create a new texture handle
    let img = noise_map.get_slice_z(noise_flow.z_position as u32).to_image();
    let new_image_handle = asset_server.add(img);
    
    for(handle_image, entity) in &query
    {
        commands.entity(entity)
        .remove::<SpriteBundle>()
        .insert(SpriteBundle {
            sprite: Sprite{ color: Color::WHITE, flip_x: false, flip_y: false, anchor: bevy::sprite::Anchor::Center,custom_size: Some(Vec2{x: 1000.0,y: 1000.0}), ..Default::default()},
            texture: new_image_handle.clone(),
            ..Default::default()
        });
        
            
    }
    
}
