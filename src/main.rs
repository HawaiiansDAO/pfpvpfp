use bevy::prelude::*;

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

struct SupTimer(Timer);
pub struct SupPlugin;

impl Plugin for SupPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SupTimer(Timer::from_seconds(2.0, true)))
            .add_startup_system(create_npc)
            .add_system(talk_npc);
    }
}

fn create_npc(mut commands: Commands) {
    commands.spawn().insert(Person).insert(Name("Callus Pindakkus".to_string()));
    commands.spawn().insert(Person).insert(Name("Vushnak Bahon"   .to_string()));
    commands.spawn().insert(Person).insert(Name("Siafu"           .to_string()));
    commands.spawn().insert(Person).insert(Name("Falaris"         .to_string()));
}

fn talk_npc(time: Res<Time>,mut timer: ResMut<SupTimer>, query: Query<&Name, With<Person>>) {
    if timer.0.tick(time.delta()).just_finished() {
        for name in query.iter() {
            println!("sup {}, you faka!", name.0);
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(SupPlugin)
        .run();
}