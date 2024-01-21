use bevy::prelude::*;
use noise::*;


mod noise;
fn main() {
    
    let _ = App::new()
    .add_plugins(DefaultPlugins)
    .add_systems(Startup, setup)
    .run();
}

fn setup(mut commands: Commands)
{
    
}