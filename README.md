# bevy_spacemouse
Bevy spacemouse integration

Bevy game engine showing how to control things using a 3d space mouse.

In this example `.insert(SpaceMouseControllable);` has been added to the cube entity 
so that you control the cube:
```sh
cargo run --example cube
```

alternatively here the component is attached to the camera so that you control the view:

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

## License

Apache 2 / MIT license like rustc is.
