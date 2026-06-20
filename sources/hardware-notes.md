# Hardware Notes

## Product

- User-reported item: LewanSoul Robotic Arm Kit 6DOF Programming Robot Arm with
  5 Servo, Handle, Mechanical Claw.
- Likely family: LewanSoul/Hiwonder LeArm-style desktop arm.
- Current software assumption: five controlled servos with IDs `1..=5`.

## Assumptions To Verify

- Servo controller protocol: many LewanSoul/Hiwonder/Lobot controller boards use
  binary frames beginning with `55 55`; this repo currently implements that
  common frame shape for servo movement.
- Servo pulse range: provisional safe range is `500..=2500 us`, with narrower
  default limits for some joints in `src/arm.rs`.
- Neutral pulse: provisional neutral is `1500 us`.
- Baud rate and serial device path are not assumed yet.
- Joint IDs, physical axis directions, and hard stops must be measured before
  enabling live serial writes.

## Bench Bring-Up

1. Power the servo controller from the manufacturer-recommended supply.
2. Disconnect the arm linkage or use very conservative joint limits for first
   motion tests.
3. Confirm controller board name, firmware/tooling, serial baud rate, and
   frame protocol.
4. Sweep one servo at a time and record min, neutral, max, positive direction,
   and any mechanical collision zones in `field-measurements.md`.
5. Only after servo IDs and limits are known, add a live serial transport.

## External Research Trail

Search terms used on 2026-06-20:

- `LewanSoul Robotic Arm Kit 6DOF Programming Robot Arm with 5 Servo`
- `LewanSoul LeArm serial protocol`
- `Hiwonder LeArm robotic arm manual`
- `Lobot servo controller 55 55 protocol`

The available search results were not reliable enough to lock exact dimensions
or protocol settings into code. The source files therefore preserve the product
identity and protocol assumptions, but mark real-world values as measurements to
take from the actual kit.
