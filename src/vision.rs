#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PixelPoint {
    pub x: f64,
    pub y: f64,
}

impl PixelPoint {
    pub const fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point3 {
    pub x_m: f64,
    pub y_m: f64,
    pub z_m: f64,
}

impl Point3 {
    pub const fn new(x_m: f64, y_m: f64, z_m: f64) -> Self {
        Self { x_m, y_m, z_m }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StereoRig {
    pub baseline_m: f64,
    pub focal_px: f64,
    pub cx_px: f64,
    pub cy_px: f64,
}

impl StereoRig {
    pub fn triangulate_disparity(
        self,
        left: PixelPoint,
        right: PixelPoint,
    ) -> Result<Point3, StereoError> {
        validate_positive("baseline_m", self.baseline_m)?;
        validate_positive("focal_px", self.focal_px)?;

        let disparity_px = left.x - right.x;
        if disparity_px <= 0.0 {
            return Err(StereoError::NonPositiveDisparity(disparity_px));
        }

        let z_m = self.focal_px * self.baseline_m / disparity_px;
        let x_m = (left.x - self.cx_px) * z_m / self.focal_px;
        let y_m = (left.y - self.cy_px) * z_m / self.focal_px;

        Ok(Point3 { x_m, y_m, z_m })
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum StereoError {
    InvalidPositiveParameter(&'static str),
    NonPositiveDisparity(f64),
}

fn validate_positive(name: &'static str, value: f64) -> Result<(), StereoError> {
    (value.is_finite() && value > 0.0)
        .then_some(())
        .ok_or(StereoError::InvalidPositiveParameter(name))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn triangulates_depth_from_disparity() {
        let rig = StereoRig {
            baseline_m: 0.12,
            focal_px: 700.0,
            cx_px: 320.0,
            cy_px: 240.0,
        };

        let point = rig
            .triangulate_disparity(PixelPoint::new(350.0, 250.0), PixelPoint::new(310.0, 250.0))
            .unwrap();

        assert!((point.z_m - 2.1).abs() < 1e-9);
        assert!((point.x_m - 0.09).abs() < 1e-9);
        assert!((point.y_m - 0.03).abs() < 1e-9);
    }

    #[test]
    fn rejects_non_positive_disparity() {
        let rig = StereoRig {
            baseline_m: 0.12,
            focal_px: 700.0,
            cx_px: 320.0,
            cy_px: 240.0,
        };

        assert_eq!(
            rig.triangulate_disparity(PixelPoint::new(10.0, 0.0), PixelPoint::new(10.0, 0.0)),
            Err(StereoError::NonPositiveDisparity(0.0))
        );
    }
}
