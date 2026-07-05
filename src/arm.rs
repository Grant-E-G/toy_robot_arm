use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct JointId(pub u8);

impl fmt::Display for JointId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Pulse(pub u16);

impl Pulse {
    pub const fn new(value: u16) -> Self {
        Self(value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct JointLimit {
    pub id: JointId,
    pub min: Pulse,
    pub neutral: Pulse,
    pub max: Pulse,
}

impl JointLimit {
    pub fn contains(self, pulse: Pulse) -> bool {
        self.min.0 <= pulse.0 && pulse.0 <= self.max.0
    }

    pub fn clamp(self, pulse: Pulse) -> Pulse {
        Pulse(pulse.0.clamp(self.min.0, self.max.0))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct JointCommand {
    pub id: JointId,
    pub pulse: Pulse,
}

impl JointCommand {
    pub const fn new(id: u8, pulse: u16) -> Self {
        Self {
            id: JointId(id),
            pulse: Pulse(pulse),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArmPose {
    commands: Vec<JointCommand>,
}

impl ArmPose {
    pub fn new(commands: Vec<JointCommand>) -> Self {
        Self { commands }
    }

    pub fn commands(&self) -> &[JointCommand] {
        &self.commands
    }

    pub fn map_pulses(&self, f: impl Fn(JointCommand) -> JointCommand) -> Self {
        Self::new(self.commands.iter().copied().map(f).collect())
    }

    pub fn get(&self, id: JointId) -> Option<Pulse> {
        self.commands
            .iter()
            .find(|command| command.id == id)
            .map(|command| command.pulse)
    }
}

#[derive(Debug, Clone)]
pub struct RobotArmSpec {
    joints: Vec<JointLimit>,
}

impl RobotArmSpec {
    pub fn new(joints: Vec<JointLimit>) -> Result<Self, ArmSpecError> {
        let has_invalid_limits = joints
            .iter()
            .any(|joint| joint.min.0 > joint.neutral.0 || joint.neutral.0 > joint.max.0);

        if has_invalid_limits {
            return Err(ArmSpecError::InvalidLimits);
        }

        let has_duplicate_id = joints.iter().enumerate().any(|(idx, joint)| {
            joints
                .iter()
                .skip(idx + 1)
                .any(|candidate| candidate.id == joint.id)
        });

        if has_duplicate_id {
            return Err(ArmSpecError::DuplicateJointId);
        }

        Ok(Self { joints })
    }

    pub fn lewansoul_learm_provisional() -> Self {
        Self::new(
            (1..=6)
                .map(|id| JointLimit {
                    id: JointId(id),
                    min: Pulse(1100),
                    neutral: Pulse(1500),
                    max: Pulse(1900),
                })
                .collect(),
        )
        .expect("provisional built-in limits are internally valid")
    }

    pub fn joints(&self) -> &[JointLimit] {
        &self.joints
    }

    pub fn neutral_pose(&self) -> ArmPose {
        ArmPose::new(
            self.joints
                .iter()
                .map(|joint| JointCommand {
                    id: joint.id,
                    pulse: joint.neutral,
                })
                .collect(),
        )
    }

    pub fn clamp_pose(&self, pose: &ArmPose) -> ArmPose {
        pose.map_pulses(|command| match self.limit_for(command.id) {
            Some(limit) => JointCommand {
                id: command.id,
                pulse: limit.clamp(command.pulse),
            },
            None => command,
        })
    }

    pub fn validate_pose(&self, pose: &ArmPose) -> Result<(), ArmPoseError> {
        pose.commands().iter().try_for_each(|command| {
            let limit = self
                .limit_for(command.id)
                .ok_or(ArmPoseError::UnknownJoint(command.id))?;

            limit
                .contains(command.pulse)
                .then_some(())
                .ok_or(ArmPoseError::PulseOutOfRange {
                    id: command.id,
                    pulse: command.pulse,
                    min: limit.min,
                    max: limit.max,
                })
        })
    }

    pub fn limit_for(&self, id: JointId) -> Option<JointLimit> {
        self.joints.iter().copied().find(|joint| joint.id == id)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArmSpecError {
    DuplicateJointId,
    InvalidLimits,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArmPoseError {
    UnknownJoint(JointId),
    PulseOutOfRange {
        id: JointId,
        pulse: Pulse,
        min: Pulse,
        max: Pulse,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn neutral_pose_validates() {
        let spec = RobotArmSpec::lewansoul_learm_provisional();

        assert_eq!(spec.validate_pose(&spec.neutral_pose()), Ok(()));
    }

    #[test]
    fn clamp_pose_limits_each_known_joint() {
        let spec = RobotArmSpec::lewansoul_learm_provisional();
        let pose = ArmPose::new(vec![JointCommand::new(1, 100), JointCommand::new(2, 9999)]);

        let clamped = spec.clamp_pose(&pose);

        assert_eq!(clamped.get(JointId(1)), Some(Pulse(1100)));
        assert_eq!(clamped.get(JointId(2)), Some(Pulse(1900)));
    }

    #[test]
    fn provisional_spec_includes_six_controller_channels() {
        let spec = RobotArmSpec::lewansoul_learm_provisional();

        assert_eq!(spec.joints().len(), 6);
        assert!(spec.limit_for(JointId(6)).is_some());
    }
}
