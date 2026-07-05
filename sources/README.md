# Sources

This folder keeps hardware information separate from implementation code.

The arm described by the user is an Amazon-listed LewanSoul/Hiwonder-style
desktop arm. The updated listing evidence identifies it as `6DOF (5DOF +
Gripper)` with a `6 Channels Servo Controller`, `Bluetooth, USB`, a `7.5 V 3 A
DC` adapter, and listed servo models `LDX-218`, `LD-1501MG`, and `LFD-06`.
LewanSoul and Hiwonder product lines have overlapping branding and controller
hardware, so the repo still treats exact controller protocol, channel mapping,
and safe joint limits as field measurements until verified on the bench.

Files:

- `hardware-notes.md`: current working assumptions and verification tasks.
- `protocol-lobot-servo-controller.md`: provisional serial frame format used by
  the Rust encoder.
- `field-measurements.md`: fill-in sheet for actual arm/camera measurements.
- `stereo-calibration-checklist.md`: optional later stereo calibration workflow.
- `robot_arm_control_project_v2.md`: updated raw hardware handoff from the
  newer listing screenshot and photo analysis.

Keep raw analysis dumps out of this folder once useful facts have been folded
into the files above.
