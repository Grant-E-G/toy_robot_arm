# Field Measurements

Fill this file in from the actual hardware.

## Arm

| Item | Value | Notes |
| --- | --- | --- |
| Controller board marking | TBD | |
| Large servo model | Hiwonder LDX-218 | Visible in photos; confirm on hardware |
| Small servo model | TBD | Photo suggests Hiwonder micro PWM, possibly LFD-01M-class |
| Serial device path | TBD | Linux example: `/dev/ttyUSB0` |
| Baud rate | TBD | |
| Power supply voltage/current | TBD | |
| Base-to-shoulder link length | TBD | meters |
| Shoulder-to-elbow link length | TBD | meters |
| Elbow-to-wrist link length | TBD | meters |
| Wrist-to-claw tip length | TBD | meters |

## Joint Limits

| ID | Joint | Min pulse | Neutral pulse | Max pulse | Positive direction | Notes |
| --- | --- | ---: | ---: | ---: | --- | --- |
| 1 | TBD | TBD | TBD | TBD | TBD | Map by `10..=20 us` jog test |
| 2 | TBD | TBD | TBD | TBD | TBD | Map by `10..=20 us` jog test |
| 3 | TBD | TBD | TBD | TBD | TBD | Map by `10..=20 us` jog test |
| 4 | TBD | TBD | TBD | TBD | TBD | Map by `10..=20 us` jog test |
| 5 | TBD | TBD | TBD | TBD | TBD | Map by `10..=20 us` jog test |

## Gripper

| Item | Value | Notes |
| --- | --- | --- |
| Open pulse | TBD | Calibrate manually |
| Closed empty pulse | TBD | Calibrate manually |
| Closed on test object pulse | TBD | Calibrate manually; avoid stall |

## Cameras

| Item | Left camera | Right camera | Notes |
| --- | --- | --- | --- |
| Device path | TBD | TBD | |
| Resolution | TBD | TBD | |
| FPS | TBD | TBD | |
| Focal length x/y | TBD | TBD | pixels after calibration |
| Principal point x/y | TBD | TBD | pixels after calibration |
| Distortion coefficients | TBD | TBD | |
| Baseline | TBD | TBD | fixed measured distance, meters |

## Fiducials

| Marker ID | Purpose | Notes |
| ---: | --- | --- |
| 10 | Gripper | Mount on a visible wrist/gripper flag |
| 20 | Object | |
| 30 | Drop zone | |
| 0-3 | Workspace corners | Optional overhead-camera calibration |
