use bevy::prelude::*;

#[derive(Component)]
struct Person;


#[derive(Component)]
struct Name(String);

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Cum in Mouth wizard".to_string())));
    commands.spawn((Person, Name("Mmm that was a good one, guy".to_string())));
    commands.spawn((Person, Name("FIREBALL!!".to_string())));
}



fn update_people(mut query: Query<&mut Name, With<Person>>) {
    for mut name in &mut query {
        if name.0 == "Cum in Mouth wizard" {
            name.0 = "Elaina Hume".to_string();
            break; // We don't need to change any other names.
        }
    }
}


fn greet_people(query: Query<&Name, With<Person>>) {
    for name in &query {
        println!("hello {}!", name.0);
    }
}

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        // add things to your app here
        app.add_systems(Startup, add_people);
        app.add_systems(Update, (update_people, greet_people).chain());
    }
}

fn main() {
    App::new().add_plugins(DefaultPlugins).add_plugins(HelloPlugin).run();
}