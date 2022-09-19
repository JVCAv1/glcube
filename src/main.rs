mod startup;

use bevy::prelude::*;
use crate::startup::startup;

fn main() {
	App::new()
		.add_plugins(DefaultPlugins)
		.add_startup_system(startup)
		.run();
}
