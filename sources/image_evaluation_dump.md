# Cheap Webcam-Guided Robot Arm Prototype

## Project context

This project uses a low-cost Amazon / Hiwonder-style desktop robot arm as a fast prototype for webcam-guided manipulation. The goal is not industrial precision. The goal is to make a cheap open-loop servo arm do useful tabletop tasks by wrapping it in an external visual feedback loop.

The intended system architecture is:

```
Laptop Python program
    -> USB serial
    -> existing servo controller board
    -> PWM hobby servos
    -> robot arm motion
    -> webcam observes gripper/object
    -> Python sends corrected servo commands
```

This is a quasi-closed-loop robot. The servos themselves do not report true joint angles back to the computer, so the feedback comes from vision.

## Hardware identified from photos

The arm appears to be a 5-servo PWM robot arm.

Observed / inferred hardware:

| Subsystem             | Count | Likely part                                                | Confidence | Notes                                     |
| --------------------- | ----: | ---------------------------------------------------------- | ---------: | ----------------------------------------- |
| Large base/arm servos |     2 | Hiwonder LDX-218                                           |       High | Label is visible in photos                |
| Wrist/gripper servos  |     3 | Hiwonder micro PWM servos, likely LFD-01M-class or similar |     Medium | Exact model not yet readable              |
| Controller board      |     1 | USB/serial PWM servo controller                            |     Medium | Protocol still needs confirmation         |
| Gripper               |     1 | Servo-driven mechanical claw                               |     Medium | Open/closed PWM values must be calibrated |

The two large servos are labeled Hiwonder LDX-218. These are standard PWM servos, not serial-bus smart servos.

Approximate LDX-218 specs:

| Parameter               | Value                                    |
| ----------------------- | ---------------------------------------- |
| Control type            | Standard PWM servo signal                |
| Voltage                 | 6.0-8.4 V                                |
| Nominal angle range     | 0-180 degrees                            |
| Nominal PWM pulse range | 500-2500 us                              |
| PWM period              | about 20 ms                              |
| Torque                  | roughly 15-17 kg-cm depending on voltage |

The three smaller servos should be treated more conservatively until their exact labels are readable. If they are LFD-01M-class micro servos, they are likely intended for about 4.8-6.0 V operation.

Because the small servos may not be rated above 6 V, the safest initial shared servo power rail is:

```
6.0 V, high current
```

A reasonable bench supply target is:

```
6 V, 5-8 A
```

Use the original kit supply if available. Do not run the shared servo rail at 7.4-8.4 V until all small-servo voltage ratings are verified.

## Main project idea

Cheap servo arms are mechanically sloppy: backlash, flex, gear play, weak grippers, and imperfect repeatability are expected. Instead of trying to make the arm precise from the inside, this project adds an external camera feedback loop.

The useful robotics pattern is:

```
cheap inaccurate actuator
+ external sensor
+ feedback control
= useful behavior
```

For the first demo, use vision to move the gripper toward an object or target zone. The robot does not need a perfect kinematic model at the beginning.

## Recommended first vision setup

Start with one overhead webcam.

Overhead camera role:

```
Tracks X/Y table position of gripper and object.
```

Optional second side camera role:

```
Tracks rough height, gripper alignment, and whether the gripper is above/below the object.
```

Do not start with full stereo vision. True stereo adds camera calibration, rectification, synchronization, depth estimation, and debugging overhead. For a fast prototype, one overhead camera plus fiducial markers is much easier.

Recommended marker system:

```
OpenCV ArUco markers
```

Suggested marker IDs:

| Marker ID | Purpose                                       |
| --------: | --------------------------------------------- |
|        10 | Gripper marker                                |
|        20 | Object marker                                 |
|        30 | Drop-zone marker                              |
|       0-3 | Optional workspace corner calibration markers |

Put the gripper marker on a small visible flag attached near the wrist/gripper, not directly on the fingertips. The marker must stay visible to the camera during motion.

## Initial software stack

Use Python.

Recommended packages:

