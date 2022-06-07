use bevy::ecs as bevy_ecs;
use bevy::prelude::*;
use bevy::window::RequestRedraw;
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

// Alter these settings to reduce or increase the effect of the mouse.
pub struct Scale {
    pub rotate_scale: f32,
    pub translate_scale: f32,
}

impl Default for Scale {
    fn default() -> Self {
        Self {
            rotate_scale: 0.0001,
            translate_scale: 0.0001,
        }
    }
}

impl Plugin for SpaceController {
    fn build(&self, app: &mut App) {
        app.init_resource::<LockState>()
            .init_resource::<Scale>()
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
    scale: Res<Scale>,
    mut locks: ResMut<LockState>,
    mut event: EventWriter<RequestRedraw>,
) {
    let mut changed = false;
    for ev in ev_levelup.iter() {
        changed = true;
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
                for mut controllable in query.iter_mut() {
                    controllable.translation.x += *x as f32 * scale.translate_scale;
                    controllable.translation.y += *y as f32 * scale.translate_scale;
                    controllable.translation.z += *z as f32 * scale.translate_scale * -1.;

                    let rot = Quat::from_euler(
                        EulerRot::XYZ,
                        *rx as f32 * scale.rotate_scale,
                        *ry as f32 * scale.rotate_scale,
                        *rz as f32 * scale.rotate_scale * -1.,
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
    if changed {
        event.send(RequestRedraw);
    }
}

/// Entities that should be controlled by the space mouse can be marked up with this component.
#[derive(Component)]
pub struct SpaceMouseRelativeControllable;

fn space_controller_relative(
    mut ev_levelup: EventReader<SpaceEvent>,
    mut query: Query<&mut Transform, With<SpaceMouseRelativeControllable>>,
    scale: Res<Scale>,
    mut locks: ResMut<LockState>,
    mut event: EventWriter<RequestRedraw>,
) {
    let changed = false;
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
                for mut controllable in query.iter_mut() {
                    // We can't simply add the x, y and z as that translation should be relative
                    // to where we are looking.

                    let (x, y, z) = (
                        *x as f32 * scale.translate_scale,
                        *y as f32 * scale.translate_scale,
                        *z as f32 * scale.translate_scale,
                    );

                    let forward = controllable.forward();
                    let right = controllable.right();
                    let up = controllable.up();

                    if !locks.translate_locked {
                        controllable.translation += x * right + y * up + z * forward;
                    }

                    if !locks.rotate_locked {
                        let rot = Quat::from_euler(
                            EulerRot::XYZ,
                            *rx as f32 * scale.rotate_scale,
                            *ry as f32 * scale.rotate_scale,
                            *rz as f32 * scale.rotate_scale * -1.,
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
    if changed {
        event.send(RequestRedraw);
    }
}
