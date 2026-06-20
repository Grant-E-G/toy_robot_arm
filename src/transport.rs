use crate::arm::{JointCommand, Pulse};

pub trait ServoTransport {
    fn write_frame(&mut self, frame: &[u8]) -> Result<(), TransportError>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TransportError {
    WriteFailed(String),
    InvalidCommand(String),
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct MockTransport {
    frames: Vec<Vec<u8>>,
}

impl MockTransport {
    pub fn frames(&self) -> &[Vec<u8>] {
        &self.frames
    }
}

impl ServoTransport for MockTransport {
    fn write_frame(&mut self, frame: &[u8]) -> Result<(), TransportError> {
        self.frames.push(frame.to_vec());
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MoveDurationMs(pub u16);

impl MoveDurationMs {
    pub const fn new(value: u16) -> Self {
        Self(value)
    }
}

pub fn send_servo_move(
    transport: &mut impl ServoTransport,
    commands: &[JointCommand],
    duration: MoveDurationMs,
) -> Result<(), TransportError> {
    let frame = encode_lobot_servo_move(commands, duration)?;
    transport.write_frame(&frame)
}

pub fn encode_lobot_servo_move(
    commands: &[JointCommand],
    duration: MoveDurationMs,
) -> Result<Vec<u8>, TransportError> {
    if commands.is_empty() {
        return Err(TransportError::InvalidCommand(
            "servo move requires at least one command".to_string(),
        ));
    }

    if commands.len() > u8::MAX as usize {
        return Err(TransportError::InvalidCommand(
            "too many servo commands for one frame".to_string(),
        ));
    }

    let params_len = 1 + 2 + (commands.len() * 3);
    let length = checked_u8(params_len + 2, "frame length")?;
    let mut frame = Vec::with_capacity(4 + params_len);

    frame.extend([0x55, 0x55, length, 0x03]);
    frame.push(commands.len() as u8);
    push_u16_le(&mut frame, duration.0);
    commands.iter().try_for_each(|command| {
        validate_pulse(command.pulse)?;
        frame.push(command.id.0);
        push_u16_le(&mut frame, command.pulse.0);
        Ok(())
    })?;

    Ok(frame)
}

pub fn bytes_to_hex(bytes: &[u8]) -> String {
    bytes
        .iter()
        .map(|byte| format!("{byte:02X}"))
        .collect::<Vec<_>>()
        .join(" ")
}

fn push_u16_le(frame: &mut Vec<u8>, value: u16) {
    frame.extend(value.to_le_bytes());
}

fn checked_u8(value: usize, name: &'static str) -> Result<u8, TransportError> {
    u8::try_from(value).map_err(|_| TransportError::InvalidCommand(format!("{name} overflow")))
}

fn validate_pulse(pulse: Pulse) -> Result<(), TransportError> {
    (pulse.0 <= 2500)
        .then_some(())
        .ok_or_else(|| TransportError::InvalidCommand(format!("pulse {} exceeds 2500", pulse.0)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::arm::JointCommand;

    #[test]
    fn encodes_common_lobot_move_frame() {
        let frame = encode_lobot_servo_move(
            &[JointCommand::new(1, 1500), JointCommand::new(2, 1600)],
            MoveDurationMs(1000),
        )
        .unwrap();

        assert_eq!(
            bytes_to_hex(&frame),
            "55 55 0B 03 02 E8 03 01 DC 05 02 40 06"
        );
    }

    #[test]
    fn mock_transport_records_frames() {
        let mut transport = MockTransport::default();

        send_servo_move(
            &mut transport,
            &[JointCommand::new(1, 1500)],
            MoveDurationMs(500),
        )
        .unwrap();

        assert_eq!(transport.frames().len(), 1);
    }
}
