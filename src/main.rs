mod startup;
use bevy::prelude::*;

fn main() {
	App::new()
		.add_startup_system(startup())
		.run();
}
