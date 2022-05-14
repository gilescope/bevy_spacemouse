use bevy::ecs as bevy_ecs;
use bevy::prelude::*;
use bevy_ecs::prelude::Component;
use spacenav_plus::Event as SpaceEvent;
use spacenav_plus::Event;
use spacenav_plus::MotionEvent;

pub struct SpaceController;

impl Plugin for SpaceController {
    fn build(&self, app: &mut App) {
        app
            .add_event::<SpaceEvent>()
            .add_system(space_controller);
    }
}



/// Entities that should be controlled by the space mouse can be marked up with this component.
#[derive(Component)]
pub struct SpaceMouseControllable;


fn space_controller(
    mut ev_levelup: EventReader<SpaceEvent>,
    mut query: Query<&mut Transform, With<SpaceMouseControllable>>,
) {
    for ev in ev_levelup.iter() {
        match ev {
            Event::Motion(MotionEvent {
                x,
                y,
                z,
                rx,
                ry,
                rz,
                ..
            }) => {
                let scale = 0.0001;
                for mut controllable in query.iter_mut() {
                    controllable.translation.x += *x as f32 * scale;
                    controllable.translation.y += *y as f32 * scale;
                    controllable.translation.z += *z as f32 * scale * -1.;

                    let rot = Quat::from_euler(
                        EulerRot::XYZ,
                        *rx as f32 * scale,
                        *ry as f32 * scale,
                        *rz as f32 * scale * -1.,
                    );
                    controllable.rotation *= rot;
                }
            }
            Event::Button(click_event) => {
                println!("{:?}", click_event);
            }
        }
    }
}
