use crate::arm::{JointId, JointLimit, Pulse};

#[kani::proof]
fn joint_limit_clamp_keeps_any_pulse_inside_limit() {
    let limit = JointLimit {
        id: JointId(1),
        min: Pulse(1100),
        neutral: Pulse(1500),
        max: Pulse(1900),
    };

    let clamped = limit.clamp(Pulse(kani::any()));

    assert!(limit.contains(clamped));
}
