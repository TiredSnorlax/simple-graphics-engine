# Graphics Engine

This is a simple graphics engine built with Rust.

## Prerequisites

Ensure you have Rust and Cargo installed. You can find installation instructions at [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).

## How to Run

1.  Clone the repository.
2.  Navigate to the project directory.
3.  Run the application using Cargo:

```bash
cargo run
```

## Controls

-   **WASD**: Move the camera.
-   **Shift/Space**: Move the camera up/down.
-   **Arrow Keys**: Rotate the camera.

The default scene is a small map of a game.
The FPS might be low if you build it normally.
To improve performance, you can use the following command:

```bash
cargo run --release
```
