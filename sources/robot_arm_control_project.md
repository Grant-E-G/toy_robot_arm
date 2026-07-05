# Webcam-Guided Cheap Robot Arm Control Project

> Superseded hardware note: `robot_arm_control_project_v2.md` incorporates the
> newer Amazon listing screenshot and should be treated as the current raw
> hardware handoff. This older file is preserved as source history and contains
> earlier assumptions, including possible `LFD-01M` small servos and a likely
> spare channel 6, that should not override the v2 notes.

## Purpose

This project uses a low-cost Amazon / Hiwonder-style desktop robot arm as a prototype platform for webcam-guided manipulation. The goal is to make an inexpensive open-loop hobby servo arm perform simple tabletop tasks by wrapping it in an external vision feedback loop.

The intended architecture is:

```text
Laptop Python program
  -> USB serial or TTL serial
  -> Hiwonder-style 6-channel servo controller
  -> PWM hobby servos
  -> robot arm motion
  -> webcam observes gripper, object, and target
  -> Python sends corrected servo commands
```

This is a quasi-closed-loop robot. The servos themselves should be treated as open-loop position actuators. The closed-loop feedback comes from the webcam.

## Assumptions

### Confirmed From Photos

| Item | Evidence | Confidence |
|---|---|---:|
| Webcam is a NexiGo N60 1080p Full HD Webcam | Box label says "NEXIGO", "1080p Full HD Webcam", and "N60" | High |
| Controller board has 6 PWM servo outputs | Servo ports are numbered 1 through 6 and the board matches Hiwonder LSC-6 layout | High |
| Controller board has a 4-pin TTL serial header | Silkscreen label reads `5V RX TX GND` | High |
| Controller board has external power input | Blue screw terminal labeled `+` and `-`, plus barrel jack | High |
| Controller board has onboard Bluetooth module | Green module with printed antenna and `www.hc01.com` marking | High |
| Controller board has onboard power switch | Slide switch labeled `ON` / `OFF` | High |
| Controller board is likely revision V1.8 | Silkscreen near Bluetooth module appears to read `V1.8` | Medium |
| Servo headers use standard signal / power / ground convention | Silkscreen near servo ports shows `S + -` | High |

### Confirmed From Prior Arm Photos

| Item | Evidence | Confidence |
|---|---|---:|
| Two large servos are Hiwonder LDX-218 | Servo labels were readable in previous photos | High |
| Three smaller wrist / gripper servos are Hiwonder micro PWM servos | Physical match to Hiwonder-style arm; exact labels not readable | Medium |
| Arm is a 5-servo PWM arm | User reports 2 large servos plus 3 smaller wrist/gripper servos | High |

### Inferred From Manufacturer Documentation

| Item | Assumption | Why It Matters |
|---|---|---|
| Controller family is Hiwonder LSC-6 or close derivative | Board layout and features match the Hiwonder 6-channel Bluetooth servo controller | Determines serial protocol and channel count |
| USB serial uses Hiwonder LSC protocol | LSC-series docs describe `0x55 0x55` framed serial commands at 9600 baud | Needed for Python control |
| Servo command position values are pulse-width-like values, typically 500 to 2500 | Hiwonder examples command values such as 1200, 2000, 2300 | Code should command pulse units first, not degrees |
| Small servos may only be 4.8 to 6.0 V tolerant | Likely LFD-01M-class micro servos | Shared servo rail should start at 6.0 V |
| NexiGo N60 behaves as a normal USB webcam / UVC device | Official page says plug-and-play, USB 2.0/3.0, driver-free | OpenCV should be able to capture from it |

### Things The Coding Agent Must Verify

Do not treat these as known until tested:

1. Which `/dev/tty*` device is the servo controller.
2. Whether the controller enumerates as USB serial without extra drivers.
3. Whether the LSC protocol works on this specific board.
4. Whether servo IDs are `1..6`, `0..5`, or app-specific.
5. Which controller channel maps to each physical joint.
6. Actual safe PWM limits for each joint.
7. Whether the small servos tolerate more than 6 V.
8. Which OpenCV camera index corresponds to the NexiGo N60.
9. Whether the N60 exposes MJPG at 1920x1080 30 FPS on the chosen machine.
10. Camera distortion and field-of-view effects after mounting.

## Hardware Summary

### Robot Arm

