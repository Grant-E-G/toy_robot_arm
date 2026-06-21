# Hardware Notes

## Product

- User-reported item: LewanSoul Robotic Arm Kit 6DOF Programming Robot Arm with
  5 Servo, Handle, Mechanical Claw.
- Likely family: LewanSoul/Hiwonder LeArm-style desktop arm.
- Current software assumption: five controlled servos with IDs `1..=5`.

## Photo-Derived Hardware Notes

- The arm appears to use five standard PWM servos, not serial-bus smart servos.
- Two large base/arm servos are visibly labeled Hiwonder `LDX-218`.
- The three smaller wrist/gripper servos look like Hiwonder micro PWM servos,
  possibly `LFD-01M`-class, but their exact labels are not readable yet.
- The controller appears to be a USB/serial PWM servo controller; protocol still
  needs bench verification.
- The gripper is a servo-driven mechanical claw. Record open and closed pulse
  limits before attempting autonomous grasping.

## Assumptions To Verify

- Servo controller protocol: many LewanSoul/Hiwonder/Lobot controller boards use
  binary frames beginning with `55 55`; this repo currently implements that
  common frame shape for servo movement.
- Servo pulse range: provisional safe range is `500..=2500 us`, with narrower
  default limits for some joints in `src/arm.rs`.
- Neutral pulse: provisional neutral is `1500 us`.
- LDX-218 servos are expected to accept standard PWM, roughly `500..=2500 us`
  over a nominal `0..=180` degree range. Treat those as datasheet-level values,
  not calibrated robot joint limits.
- LDX-218 voltage is expected around `6.0..=8.4 V`, but the smaller servos may
  only be rated for about `4.8..=6.0 V`.
- Use a shared servo rail of `6 V` for initial bench work unless the exact
  small-servo model proves a higher voltage is safe. A `5..=8 A` supply is a
  reasonable starting target.
- Baud rate and serial device path are not assumed yet.
- Joint IDs, physical axis directions, and hard stops must be measured before
  enabling live serial writes.

## Bench Bring-Up

1. Power the servo controller from the manufacturer-recommended supply, or a
   current-limited `6 V` bench supply.
2. Disconnect the arm linkage or use very conservative joint limits for first
   motion tests.
3. Confirm controller board name, firmware/tooling, serial baud rate, and
   frame protocol.
4. Jog one channel at a time by only `10..=20 us` from the current pose, then
   return it before testing the next channel.
5. Record channel-to-joint mapping, min, neutral, max, positive direction, and
   any mechanical collision zones in `field-measurements.md`.
6. Only after servo IDs and limits are known, add a live serial transport.

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

The available search results were not reliable enough to lock exact dimensions
or protocol settings into code. The source files therefore preserve the product
identity and protocol assumptions, but mark real-world values as measurements to
take from the actual kit.
