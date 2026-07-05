# Webcam-Guided Robot Arm Control Project - Updated Hardware Handoff

## What Changed In This Version

This version incorporates the Amazon listing screenshot for the robot arm. That screenshot materially improves the hardware assumptions:

1. The intended power supply is `7.5V 3A DC`.
2. The arm is listed as `6DOF (5DOF + Gripper)`.
3. The listed servo set is `LDX-218, LD-1501MG, LFD-06`.
4. The listed controller is a `6 Channels Servo Controller`.
5. The listed control methods are `PC, APP, Wireless Handle`.
6. Communication is listed as `Bluetooth, USB`.
7. Product size is listed as `285 * 120 * 465 mm`.
8. Product weight is listed as `1.24 kg`.

The biggest practical update is power: the `7.5V 3A DC` adapter is not a weird bin find. It appears to be the intended kit supply. Use it for normal initial testing if polarity and connector fit are correct.

## Project Goal

Use a low-cost Hiwonder / LeArm-style desktop robot arm as a fast prototype for webcam-guided manipulation.

The goal is not industrial-precision robotics. The goal is to show that a cheap open-loop hobby servo arm can perform simple tabletop tasks when wrapped in a camera feedback loop.

System architecture:

```text
Laptop Python program
  -> USB serial or TTL serial
  -> 6-channel Hiwonder-style servo controller
  -> PWM hobby servos
  -> robot arm motion
  -> webcam observes gripper, object, and target
  -> Python sends corrected servo commands
```

The servos should be treated as open-loop position actuators. The closed-loop feedback comes from the camera.

## Assumptions

### High-Confidence Facts From User Photos And Screenshot

| Item | Value | Evidence |
|---|---|---|
| Robot type | Hiwonder / LeArm-style metal desktop arm | Amazon listing screenshot and board/servo photos |
| DOF | `6DOF (5DOF + Gripper)` | Amazon listing screenshot |
| Product dimensions | `285 * 120 * 465 mm` | Amazon listing screenshot |
| Product weight | `1.24 kg` | Amazon listing screenshot |
| Material | Aluminum alloy | Amazon listing screenshot |
| Intended power supply | `7.5V 3A DC Power Adapter` | Amazon listing screenshot |
| Controller | `6 Channels Servo Controller` | Amazon listing screenshot and board photo |
| Communication | Bluetooth, USB | Amazon listing screenshot and board photo |
| Control methods | PC, app, wireless handle | Amazon listing screenshot |
| Large servos | `LDX-218` | Amazon listing screenshot and earlier servo label photo |
| Other servos | `LD-1501MG`, `LFD-06` | Amazon listing screenshot |
| Webcam | NexiGo N60 1080p Full HD Webcam | Webcam box photo |

### Board-Level Facts From Controller Photo

| Board Feature | Observed Detail | Use |
|---|---|---|
| Servo ports | Numbered 1 through 6 | PWM servo outputs |
| Servo pin labels | `S + -` | Signal, servo power, ground |
| TTL serial header | `5V RX TX GND` | External MCU/SBC serial control |
| Power input | Blue screw terminal labeled `+` and `-` | External DC input |
| Alternate power input | Barrel jack | Use only if polarity is verified |
| Switch | `ON` / `OFF` | Board power switch |
| Bluetooth module | Green antenna module marked `www.hc01.com` | App / Bluetooth control path |
| Buzzer | Present | Likely low-voltage or protocol-error alarm |
| Bulk capacitor | `220 uF 16 V` | Input smoothing, not a voltage recommendation |
| Board revision | Appears to read `V1.8` | Medium-confidence board version |

### Inferred But Still Needs Testing

| Item | Working Assumption | Test Needed |
|---|---|---|
| Controller family | Hiwonder LSC-6-style controller | Confirm serial protocol works |
| USB behavior | Enumerates as USB serial | Check `/dev/ttyUSB*` or `/dev/ttyACM*` |
| Serial protocol | Hiwonder LSC protocol at 9600 baud | Send a safe test packet |
| Servo IDs | Probably `1..6` | Jog one channel at a time |
| Camera behavior | NexiGo N60 works as UVC webcam | Open with OpenCV |
| Actual joint order | Unknown | Map by careful jog test |