| Package               | Purpose                                    |
| --------------------- | ------------------------------------------ |
| opencv-contrib-python | Webcam capture and ArUco detection         |
| numpy                 | Matrix math                                |
| scipy                 | Optional calibration / least-squares tools |
| pyserial              | USB serial control of servo board          |
| pyyaml                | Config files                               |

Suggested repo layout:

```
robot_arm_project/
  README.md
  main.py
  vision.py
  robot_serial.py
  controller.py
  calibrate_jacobian.py
  config.yaml
  markers/
```

## Suggested config.yaml structure

```
robot:
  name: cheap_hiwonder_pwm_arm
  dof: 4_plus_gripper
  control_mode: usb_serial_to_pwm
  feedback_mode: webcam_visual_servoing

power:
  servo_bus_voltage_target: 6.0
  current_supply_min_recommended_A: 5
  note: "Use original supply if available. Verify small-servo voltage rating before using >6V."

servos:
  base_yaw:
    servo_id: null
    model: Hiwonder LDX-218
    type: standard_pwm_servo
    voltage_range_V: [6.0, 8.4]
    pwm_us_range_nominal: [500, 2500]
    safe_pwm_us_initial: [1000, 2000]
    role: "Base rotation. Confirm channel by jog test."

  shoulder_lift:
    servo_id: null
    model: Hiwonder LDX-218
    type: standard_pwm_servo
    voltage_range_V: [6.0, 8.4]
    pwm_us_range_nominal: [500, 2500]
    safe_pwm_us_initial: [1100, 1900]
    role: "Main lift joint. Confirm channel by jog test."

  joint_3:
    servo_id: null
    model: "Hiwonder micro PWM servo, likely LFD-01M-class"
    type: micro_pwm_servo
    voltage_range_V: [4.8, 6.0]
    pwm_us_range_nominal: [500, 2500]
    safe_pwm_us_initial: [1100, 1900]
    role: "Map by jog test."

  joint_4:
    servo_id: null
    model: "Hiwonder micro PWM servo, likely LFD-01M-class"
    type: micro_pwm_servo
    voltage_range_V: [4.8, 6.0]
    pwm_us_range_nominal: [500, 2500]
    safe_pwm_us_initial: [1100, 1900]
    role: "Map by jog test."

  gripper:
    servo_id: null
    model: "Hiwonder micro PWM servo, likely LFD-01M-class"
    type: micro_pwm_servo
    voltage_range_V: [4.8, 6.0]
    pwm_us_range_nominal: [500, 2500]
    open_pwm_us: null
    closed_pwm_us: null
    role: "Open/close claw. Calibrate manually."

vision:
  primary_camera: overhead
  secondary_camera: optional_side_camera
  fiducial_system: aruco
  gripper_marker_id: 10
  object_marker_id: 20
  drop_zone_marker_id: 30

control:
  pixel_tolerance: 12
  max_servo_step_us: 15
  loop_hz: 5
  gain: 0.25
```

## Servo mapping procedure

Do not command all servos to 1500 us immediately. On an assembled robot arm, 1500 us may not correspond to a mechanically safe pose.

Use a slow jog test:

1. Power the servo rail with a safe 6 V supply.
2. Connect the controller board to the computer.
3. Identify the serial port.
4. Pick one servo channel.
5. Move that channel by a tiny amount, for example +10 to +20 us.
6. Observe which joint moved.
7. Move it back.
8. Record the channel-to-joint mapping.
9. Repeat for all channels.

Start with generic names:

| Temporary name | Later mapped name                    |
| -------------- | ------------------------------------ |
| channel_1      | base_yaw, shoulder_lift, wrist, etc. |
| channel_2      | base_yaw, shoulder_lift, wrist, etc. |
| channel_3      | base_yaw, shoulder_lift, wrist, etc. |
| channel_4      | base_yaw, shoulder_lift, wrist, etc. |
| channel_5      | gripper or wrist                     |

Only rename channels after observing actual motion.

## Visual-servoing approach

