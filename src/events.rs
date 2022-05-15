use bevy::prelude::*;
pub use spacenav_plus::Event as SpaceEvent;

pub struct SpaceMouse;

impl Plugin for SpaceMouse {
    fn build(&self, app: &mut App) {
        app.add_event::<SpaceEvent>()
            .add_system(space_mouse)
            .add_startup_system(setup);
    }
}

fn setup() {
    if let Ok(()) = spacenav_plus::lib::spnav_open() {
        // println!("space mouse open");
    } else {
        eprintln!("open failed! maybe spacenavd is not running? (or you don't have a spacemouse plugged in)");
    }
}

fn space_mouse(mut ev_motion: EventWriter<SpaceEvent>) {
    if let Some(event) = spacenav_plus::lib::spnav_poll_event() {
        ev_motion.send(event);
    }
}
