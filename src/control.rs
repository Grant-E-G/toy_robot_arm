use crate::arm::{ArmPose, JointCommand, JointId, Pulse, RobotArmSpec};
use crate::vision::Point3;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VisualServoGoal {
    pub target: Point3,
    pub tolerance_m: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ControllerGains {
    pub base_pulse_per_m: f64,
    pub shoulder_pulse_per_m: f64,
    pub elbow_pulse_per_m: f64,
    pub wrist_pulse_per_m: f64,
    pub max_delta_pulse: i32,
}

impl Default for ControllerGains {
    fn default() -> Self {
        Self {
            base_pulse_per_m: 450.0,
            shoulder_pulse_per_m: -650.0,
            elbow_pulse_per_m: 500.0,
            wrist_pulse_per_m: -180.0,
            max_delta_pulse: 40,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ControllerStep {
    pub next_pose_reached_goal: bool,
    pub error: Point3,
}

pub fn step_visual_servo(
    spec: &RobotArmSpec,
    current_pose: &ArmPose,
    observed_tool: Point3,
    goal: VisualServoGoal,
    gains: ControllerGains,
) -> (ArmPose, ControllerStep) {
    let error = Point3 {
        x_m: goal.target.x_m - observed_tool.x_m,
        y_m: goal.target.y_m - observed_tool.y_m,
        z_m: goal.target.z_m - observed_tool.z_m,
    };

    let reached = distance_m(error) <= goal.tolerance_m;
    if reached {
        return (
            spec.clamp_pose(current_pose),
            ControllerStep {
                next_pose_reached_goal: true,
                error,
            },
        );
    }

    let deltas = [
        (JointId(1), error.x_m * gains.base_pulse_per_m),
        (JointId(2), error.z_m * gains.shoulder_pulse_per_m),
        (JointId(3), error.z_m * gains.elbow_pulse_per_m),
        (JointId(4), error.y_m * gains.wrist_pulse_per_m),
    ];

    let moved = current_pose.map_pulses(|command| {
        let delta = deltas
            .iter()
            .find(|(id, _)| *id == command.id)
            .map(|(_, delta)| limited_delta(*delta, gains.max_delta_pulse))
            .unwrap_or(0);

        JointCommand {
            id: command.id,
            pulse: add_delta(command.pulse, delta),
        }
    });

    (
        spec.clamp_pose(&moved),
        ControllerStep {
            next_pose_reached_goal: reached,
            error,
        },
    )
}

fn add_delta(pulse: Pulse, delta: i32) -> Pulse {
    Pulse((pulse.0 as i32 + delta).clamp(0, u16::MAX as i32) as u16)
}

fn limited_delta(delta: f64, max_abs: i32) -> i32 {
    let max_abs = max_abs.max(0) as f64;
    delta.round().clamp(-max_abs, max_abs) as i32
}

fn distance_m(point: Point3) -> f64 {
    (point.x_m.powi(2) + point.y_m.powi(2) + point.z_m.powi(2)).sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn controller_moves_within_rate_limit() {
        let spec = RobotArmSpec::lewansoul_learm_provisional();
        let pose = spec.neutral_pose();

        let (next, step) = step_visual_servo(
            &spec,
            &pose,
            Point3::new(0.0, 0.0, 0.6),
            VisualServoGoal {
                target: Point3::new(0.5, 0.0, 0.4),
                tolerance_m: 0.01,
            },
            ControllerGains::default(),
        );

        assert!(!step.next_pose_reached_goal);
        assert_eq!(next.get(JointId(1)), Some(Pulse(1540)));
        assert_eq!(next.get(JointId(2)), Some(Pulse(1540)));
        assert_eq!(next.get(JointId(3)), Some(Pulse(1460)));
    }

    #[test]
    fn controller_holds_pose_inside_tolerance() {
        let spec = RobotArmSpec::lewansoul_learm_provisional();
        let pose = spec.neutral_pose();

        let (next, step) = step_visual_servo(
            &spec,
            &pose,
            Point3::new(0.0, 0.0, 0.0),
            VisualServoGoal {
                target: Point3::new(0.001, 0.001, 0.001),
                tolerance_m: 0.01,
            },
            ControllerGains::default(),
        );

        assert!(step.next_pose_reached_goal);
        assert_eq!(next, pose);
    }
}
