use bevy::prelude::*;

fn main() {
    
    let _ = App::new()
    .add_plugins(DefaultPlugins)
    .add_systems(Startup, setup)
    .run();
}

fn setup(mut commands: Commands)
{
    
}