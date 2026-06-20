# Stereo Calibration Checklist

1. Mount both webcams rigidly so their relative position cannot shift.
2. Measure the camera baseline between optical centers as closely as practical.
3. Capture checkerboard images from both cameras at the same time.
4. Estimate intrinsics for each camera.
5. Estimate stereo extrinsics and rectification.
6. Save calibrated focal length, principal point, baseline, and distortion.
7. Test with a target at known distances and compare measured depth error.
8. Use the calibrated values in `StereoRig` only after the reprojection error is
   acceptable for the task.

The current Rust `StereoRig` assumes rectified images and computes depth from
horizontal disparity:

```text
z = focal_px * baseline_m / disparity_px
x = (left_x - cx_px) * z / focal_px
y = (left_y - cy_px) * z / focal_px
```
