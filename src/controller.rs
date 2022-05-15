use bevy::ecs as bevy_ecs;
use bevy::prelude::*;
use bevy_ecs::prelude::Component;
use spacenav_plus::Event as SpaceEvent;
use spacenav_plus::Event;
use spacenav_plus::MotionEvent;

pub struct SpaceController;

#[derive(Default)]
struct LockState {
    // Should rotations be sent through?
    rotate_locked: bool,
    // Should translations be sent through?
    translate_locked: bool,
}

impl Plugin for SpaceController {
    fn build(&self, app: &mut App) {
        app.init_resource::<LockState>()
            .add_event::<SpaceEvent>()
            .add_system(space_controller)
            .add_system(space_controller_relative);
    }
}

/// Entities that should be controlled by the space mouse can be marked up with this component.
#[derive(Component)]
pub struct SpaceMouseControllable;

fn space_controller(
    mut ev_levelup: EventReader<SpaceEvent>,
    mut query: Query<&mut Transform, With<SpaceMouseControllable>>,
    mut locks: ResMut<LockState>,
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
                if click_event.press {
                    if click_event.bnum == 0 {
                        locks.rotate_locked = !locks.rotate_locked;
                    } else if click_event.bnum == 1 {
                        locks.translate_locked = !locks.translate_locked;
                    } else {
                        // Some models have a lot of buttons...
                        println!("{:?}", click_event);
                    }
                }
            }
        }
    }
}

/// Entities that should be controlled by the space mouse can be marked up with this component.
#[derive(Component)]
pub struct SpaceMouseRelativeControllable;

fn space_controller_relative(
    mut ev_levelup: EventReader<SpaceEvent>,
    mut query: Query<&mut Transform, With<SpaceMouseRelativeControllable>>,
    mut locks: ResMut<LockState>,
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
                    // We can't simply add the x, y and z as that translation should be relative
                    // to where we are looking.

                    let (x, y, z) = (*x as f32 * scale, *y as f32 * scale, *z as f32 * scale);

                    let forward = controllable.forward();
                    let right = controllable.right();
                    let up = controllable.up();

                    if !locks.translate_locked {
                        controllable.translation += x * right + y * up + z * forward;
                    }

                    if !locks.rotate_locked {
                        let rot = Quat::from_euler(
                            EulerRot::XYZ,
                            *rx as f32 * scale,
                            *ry as f32 * scale,
                            *rz as f32 * scale * -1.,
                        );
                        controllable.rotation *= rot;
                    }
                }
            }
            Event::Button(click_event) => {
                if click_event.press {
                    if click_event.bnum == 0 {
                        locks.rotate_locked = !locks.rotate_locked;
                    } else if click_event.bnum == 1 {
                        locks.translate_locked = !locks.translate_locked;
                    } else {
                        // Some models have a lot of buttons...
                        println!("{:?}", click_event);
                    }
                }
            }
        }
    }
}
