# Sources

This folder keeps hardware information separate from implementation code.

The arm described by the user is an Amazon-listed "LewanSoul Robotic Arm Kit
6DOF Programming Robot Arm with 5 Servo, Handle, Mechanical Claw". LewanSoul
and Hiwonder product lines have overlapping branding and controller hardware,
so the repo treats the exact controller protocol and servo mapping as field
measurements until verified on the bench.

Files:

- `hardware-notes.md`: current working assumptions and verification tasks.
- `protocol-lobot-servo-controller.md`: provisional serial frame format used by
  the Rust encoder.
- `field-measurements.md`: fill-in sheet for actual arm/camera measurements.
- `stereo-calibration-checklist.md`: camera calibration workflow.
