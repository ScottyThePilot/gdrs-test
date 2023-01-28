# Godot-Rust Test Project

This project demonstrates using Godot and Rust together.
It includes an editor plugin that automatically builds the native library with a python script before running/playing.
Sometimes `gdnative` seems to break and emits errors when built by Godot, doing a clean build with `cargo clean` seems to fix this.

## Setup for your own project
- Rename the `gdrs-test-native` and `gdrs-test-project` folders
- Rename the cargo project in `gdrs-test-native`
- Rename the `gdrs-test-native.gdnlib` and update the file paths to match the cargo project's name
  - Also rename the resource path in `gdrs-test-project/scripts/HelloRust.gdns`
- In the editor:
  - Rename the `res://scripts/HelloRust.gdns` script
    - Update the `HelloRust` struct in `gdrs-test-native/src/lib.rs` to match
  - Rename the `res://Main.tscn` scene, if so desired
  - Rename the project
