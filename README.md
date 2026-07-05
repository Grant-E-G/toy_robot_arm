# Toy Robot Arm Control

Rust control scaffold for a LewanSoul/Hiwonder-style 6DOF toy robot arm with
six controller channels, a mechanical claw, and webcam-guided feedback.

The current code is intentionally conservative: it separates pure arm/vision
math from hardware transport so the controller can be tested before connecting
servos.

## Layout

- `src/arm.rs`: servo IDs, pulse limits, poses, validation, and clamping.
- `src/control.rs`: simple proportional visual-servo controller.
- `src/vision.rs`: pinhole/stereo geometry helpers for camera feedback.
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

## Python Hardware Tools

Create and activate the Conda environment:

```sh
conda env create -f environment.yml
conda activate toy-robot-arm
```

The environment includes `pyserial` for serial-port discovery:

```sh
python -m serial.tools.list_ports
```

Verify USB serial assumptions before sending anything:

```sh
python tools/controller_verify.py
python tools/controller_verify.py --device /dev/ttyUSB0 --baud 9600 --listen-seconds 2
```

`controller_verify.py` lists likely USB serial devices, prints Linux tty/USB
details, opens the selected port in raw mode, and optionally listens for
unsolicited bytes. It never transmits, so it should not move a servo.

Build a one-channel jog packet without sending it:

```sh
python tools/safe_jog.py --channel 1 --current-pulse 1500 --delta 10
```

Only after the controller port, baud rate, power setup, and channel risk are
understood, transmit the jog and return frames:

```sh
python tools/safe_jog.py \
  --device /dev/ttyUSB0 \
  --channel 1 \
  --current-pulse 1500 \
  --delta 10 \
  --send \
  --i-understand-this-can-move
```

`safe_jog.py` clamps the initial test window to `1100..=1900 us` and refuses
deltas larger than `20 us`. The `--current-pulse` value is a best-known command
position, not measured servo feedback; if it is wrong, the first transmitted
command can still cause a larger move than intended. For the first live test,
use detached linkages, servos unloaded, or the servo rail off while validating
that the controller accepts frames.

The updated hardware notes identify the intended kit supply as `7.5 V DC, 3 A`.
Verify DC output, connector fit, and polarity before powering the board. A
current-capable `6 V` bench supply is still useful for cautious debugging, but
it may reduce torque or trigger a low-voltage alarm.

## Hardware Plan

1. Record arm dimensions, servo IDs, neutral pulses, and safe pulse limits in
   `sources/field-measurements.md`.
2. Start with one overhead camera and fiducial markers for X/Y visual servoing.
3. Validate serial protocol with servos detached, linkages unloaded, or the
   servo rail disabled if the controller allows logic-only USB testing.
4. Add a real transport backend after protocol and serial settings are verified.
5. Add side-camera or stereo calibration only after overhead tracking works.

The first controller is visual servoing, not full inverse kinematics. It moves
joint pulses from observed 3D target error with clamp and rate limits. Full IK
can be added once link lengths and joint axes are measured.
