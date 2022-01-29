use bevy::prelude::*;


fn sup() {
    println!("sup u faka!")
}

fn main() {
    Add::new()
        .add_system(sup)
        .run();
}