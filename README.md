# Godot-Rust Test Project

This project demonstrates using Godot and Rust together.
It includes an editor plugin that automatically builds the native library with a python script before running/playing.
Sometimes `gdnative` seems to break and emits errors when built by Godot, doing a clean build with `cargo clean` seems to fix this.