## Power Supply

### Use This If You Found It

The Amazon listing says the kit supply is:

```text
7.5V 3A DC Power Adapter
```

This should be treated as the correct original supply for this kit, assuming:

1. It is DC output, not AC.
2. The plug fits without force.
3. The polarity matches the controller input.
4. The supply is in good physical condition.

For a barrel plug, verify polarity before use. If polarity is uncertain, use the blue screw terminal instead because it is explicitly labeled `+` and `-`.

### What To Search For In The Bin

Best match:

```text
Output: 7.5V DC
Current: 3A or higher
Polarity: center-positive if using barrel jack, but verify
```

Likely acceptable original adapter markings:

```text
7.5V 3A DC
7.5V 3000mA DC
7.4V 3A DC
```

Avoid:

```text
9V DC
12V DC
Any AC output adapter
Anything with unknown polarity
Anything under 2A
USB-only power for servos
```

### 6V Versus 7.5V Update

Earlier, a conservative shared rail of `6V` was suggested because unidentified micro servos are often 4.8-6.0V parts. The Amazon listing changes that risk assessment.

Updated view:

| Supply | Use Case | Notes |
|---|---|---|
| `7.5V 3A DC` | Correct original kit supply | Best match to listing |
| `7.4V 3A DC` | Probably acceptable if polarity is correct | Close to listed value |
| `6V 5A` | Conservative bench/debug supply | May reduce torque; may trigger low-voltage alarm |
| `5V` | Not recommended | Likely brownouts under servo load |
| `9V` or `12V` | Do not use | Too high for this kit |

Do not try to "make" a 7.5V 3A supply from resistors, diodes, or a small linear regulator. Servo current is spiky. Use the original adapter, a real bench supply, or a properly rated buck converter.

## Servo Inventory

The Amazon screenshot lists:

```text
Servo: LDX-218, LD-1501MG, LFD-06
```

The visual diagram labels:

```text
LDX-218 Large torque digital servo
LDX-218 Large torque digital servo
LD-1501MG Digital servo
LFD-06 Anti-blocking servo
Mechanic gripper
```

Likely servo count:

| Servo Model | Likely Count | Role | Confidence |
|---|---:|---|---:|
| `LDX-218` | 2 | High-torque base / arm joints | High |
| `LD-1501MG` | 1 | Mid/high-torque joint, likely base rotation or wrist/forearm depending build | High |
| `LFD-06` | 2 | Anti-blocking wrist / gripper-related joints | High from screenshot, medium on exact electrical specs |

This explains the `6DOF (5DOF + Gripper)` language: the arm likely has five positioning axes plus one gripper axis, controlled by the six-channel servo controller.

### LDX-218

Official Hiwonder specs for LDX-218:

| Parameter | Value |
|---|---|
| Control | PWM servo |
| Voltage | `6.0-8.4V` |
| Pulse width | `500-2500 us` |
| Period | `20 ms` |
| Angle range | `0-180 deg` |
| Torque | About `15 kg.cm at 6V`, `17 kg.cm at 7.4V` |