Instead of deriving full robot kinematics first, learn how small servo movements affect the gripper marker in the camera image.

For each controllable servo, measure:

```
If servo_i changes by a small amount, how does the gripper marker move in the image?
```

This gives a local image Jacobian:

```
image_motion = J * servo_motion
```

Then use a pseudoinverse to choose small servo updates that reduce visual error:

```
servo_update = gain * pseudoinverse(J) * image_error
```

Where:

```
image_error = target_position_xy - gripper_position_xy
```

This is a practical image-based visual-servoing approach. It is well suited to a cheap robot because it can compensate for some mechanical slop using camera feedback.

## MVP control loop

The first autonomous behavior should be slow and incremental.

Control loop:

1. Capture webcam frame.
2. Detect gripper marker.
3. Detect object marker or target marker.
4. Compute image-space error.
5. If error is below tolerance, stop or advance the state machine.
6. Otherwise compute a small servo correction.
7. Clamp servo correction to safe limits.
8. Send command to servo controller.
9. Wait briefly.
10. Repeat.

The robot should stop if the camera loses the gripper marker or object marker.

## First pick-and-place state machine

For the initial demo, use vision only for horizontal alignment. Use fixed servo poses for vertical approach, grasp, lift, and drop.

State machine:

```
HOME
  -> OPEN_GRIPPER
  -> MOVE_ABOVE_OBJECT using webcam feedback
  -> LOWER_TO_GRASP using fixed servo pose
  -> CLOSE_GRIPPER
  -> LIFT using fixed servo pose
  -> MOVE_ABOVE_DROP_ZONE using webcam feedback
  -> LOWER_TO_DROP using fixed servo pose
  -> OPEN_GRIPPER
  -> HOME
```

This is much easier than full 3D manipulation and is realistic for a cheap hobby arm.

## Development milestones

1. Confirm Python can open the robot controller serial port.
2. Move one servo by a tiny safe increment.
3. Map controller channels to physical joints.
4. Confirm OpenCV can read one webcam.
5. Detect one ArUco marker in live video.
6. Tape marker to gripper and track it.
7. Display gripper marker center live.
8. Manually jog servos and observe marker motion.
9. Learn the local image Jacobian.
10. Implement click-to-move in camera coordinates.
11. Add object marker.
12. Implement simple pick-and-place.
13. Add side camera only after overhead-camera control works.

## Safety rules

1. Add an emergency stop key before autonomous motion.
2. Clamp every servo command to calibrated safe limits.
3. Use small incremental moves only.
4. Stop if the gripper marker is lost.
5. Stop if image error jumps unexpectedly.
6. Keep the arm clear of laptops, glassware, coffee, lab samples, and fingers.
7. Start with no payload.
8. Use slow motion and long servo move durations.
9. Do not rely on USB power for servos.
10. Do not use more than 6 V until all servo voltage ratings are verified.

## Demo framing

The clean demo claim is:

A low-cost hobby robot arm can perform simple tabletop tasks when wrapped in a webcam-based feedback loop, even without accurate joint encoders or a full kinematic model.

Good first demo:

```
Tagged object starts anywhere in the visible workspace.
Robot detects object and gripper.
Robot moves gripper above object.
Robot lowers using a fixed pose.
Robot closes gripper.
Robot lifts object.
Robot moves to a tagged drop zone.
Robot releases object.
```

This is simple, visually understandable, and defensible.

## Sources / references to verify

Hiwonder LDX-218 product page:
https://www.hiwonder.com/products/ldx-218

Hiwonder LFD-01M product page:
https://www.hiwonder.com/products/lfd-01m

OpenCV ArUco marker documentation:
https://docs.opencv.org/4.x/d5/dae/tutorial_aruco_detection.html

OpenCV camera calibration documentation:
https://docs.opencv.org/4.x/dc/dbb/tutorial_py_calibration.html

OpenCV solvePnP documentation:
https://docs.opencv.org/4.x/d5/d1f/calib3d_solvePnP.html
