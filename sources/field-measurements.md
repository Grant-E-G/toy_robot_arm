# Field Measurements

These values are populated from the photo-derived project notes where possible.
Treat every photo-derived or inferred value as provisional until checked on the
actual hardware.

## Arm

| Item | Value | Notes |
| --- | --- | --- |
| Controller board marking | Hiwonder LSC-6-style controller; possible `V1.8` revision | Photo-derived; board appears to have 6 PWM outputs, USB, Bluetooth, and `5V RX TX GND` TTL header |
| Large servo model | Hiwonder LDX-218 | Visible in photos; confirm on hardware |
| Small servo model | Hiwonder micro PWM servo, exact model TBD | Photo suggests LFD-01M-class; keep power conservative until label is confirmed |
| Serial device path | TBD | Discover with `python -m serial.tools.list_ports` or `dmesg`; likely `/dev/ttyUSB*` or `/dev/ttyACM*` |
| Baud rate | 9600 | Inferred from Hiwonder LSC-series docs; verify with this board |
| Power supply voltage/current | 6.0 V, 5-8 A initial target | Use original supply if available; do not exceed 6 V until small-servo voltage rating is confirmed |
| Base-to-shoulder link length | TBD | meters |
| Shoulder-to-elbow link length | TBD | meters |
| Elbow-to-wrist link length | TBD | meters |
| Wrist-to-claw tip length | TBD | meters |

## Joint Limits

| ID | Joint | Min pulse | Neutral pulse | Max pulse | Positive direction | Notes |
| --- | --- | ---: | ---: | ---: | --- | --- |
| 1 | TBD | 1100 | 1500 | 1900 | TBD | Initial jog-only range from photo-derived notes; not a calibrated hard-stop range |
| 2 | TBD | 1100 | 1500 | 1900 | TBD | Initial jog-only range from photo-derived notes; not a calibrated hard-stop range |
| 3 | TBD | 1100 | 1500 | 1900 | TBD | Initial jog-only range from photo-derived notes; not a calibrated hard-stop range |
| 4 | TBD | 1100 | 1500 | 1900 | TBD | Initial jog-only range from photo-derived notes; not a calibrated hard-stop range |
| 5 | TBD | 1100 | 1500 | 1900 | TBD | Initial jog-only range from photo-derived notes; not a calibrated hard-stop range |
| 6 | Spare/unused | TBD | TBD | TBD | TBD | Controller has 6 outputs; arm is expected to use 5 servos |

## Gripper

| Item | Value | Notes |
| --- | --- | --- |
| Open pulse | TBD | Calibrate manually after channel-to-joint mapping |
| Closed empty pulse | TBD | Calibrate manually after channel-to-joint mapping |
| Closed on test object pulse | TBD | Calibrate manually; avoid stall |

## Cameras

| Item | Overhead camera | Side/stereo camera | Notes |
| --- | --- | --- | --- |
| Device path | TBD | N/A | Start with one overhead NexiGo N60; discover `/dev/video*` before use |
| Resolution | 1920x1080 | N/A | NexiGo N60 target setting from photo-derived notes/product info; verify actual capture mode |
| FPS | 30 | N/A | NexiGo N60 target setting; prefer MJPG if raw YUY2 bandwidth is too high |
| Focal length x/y | TBD | N/A | pixels after calibration |
| Principal point x/y | TBD | N/A | pixels after calibration |
| Distortion coefficients | TBD | N/A | |
| Baseline | N/A | N/A | Single overhead camera first; measure only if a stereo/side-camera setup is added |

## Fiducials

| Marker ID | Purpose | Notes |
| ---: | --- | --- |
| 10 | Gripper | Mount on a visible wrist/gripper flag |
| 20 | Object | |
| 30 | Drop zone | |
| 0-3 | Workspace corners | Optional overhead-camera calibration |
