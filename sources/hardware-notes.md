# Hardware Notes

## Product

- User-reported item: LewanSoul Robotic Arm Kit 6DOF Programming Robot Arm with
  5 Servo, Handle, Mechanical Claw.
- Likely family: LewanSoul/Hiwonder LeArm-style desktop arm.
- Updated listing evidence: `6DOF (5DOF + Gripper)`, `6 Channels Servo
  Controller`, `Bluetooth, USB`, product size `285 * 120 * 465 mm`, and product
  weight `1.24 kg`.
- Current software assumption: six controller channels with IDs `1..=6`, all to
  be mapped by safe jog testing.

## Photo-Derived Hardware Notes

- The arm appears to use standard PWM servos, not serial-bus smart servos.
- The updated listing names the servo set as Hiwonder `LDX-218`, `LD-1501MG`,
  and `LFD-06`.
- Two large base/arm servos are visibly labeled Hiwonder `LDX-218`.
- The controller appears to be a USB/serial PWM servo controller with six PWM
  outputs; protocol still needs bench verification.
- The gripper is a servo-driven mechanical claw. Record open and closed pulse
  limits before attempting autonomous grasping.
- The intended kit supply appears to be a `7.5 V DC, 3 A` adapter.

## Assumptions To Verify

- Servo controller protocol: many LewanSoul/Hiwonder/Lobot controller boards use
  binary frames beginning with `55 55`; this repo currently implements that
  common frame shape for servo movement.
- Servo pulse range: `500..=2500 us` is the nominal/datasheet-level range for
  known PWM servos. The built-in startup spec uses `1100..=1900 us` for every
  controller channel until measured limits are recorded.
- Neutral pulse: provisional neutral is `1500 us`.
- LDX-218 servos are expected to accept standard PWM, roughly `500..=2500 us`
  over a nominal `0..=180` degree range. Treat those as datasheet-level values,
  not calibrated robot joint limits.
- LDX-218 and LD-1501MG are expected around `6.0..=8.4 V`; detailed LFD-06
  specs remain lower confidence, but the kit listing pairs it with the `7.5 V`
  adapter.
- Use the original `7.5 V DC, 3 A` supply for normal initial testing if the
  label, connector fit, and polarity are verified. A current-capable `6 V`
  bench supply remains a conservative debug option but may reduce torque or
  trigger a low-voltage alarm.
- Baud rate is assumed to be `9600` for the LSC-style protocol; serial device
  path is still a field measurement.
- Joint IDs, physical axis directions, and hard stops must be measured before
  enabling live serial writes.

## Bench Bring-Up

1. Verify the adapter is DC, approximately `7.5 V`, rated at least `3 A`, and
   has correct polarity before powering the controller.
2. Power the servo controller from the verified original supply, or a
   current-limited `6 V` bench supply for cautious debugging.
3. Disconnect the arm linkage or use very conservative joint limits for first
   motion tests.
4. Confirm controller board name, firmware/tooling, serial baud rate, and
   frame protocol.
5. Jog one channel at a time by only `10..=20 us` from the current pose, then
   return it before testing the next channel.
6. Record channel-to-joint mapping, min, neutral, max, positive direction, and
   any mechanical collision zones in `field-measurements.md`.
7. Only after servo IDs and limits are known, add a live serial transport.

## Vision Bring-Up

Start with one overhead camera and fiducial markers. Use it for X/Y alignment
of the gripper, object, and drop zone. Add a side camera or stereo depth only
after overhead tracking and safe incremental motion work.

Suggested marker IDs:

| Marker ID | Purpose |
| ---: | --- |
| 10 | Gripper marker |
| 20 | Object marker |
| 30 | Drop-zone marker |
| 0-3 | Optional workspace corner calibration markers |

Put the gripper marker on a small visible flag near the wrist/gripper rather
than on the fingertips, so the camera can still see it during grasping.

## External Research Trail

Search terms used on 2026-06-20:

- `LewanSoul Robotic Arm Kit 6DOF Programming Robot Arm with 5 Servo`
- `LewanSoul LeArm serial protocol`
- `Hiwonder LeArm robotic arm manual`
- `Lobot servo controller 55 55 protocol`
- `Hiwonder LDX-218`
- `Hiwonder LFD-01M`

Updated source added on 2026-07-04:

- Amazon listing screenshot from user, indicating `7.5 V 3 A DC`, `6DOF (5DOF
  + Gripper)`, `6 Channels Servo Controller`, and servo models `LDX-218`,
  `LD-1501MG`, `LFD-06`.

The available search results were not reliable enough to lock exact dimensions
or protocol settings into code. The source files therefore preserve the product
identity and protocol assumptions, but mark real-world values as measurements to
take from the actual kit.