Source: [Hiwonder LDX-218](https://www.hiwonder.com/products/ldx-218)

### LD-1501MG

Hiwonder documentation for LD-1501MG lists:

| Parameter | Value |
|---|---|
| Control | PWM |
| Working voltage | `DC 6-8.4V` |
| Pulse width | `500-2500 us` |
| Period | `20 ms` |
| Rotation range | `0-180 deg` |
| Stall current | `2.4-3A` |
| Stall torque | Up to about `17 kg.cm at 7.4V` |

Source: [Hiwonder ROS Robot Control Board docs, LD-1501MG section](https://wiki.hiwonder.com/projects/ROS-Robot-Control-Board/en/latest/docs/3_RosRobot_Controller_Program_Analysis.html)

### LFD-06

The Amazon screenshot identifies the gripper-side anti-blocking servo as `LFD-06`. I found less clean official standalone documentation for LFD-06 than for LDX-218 and LD-1501MG, so treat detailed LFD-06 specs as lower confidence.

Useful sourced values from Hiwonder-related product pages:

| Parameter | Value | Confidence |
|---|---|---:|
| Servo family | LFD anti-blocking servo | High |
| Used in Hiwonder hand/arm products | Yes | High |
| Approximate size | `40 * 20 * 40.5 mm` | Medium |
| Stall torque reference | `6 kg.cm at 6.6V` | Medium |

Sources: [Hiwonder uHandPi listing via reseller](https://pronetkrakow.com.pl/en/robots-ai/1068-hiwonder-uhandpi.html), [Hiwonder uHand UNO listing](https://www.hiwonder.com/products/uhand-uno?variant=40525009879127)

Practical conclusion: because the arm listing pairs `LFD-06` with the `7.5V 3A` adapter, using the original 7.5V supply is reasonable. Still, during first tests, watch the LFD-06 servos for heat, buzzing, or hard stalls.

## Controller Protocol

The controller is likely compatible with the Hiwonder LSC-series serial protocol.

From Hiwonder LSC-series docs:

| Field | Value |
|---|---|
| Baud rate | `9600` |
| Frame header | `0x55 0x55` |
| Packet format | `0x55 0x55 LEN CMD PARAMS...` |
| Servo move command | `CMD = 0x03` |
| Length formula | `LEN = parameter_bytes + 2` |

Source: [Hiwonder LSC Series Controller Communication Protocol](https://docs.hiwonder.com/projects/16-Channel-Servo-Controller/en/latest/docs/2_Communication_Protocol.html)

Example: move servo ID 1 to position 1500 over 1000 ms.

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

For multiple servos:

```text
0x55 0x55 LEN 0x03 COUNT TIME_L TIME_H ID1 POS1_L POS1_H ID2 POS2_L POS2_H ...
```

Length:

```text
LEN = COUNT * 3 + 5
```

## Webcam

The webcam box photo identifies the camera as:

```text
NexiGo N60 1080p Full HD Webcam
```

Official NexiGo N60 specs:

| Parameter | Value |
|---|---|
| Resolution | `1920x1080` |
| Frame rate | `30 FPS` |
| Sensor | `2MP CMOS` |
| Field of view | `110 deg` |
| Focus | Fixed focus |
| Focus range | `15.7 in - 118 in` |
| Video formats | `YUY2 / MJPG` |
| Tripod mount | `1/4 in compatible` |

Sources: [NexiGo N60 product page](https://www.nexigo.com/products/nexigo-n60-1080p-webcam-with-microphone), [NexiGo N60 technical information](https://www.nexigo.com/pages/n60-technical-information)

Camera implication: the wide 110 degree field of view is useful for seeing the whole workspace, but it will introduce distortion. For first demos, raw pixel feedback is fine. For cleaner control, calibrate the camera or use a table homography.

## Updated Config Skeleton

```yaml
robot:
  name: hiwonder_learm_style_webcam_guided_arm
  dof_label_from_listing: "6DOF (5DOF + Gripper)"
  material: aluminum_alloy
  product_dimensions_mm: [285, 120, 465]
  product_weight_kg: 1.24
  control_mode: usb_serial_to_6_channel_servo_controller
  feedback_mode: webcam_visual_servoing

power:
  original_adapter:
    voltage_V: 7.5
    current_A: 3.0
    type: DC
    source: amazon_listing_screenshot
  bench_debug_alternative:
    voltage_V: 6.0
    current_A_min: 5.0
    note: "May reduce torque or trigger low-voltage alarm."
  forbidden:
    - "9V adapter"
    - "12V adapter"
    - "AC output adapter"
    - "USB-only servo power"

controller:
  likely_model_family: Hiwonder LSC-6 style
  channels: 6
  communication_from_listing:
    - USB
    - Bluetooth
  control_methods_from_listing:
    - PC
    - APP
    - Wireless Handle
  ttl_header_observed: ["5V", "RX", "TX", "GND"]
  baud_rate_assumption: 9600
  frame_header: [0x55, 0x55]
  move_command: 0x03

servos:
  listed_models:
    - LDX-218
    - LD-1501MG
    - LFD-06
  expected_channels:
    1:
      mapped_joint: null
      model: unknown_until_jog_test
      safe_pwm_us_initial: [1100, 1900]
    2:
      mapped_joint: null
      model: unknown_until_jog_test
      safe_pwm_us_initial: [1100, 1900]
    3:
      mapped_joint: null
      model: unknown_until_jog_test
      safe_pwm_us_initial: [1100, 1900]
    4:
      mapped_joint: null
      model: unknown_until_jog_test
      safe_pwm_us_initial: [1100, 1900]
    5:
      mapped_joint: null
      model: unknown_until_jog_test
      safe_pwm_us_initial: [1100, 1900]
    6:
      mapped_joint: null
      model: unknown_until_jog_test
      safe_pwm_us_initial: [1100, 1900]

camera:
  model: NexiGo N60
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

## First Tests For The Coding Agent

### 1. Power Check

Before plugging in:

1. Confirm adapter label says DC output.
2. Confirm voltage is approximately 7.5V.
3. Confirm current rating is at least 3A.
4. Confirm barrel polarity if using barrel jack.
5. If polarity is uncertain, use the blue screw terminal and a multimeter.

Acceptance criterion:

```text
The board powers on from the intended 7.5V 3A DC adapter without servo motion commands.
No smoke, smell, excessive heat, or repeated alarm condition.
```

### 2. USB Serial Discovery

Commands:

```bash
python -m serial.tools.list_ports
dmesg | tail -50
```

Acceptance criterion:

```text
The controller appears as a serial device, likely /dev/ttyUSB* or /dev/ttyACM*.
```

### 3. Packet Builder Test

Implement:

```python
def build_servo_move_packet(servo_moves, duration_ms):
    """
    servo_moves: list of (servo_id, position)
    duration_ms: int
    Returns Hiwonder LSC packet bytes.
    """
```

Unit test:

```text
build_servo_move_packet([(1, 1500)], 1000)
should return:
55 55 08 03 01 e8 03 01 dc 05
```

### 4. Safe Channel Mapping

Do not send all servos to 1500 us. On an assembled robot, 1500 us may not be a safe neutral pose.

Procedure:

1. Power with the original `7.5V 3A DC` adapter.
2. Keep one hand near the power switch.
3. Select one channel.
4. Move only `+10` to `+20 us`.
5. Observe which joint moves.
6. Move it back.
7. Record channel, joint, direction, and any mechanical limits.

Initial jog limits:

```text
Command range: 1100 to 1900 us
Step size: 10 to 20 us
Move duration: 500 to 1000 ms
```

Mapping table to fill:

| Channel | Joint | Servo Model | Safe Min us | Safe Max us | Notes |
|---:|---|---|---:|---:|---|
| 1 | unknown | unknown | 1100 | 1900 | map by jog |
| 2 | unknown | unknown | 1100 | 1900 | map by jog |
| 3 | unknown | unknown | 1100 | 1900 | map by jog |
| 4 | unknown | unknown | 1100 | 1900 | map by jog |
| 5 | unknown | unknown | 1100 | 1900 | map by jog |
| 6 | unknown | unknown | 1100 | 1900 | map by jog |

### 5. Camera Test

Linux commands:

```bash
v4l2-ctl --list-devices
v4l2-ctl --list-formats-ext -d /dev/video0
```

Acceptance criterion:

```text
OpenCV can capture nonblank frames from the NexiGo N60.
The actual resolution, frame rate, and camera index are recorded.
```

### 6. ArUco Test

1. Generate or print ArUco markers.
2. Detect one marker in live video.
3. Draw marker outline and center.
4. Log marker ID and pixel coordinates.

Suggested marker IDs:

| Marker ID | Use |
|---:|---|
| 10 | Gripper |
| 20 | Object |
| 30 | Drop zone |
| 0 to 3 | Workspace corners |

## Visual Servoing Plan

Start with one overhead camera.

Do not start with full inverse kinematics or stereo vision. The shortest useful path is image-based visual servoing.

Measured state:

```text
gripper_xy = marker center from camera
target_xy = object or drop-zone marker center
error_xy = target_xy - gripper_xy
```

Actuator state:

```text
q = [servo_1_us, servo_2_us, servo_3_us, servo_4_us, servo_5_us, servo_6_us]
```

Empirically learn a local image Jacobian:

```text
For each servo:
  move by a small delta
  measure gripper marker dx, dy
  record the image-space effect of that servo
```

Then compute small corrections:

```python
dq = gain * np.linalg.pinv(J) @ error_xy
```

Clamp every update before sending:

```text
max_servo_step_us = 10 to 15 us
```

## Minimum Viable Demo

The first demo should be tabletop pick-and-place:

```text
HOME
  -> OPEN_GRIPPER
  -> MOVE_ABOVE_OBJECT using overhead webcam feedback
  -> LOWER_TO_GRASP using fixed safe pose
  -> CLOSE_GRIPPER
  -> LIFT using fixed safe pose
  -> MOVE_ABOVE_DROP_ZONE using overhead webcam feedback
  -> LOWER_TO_DROP using fixed safe pose
  -> OPEN_GRIPPER
  -> HOME
```

Use vision for horizontal alignment first. Use fixed poses for lowering, lifting, and gripper open/close. Add a side camera only after overhead control works.

## Safety Rules

1. Use the original `7.5V 3A DC` adapter if available and verified.
2. Do not use 9V or 12V adapters.
3. Do not use any AC-output adapter.
4. Do not power servos from USB.
5. Do not power servos from the `5V RX TX GND` header.
6. Verify polarity before using the barrel jack.
7. Add an emergency stop key before autonomous motion.
8. Stop if the gripper marker is lost.
9. Stop if image error jumps unexpectedly.
10. Clamp every servo command to calibrated limits.
11. Use small incremental moves only.
12. Start with no payload.
13. Keep fingers, liquids, glassware, lab samples, and laptops out of the workspace.
14. If any servo gets hot, buzzes hard, smells bad, or stalls mechanically, stop.
15. If using Raspberry Pi UART later, level-shift or verify voltage before connecting board TX to Pi RX.

## Source Links

- Amazon listing screenshot supplied by user, captured 2026-07-04.
- [Hiwonder LDX-218 servo](https://www.hiwonder.com/products/ldx-218)
- [Hiwonder LD-1501MG documentation](https://wiki.hiwonder.com/projects/ROS-Robot-Control-Board/en/latest/docs/3_RosRobot_Controller_Program_Analysis.html)
- [Hiwonder LSC-6 servo controller](https://www.hiwonder.com/products/lsc-6)
- [Hiwonder LSC Series Controller Communication Protocol](https://docs.hiwonder.com/projects/16-Channel-Servo-Controller/en/latest/docs/2_Communication_Protocol.html)
- [Hiwonder LeArm AI listing with related servo set](https://www.hiwonder.com/products/learm-ai)
- [Hiwonder uHand UNO listing mentioning LFD-06](https://www.hiwonder.com/products/uhand-uno?variant=40525009879127)
- [NexiGo N60 product page](https://www.nexigo.com/products/nexigo-n60-1080p-webcam-with-microphone)
- [NexiGo N60 technical information](https://www.nexigo.com/pages/n60-technical-information)
- [OpenCV ArUco marker detection](https://docs.opencv.org/4.x/d5/dae/tutorial_aruco_detection.html)
- [OpenCV camera calibration](https://docs.opencv.org/4.x/dc/dbb/tutorial_py_calibration.html)
- [OpenCV perspective transforms](https://docs.opencv.org/4.x/da/d6e/tutorial_py_geometric_transformations.html)

