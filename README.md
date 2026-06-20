# Toy Robot Arm Control

Rust control scaffold for a LewanSoul/Hiwonder-style 6DOF toy robot arm with
five servos, a mechanical claw, and a planned stereo webcam feedback loop.

The current code is intentionally conservative: it separates pure arm/vision
math from hardware transport so the controller can be tested before connecting
servos.

## Layout

- `src/arm.rs`: servo IDs, pulse limits, poses, validation, and clamping.
- `src/control.rs`: simple proportional visual-servo controller.
- `src/vision.rs`: stereo pinhole geometry for disparity-to-3D conversion.
- `src/transport.rs`: controller-frame encoding and mock transport.
- `sources/`: hardware notes, protocol assumptions, and calibration checklist.

## Commands

```sh
cargo test
cargo run -- sim
cargo run -- frame --ms 750 1:1500 2:1450 3:1600
```

`sim` runs one quasi-closed-loop step from a neutral pose toward a sample
camera target and prints the servo commands plus encoded controller frame.

`frame` only builds the serial frame for direct servo pulse commands. Use it to
inspect bytes before adding a real serial transport.

## Hardware Plan

1. Record arm dimensions, servo IDs, neutral pulses, and safe pulse limits in
   `sources/field-measurements.md`.
2. Calibrate each webcam independently for focal length and distortion.
3. Measure the camera baseline and run stereo calibration.
4. Validate serial protocol on an unpowered controller or with servos detached.
5. Add a real transport backend after protocol and serial settings are verified.

The first controller is visual servoing, not full inverse kinematics. It moves
joint pulses from observed 3D target error with clamp and rate limits. Full IK
can be added once link lengths and joint axes are measured.
