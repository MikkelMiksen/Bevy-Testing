use bevy::prelude::*;

#[derive(Component)]
struct Person;


#[derive(Component)]
struct Name(String);

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Elaina Hume".to_string())));
    commands.spawn((Person, Name("FIREBALL!!".to_string())));
    commands.spawn((Person, Name("Mmm that was a good one, 10hi f9s".to_string())));
}

#[derive(Resource)]
struct GreetTimer(Timer);


fn update_people(mut query: Query<&mut Name, With<Person>>) {
    for mut name in &mut query {
        if name.0 == "Elaina Hume" {
            name.0 = "Cum in Mouth!".to_string();
            break; // We don't need to change any other names.
        }
    }
}


fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
    // update our timer with the time elapsed since the last update
    // if that caused the timer to finish, we say hello to everyone
    if timer.0.tick(time.delta()).just_finished() {
        for name in &query {
            println!("hello {}!", name.0);
        }
    }
}

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Once)));
        app.add_systems(Startup, add_people);
        app.add_systems(Update, (update_people, greet_people).chain());
    }
}

fn main() {

    App::new().add_plugins(DefaultPlugins).add_plugins(HelloPlugin).run();
}