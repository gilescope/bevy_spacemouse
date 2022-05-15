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
        app.init_resource::<LockState>();
        // app.add_event::<SpaceEvent>().add_system(space_controller);
        app.add_event::<SpaceEvent>().add_system(space_controller_relative);
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
                    // to where we are looking. Where we are looking is `controllable.rotation`.

                    let (x,y,z) = (*x as f32 * scale,*y as f32* scale,*z as f32* scale);


                    use std::f32::consts::PI;
                    
                    let into = Quat::from_euler(EulerRot::XYZ, -PI / 2., -PI, PI / 2.); // radians
                    let (into_x,into_y,into_z) = (controllable.rotation * into).to_euler(EulerRot::XYZ);

                    let right = Quat::from_euler(EulerRot::XYZ, -PI / 2., PI, PI); // radians
                    let (right_x,right_y,right_z) = (controllable.rotation * right).to_euler(EulerRot::XYZ);

                    let up = Quat::from_euler(EulerRot::XYZ, -PI, PI / 2., PI); // radians
                    let (up_x,up_y,up_z) = (controllable.rotation * up).to_euler(EulerRot::XYZ);

                    // controllable.translation.x += y * up_x;
                    // controllable.translation.y += y * up_y;
                    // controllable.translation.z += y * up_z;

                    // controllable.translation.x += x * right_x;
                    // controllable.translation.y += x * right_y;
                    // controllable.translation.z += x * right_z;

                    // controllable.translation.x += z * into_x;
                    // controllable.translation.y += z * into_y;
                    // controllable.translation.z += z * into_z;

                    if !locks.translate_locked {
                        controllable.translation.x += x * right_x + y * up_x + z * into_x;
                        controllable.translation.y += x * right_y + y * up_y + z * into_y;
                        controllable.translation.z += x * right_z + y * up_z + z * into_z;
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
