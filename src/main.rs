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

fn hello_world() {
    println!("hello world!");
}

fn greet_people(query: Query<&Name, With<Person>>) {
    for name in &query {
        println!("hello {}!", name.0);
    }
}



fn main() {
    App::new()
        .add_systems(Startup, add_people)
        .add_systems(Update, greet_people)
        .add_systems(Update, hello_world)
        .run();
}

