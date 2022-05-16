# bevy_spacemouse
Bevy spacemouse integration

Bevy game engine showing how to control things using a 3d space mouse.

In this example `.insert(SpaceMouseControllable);` has been added to the cube entity 
so that you control the cube:
```sh
cargo run --example cube
```

alternatively here the relative component is attached to the camera 
(`.insert(SpaceMouseRelativeControllable);`)
so that you control the view:

```sh
cargo run --example cube-camera-controlled
```

## dependencies:

This (alas) depends on c; 
if you're on mac then you can install
```
brew install libspnav
```

and also that your running the `spacenavd` daemon. (This is the open source driver for it.)
I think this driver only works on linux sorry!

```toml
[dependencies]
bevy_spacemouse = { git="https://github.com/gilescope/bevy_spacemouse.git" }
```

## imports:
```rust
use bevy_spacemouse::{SpaceMouseControllable, SpaceMousePlugin};
```

## plugin:

The SpaceMouse will trigger events when the mouse is moved.
```rust
    .add_plugin(SpaceMousePlugin)
```

## Configuration:

There's a resource called Scale that you can override the values there so that they
make sense in the scale of the world that you are living in. Feel free to set both values
to the same number.

## License

Apache 2 / MIT license like rustc is.
