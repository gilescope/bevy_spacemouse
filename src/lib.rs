use crate::controller::SpaceController;
use crate::events::SpaceMouse;
use bevy::prelude::*;

pub mod controller;
pub mod events;

pub struct SpaceMousePlugin;

pub use controller::SpaceMouseControllable;
pub use controller::SpaceMouseRelativeControllable;
pub use controller::Scale;

impl Plugin for SpaceMousePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(SpaceMouse).add_plugin(SpaceController);
    }
}