| Subsystem | Count | Likely Part | Confidence | Notes |
|---|---:|---|---:|---|
| Large base / arm servos | 2 | Hiwonder LDX-218 | High | Read from servo labels |
| Wrist / gripper servos | 3 | Hiwonder micro PWM servos, likely LFD-01M-class or similar | Medium | Exact model not readable |
| Servo controller | 1 | Hiwonder LSC-6-style 6-channel Bluetooth PWM servo controller | High | Board photo matches features |
| Webcam | 1 or 2 | NexiGo N60 1080p Full HD Webcam | High | Box label visible |

### Hiwonder LDX-218 Large Servo

Use the official LDX-218 page as the source of truth when coding limits and comments. Key practical values:

| Parameter | Value |
|---|---|
| Control type | Standard PWM servo |
| Working voltage | 6.0 to 8.4 V |
| Nominal control angle | 180 deg |
| Pulse width range | 500 to 2500 us |
| PWM period | 20 ms |
| Torque | About 15 kg-cm at 6 V; about 17 kg-cm at 7.4 V |

Source: [Hiwonder LDX-218](https://www.hiwonder.com/products/ldx-218)

### Likely Hiwonder LFD-01M-Class Micro Servos

The small servos are not positively identified. Treat them conservatively.

| Parameter | Conservative Initial Assumption |
|---|---|
| Control type | Standard PWM servo |
| Working voltage | 4.8 to 6.0 V |
| Nominal control angle | 180 deg |
| Pulse width range | 500 to 2500 us |
| Safe initial command range | 1100 to 1900 us |

Source for similar class: [Hiwonder LFD-01M](https://www.hiwonder.com/products/lfd-01m)

### Servo Power

Initial recommended servo rail:

```text
6.0 V, 5 to 8 A bench supply or original kit supply
```

Do not power the servos from USB. Do not power the servos from the `5V` pin on the TTL header. The `5V RX TX GND` header is for logic / secondary development, not for driving servo current.

Do not run the shared servo rail at 7.4 to 8.4 V until the small servos are positively identified as high-voltage tolerant or are placed on a separate regulated rail.

## Controller Board

The controller appears to be a Hiwonder LSC-6-style 6-channel Bluetooth servo controller.

Useful visible features:

| Board Feature | Use |
|---|---|
| `ON` / `OFF` switch | Main board power switch |
| Blue `+` / `-` screw terminal | External servo power input |
| Barrel jack | Alternate external power input |
| USB connector | First-choice control interface from laptop |
| `5V RX TX GND` header | TTL serial interface for MCU / SBC secondary control |
| 6 servo headers | PWM servo outputs |
| Bluetooth module | Optional app / Bluetooth serial control |
| Buzzer | Likely low-voltage / protocol-error alarm |
| `220 uF 16 V` capacitor | Bulk input capacitance, not a safe-voltage recommendation |

Manufacturer source for matching controller family: [Hiwonder LSC-6](https://www.hiwonder.com/products/lsc-6)

Hiwonder lists the LSC-6 as a 6-channel servo controller with Bluetooth 4.0, TTL serial communication, low-voltage alarm, over-current protection, onboard action-group memory, and support for external single-chip secondary development.

### Serial Protocol Assumption

Hiwonder LSC-series documentation describes this serial protocol:

| Field | Value |
|---|---|
| Baud rate | 9600 |
| Frame header | `0x55 0x55` |
| Packet layout | `0x55 0x55 LEN CMD PARAMS...` |
| Length formula | `LEN = number_of_parameter_bytes + 2` |
| Servo move command | `CMD = 0x03` |

Source: [Hiwonder LSC Series Controller Communication Protocol](https://docs.hiwonder.com/projects/16-Channel-Servo-Controller/en/latest/docs/2_Communication_Protocol.html)

Example packet: move servo ID 1 to position 1500 over 1000 ms.

```python
packet = bytes([
    0x55, 0x55,   # frame header
    0x08,         # length
    0x03,         # CMD_SERVO_MOVE
    0x01,         # number of servos
    0xE8, 0x03,   # time = 1000 ms, little endian
    0x01,         # servo ID
    0xDC, 0x05,   # position = 1500, little endian
])
```

For multiple servos in one synchronized command:

```text
0x55 0x55 LEN 0x03 COUNT TIME_L TIME_H ID1 POS1_L POS1_H ID2 POS2_L POS2_H ...
```

Expected length:

```text
LEN = COUNT * 3 + 5
```

### TTL Serial Header Caution

The header labeled `5V RX TX GND` is useful, but USB should be tested first.

If using an external microcontroller:

| External Device | Controller Board |
|---|---|
| TX | RX |
| RX | TX |
| GND | GND |

If using a Raspberry Pi UART, protect the Pi RX pin unless the board TX voltage has been measured. The board likely uses 5 V TTL logic, and Raspberry Pi GPIO is not 5 V tolerant.

## Webcam

The webcam box identifies it as a NexiGo N60 1080p Full HD Webcam.

Official NexiGo N60 technical information:

| Parameter | Value |
|---|---|
| Model | N60 |
| Resolution | 1920x1080 |
| Frame rate | 30 FPS |
| Sensor | 2 MP CMOS |
| Field of view | 110 deg |
| Focus | Fixed focus |
| Focus range | 15.7 in to 118 in |
| Video formats | YUY2 / MJPG |
| USB | Plug-and-play USB |
| Tripod mount | 1/4 in compatible |

Sources: [NexiGo N60 product page](https://www.nexigo.com/products/nexigo-n60-1080p-webcam-with-microphone), [NexiGo N60 technical information](https://www.nexigo.com/pages/n60-technical-information)

### Camera Implications

The N60 is good enough for the prototype, but the 110 deg field of view is wide. That means:

1. Expect visible lens distortion.
2. Use camera calibration or homography before trusting metric measurements.
3. Keep the camera rigidly mounted.
4. Avoid moving autofocus issues because the camera is fixed focus.
5. Keep the workspace inside the stated focus range.
6. Prefer MJPG at 1080p if raw YUY2 bandwidth is too high.

For the first version, use one overhead camera. Add the second camera only after overhead tracking works.

Recommended camera roles:

| Camera | Role |
|---|---|
| Overhead camera | Track X/Y position of gripper, object, and drop zone |
| Optional side camera | Track rough height and gripper-object vertical alignment |

Do not start with true stereo vision. Stereo adds calibration, rectification, synchronization, depth estimation, and debugging overhead. One overhead camera plus fiducial markers is the shortest path to a reliable demo.

## Vision Plan

Use OpenCV with ArUco markers.

Suggested marker IDs:

| Marker ID | Purpose |
|---:|---|
| 10 | Gripper marker |
| 20 | Object marker |
| 30 | Drop-zone marker |
| 0 to 3 | Optional workspace corner calibration markers |

Mount the gripper marker on a small visible flag near the wrist or gripper. Do not put the only marker directly on the fingertips if the fingers can occlude it.

The first usable coordinate system can be image pixels:

```text
error_xy = target_marker_center_xy - gripper_marker_center_xy
```

The better coordinate system is table millimeters from an overhead homography:

```text
image pixels -> table coordinates in mm
```

Use four fixed corner markers at known table coordinates to compute the homography.

## Control Strategy

Start with image-based visual servoing, not full inverse kinematics.

The robot state for early control should be:

```text
q = [servo_1_us, servo_2_us, servo_3_us, servo_4_us, servo_5_us]
```

The measured state should be:

```text
gripper_xy = ArUco marker center from camera
target_xy = object marker or drop-zone marker center
error_xy = target_xy - gripper_xy
```

Learn a local image Jacobian empirically:

```text
For each servo i:
  move servo i by a small delta, such as +10 us
  measure gripper marker movement in the image
  record dx/dservo_i and dy/dservo_i
```

Then compute small updates with a pseudoinverse:

```python
dq = gain * np.linalg.pinv(J) @ error_xy
```

Clamp every `dq` before sending it:

```text
max_servo_step_us = 10 to 15 us
```

This avoids needing a perfect arm model at the beginning.

## Suggested Initial Config

```yaml
robot:
  name: cheap_hiwonder_pwm_arm
  dof: 4_plus_gripper
  control_mode: usb_serial_to_lsc_pwm_controller
  feedback_mode: webcam_visual_servoing

power:
  servo_bus_voltage_target: 6.0
  current_supply_min_recommended_A: 5
  note: "Use original supply if available. Verify small-servo voltage rating before using >6V."

controller:
  likely_model: Hiwonder LSC-6 or derivative
  board_revision_observed: V1.8
  baud_rate_assumption: 9600
  frame_header: [0x55, 0x55]
  move_command: 0x03
  connection_priority:
    - usb_serial
    - ttl_serial_header
    - bluetooth

servos:
  channel_1:
    mapped_joint: null
    model: unknown
    safe_pwm_us_initial: [1100, 1900]
  channel_2:
    mapped_joint: null
    model: unknown
    safe_pwm_us_initial: [1100, 1900]
  channel_3:
    mapped_joint: null
    model: unknown
    safe_pwm_us_initial: [1100, 1900]
  channel_4:
    mapped_joint: null
    model: unknown
    safe_pwm_us_initial: [1100, 1900]
  channel_5:
    mapped_joint: null
    model: unknown
    safe_pwm_us_initial: [1100, 1900]
  channel_6:
    mapped_joint: spare_or_unused
    model: none
    safe_pwm_us_initial: null

known_servo_models:
  large_servos:
    count: 2
    model: Hiwonder LDX-218
    voltage_range_V: [6.0, 8.4]
    pwm_us_range_nominal: [500, 2500]
  small_servos:
    count: 3
    model: "Hiwonder micro PWM servo, exact model unknown"
    voltage_range_V_assumption: [4.8, 6.0]
    pwm_us_range_nominal: [500, 2500]

camera:
  model: NexiGo N60
  role: overhead_camera
  resolution_target: [1920, 1080]
  fps_target: 30
  format_preference: MJPG
  field_of_view_deg: 110
  focus_type: fixed
  focus_range_in: [15.7, 118]

vision:
  library: opencv
  fiducial_system: aruco
  gripper_marker_id: 10
  object_marker_id: 20
  drop_zone_marker_id: 30
  workspace_corner_marker_ids: [0, 1, 2, 3]

control:
  pixel_tolerance_initial: 12
  max_servo_step_us: 15
  loop_hz_initial: 5
  gain_initial: 0.25
  stop_on_marker_loss: true
```

## Repo Implementation Plan For Coding Agent

### Suggested Files

```text
robot_arm_project/
  README.md
  config.yaml
  requirements.txt
  src/
    hiwonder_lsc.py
    serial_discovery.py
    camera_discovery.py
    aruco_tracking.py
    jog_servos.py
    map_channels.py
    calibrate_camera.py
    calibrate_homography.py
    calibrate_image_jacobian.py
    visual_servo_controller.py
    pick_and_place_state_machine.py
  scripts/
    print_aruco_markers.py
    smoke_test_camera.py
    smoke_test_serial.py
    safe_jog_one_channel.py
```

### Python Dependencies

```text
opencv-contrib-python
numpy
scipy
pyserial
pyyaml
```

Use `opencv-contrib-python`, not only `opencv-python`, because ArUco support is in the contrib package for many OpenCV builds.

## First Tests

### 1. Camera Discovery

Goal: find the NexiGo camera and confirm capture settings.

Linux commands to try:

```bash
v4l2-ctl --list-devices
v4l2-ctl --list-formats-ext -d /dev/video0
```

Python smoke test:

1. Open camera index 0, 1, 2, etc.
2. Try 1920x1080.
3. Try MJPG mode if available.
4. Display frame rate and actual frame shape.
5. Save a sample frame.

Acceptance criteria:

```text
OpenCV can capture stable frames from the N60.
Frame is not blank.
Resolution is known.
Camera index or /dev/video path is recorded.
```

### 2. Serial Discovery

Goal: identify the servo controller serial device.

Commands:

```bash
python -m serial.tools.list_ports
dmesg | tail -50
```

Acceptance criteria:

```text
The controller appears as a serial device, likely /dev/ttyUSB* or /dev/ttyACM*.
The device path is recorded.
```

### 3. Protocol Smoke Test Without Motion Risk

Goal: open the serial port and send only a minimal safe command after manual review.

Do not command all servos to 1500 at startup. On an assembled arm, 1500 us may not be mechanically safe.

First code task:

```python
def build_servo_move_packet(servo_moves, duration_ms):
    """
    servo_moves: list of (servo_id, position)
    duration_ms: int
    Returns Hiwonder LSC packet bytes.
    """
```

Packet builder acceptance criteria:

```text
build_servo_move_packet([(1, 1500)], 1000)
returns:
55 55 08 03 01 e8 03 01 dc 05
```

### 4. Safe One-Channel Jog

Goal: discover channel-to-joint mapping without slamming into mechanical stops.

Procedure:

1. Power servo rail at 6.0 V.
2. Keep one hand near power switch.
3. Start from the current pose, not an assumed neutral.
4. Choose one channel.
5. Move by only +10 to +20 us.
6. Observe which joint moves.
7. Move it back.
8. Record mapping.

Initial safety limits:

```text
large servos: do not exceed 1000 to 2000 us until mapped
small servos: do not exceed 1100 to 1900 us until mapped
step size: 10 to 20 us
duration: 500 to 1000 ms
```

Expected mapping table to fill in:

| Channel | Physical Joint | Servo Model | Safe Min us | Safe Max us | Notes |
|---:|---|---|---:|---:|---|
| 1 | unknown | unknown | 1100 | 1900 | Map by jog |
| 2 | unknown | unknown | 1100 | 1900 | Map by jog |
| 3 | unknown | unknown | 1100 | 1900 | Map by jog |
| 4 | unknown | unknown | 1100 | 1900 | Map by jog |
| 5 | unknown | unknown | 1100 | 1900 | Map by jog |
| 6 | spare | none |  |  | Likely unused |

## Minimum Viable Demo

The first demo should be a tabletop sorting / pick-and-place task.

Task:

```text
1. A tagged object starts anywhere in the visible workspace.
2. The overhead camera detects the object marker and gripper marker.
3. The robot moves the gripper above the object using visual feedback.
4. The robot lowers using a fixed safe pose.
5. The gripper closes.
6. The robot lifts using a fixed safe pose.
7. The robot moves above a tagged drop zone using visual feedback.
8. The robot lowers using a fixed safe pose.
9. The gripper opens.
10. The robot returns home.
```

Initial state machine:

```text
HOME
  -> OPEN_GRIPPER
  -> MOVE_ABOVE_OBJECT
  -> LOWER_TO_GRASP
  -> CLOSE_GRIPPER
  -> LIFT
  -> MOVE_ABOVE_DROP_ZONE
  -> LOWER_TO_DROP
  -> OPEN_GRIPPER
  -> HOME
```

Use vision for horizontal alignment first. Use fixed poses for lowering, lifting, and gripper open/close until the side camera is added.

## Safety Rules

1. Add an emergency stop key before autonomous movement.
2. Stop if the gripper marker is lost.
3. Stop if the object marker is lost during approach.
4. Stop if image error jumps unexpectedly.
5. Clamp every servo command to calibrated limits.
6. Use small incremental moves only.
7. Start with no payload.
8. Keep fingers, glassware, liquids, lab samples, and laptops out of the workspace.
9. Do not power servos from USB.
10. Do not use more than 6 V until small-servo voltage ratings are verified.
11. Do not connect Raspberry Pi GPIO directly to possible 5 V TTL output without level shifting or voltage verification.

## Strong Opinions For The AI Coding Agent

1. Do not begin with full inverse kinematics.
2. Do not begin with stereo vision.
3. Do not send all channels to 1500 us at startup.
4. Do not assume channel numbers match physical joint order.
5. Do not assume the gripper marker is the actual grasp point. Add an offset calibration.
6. Prefer slow, observable, incremental moves.
7. Record all calibration values in `config.yaml`, not scattered constants.
8. Build smoke tests before autonomy.
9. Make the robot stop safely when vision fails.
10. Keep the first demo boring and reliable.

## Project Framing

The useful claim for a writeup or video is:

> A cheap open-loop hobby robot arm can perform simple tabletop manipulation when wrapped in a webcam-based feedback loop, even without joint encoders or a precise kinematic model.

The general lesson:

```text
cheap inaccurate actuator
+ external sensor
+ feedback control
= useful behavior
```

That lesson generalizes to robotics, lab automation, microscope alignment, and low-cost hardware control.

## Source Links

- [Hiwonder LSC-6 servo controller](https://www.hiwonder.com/products/lsc-6)
- [Hiwonder LSC Series Controller Communication Protocol](https://docs.hiwonder.com/projects/16-Channel-Servo-Controller/en/latest/docs/2_Communication_Protocol.html)
- [Hiwonder LDX-218 servo](https://www.hiwonder.com/products/ldx-218)
- [Hiwonder LFD-01M micro servo](https://www.hiwonder.com/products/lfd-01m)
- [NexiGo N60 product page](https://www.nexigo.com/products/nexigo-n60-1080p-webcam-with-microphone)
- [NexiGo N60 technical information](https://www.nexigo.com/pages/n60-technical-information)
- [OpenCV ArUco marker detection](https://docs.opencv.org/4.x/d5/dae/tutorial_aruco_detection.html)
- [OpenCV camera calibration](https://docs.opencv.org/4.x/dc/dbb/tutorial_py_calibration.html)
- [OpenCV perspective transform / homography tutorial](https://docs.opencv.org/4.x/da/d6e/tutorial_py_geometric_transformations.html)
