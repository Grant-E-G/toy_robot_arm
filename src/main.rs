use std::env;
use std::process::ExitCode;

use toy_robot_arm::arm::{JointCommand, RobotArmSpec};
use toy_robot_arm::control::{step_visual_servo, ControllerGains, VisualServoGoal};
use toy_robot_arm::transport::{bytes_to_hex, encode_lobot_servo_move, MoveDurationMs};
use toy_robot_arm::vision::Point3;

fn main() -> ExitCode {
    match run(env::args().skip(1).collect()) {
        Ok(output) => {
            println!("{output}");
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("{error}");
            ExitCode::from(2)
        }
    }
}

fn run(args: Vec<String>) -> Result<String, String> {
    match args.first().map(String::as_str) {
        Some("sim") => run_sim(),
        Some("frame") => run_frame(&args[1..]),
        Some("help") | Some("--help") | Some("-h") | None => Ok(help()),
        Some(command) => Err(format!("unknown command '{command}'\n\n{}", help())),
    }
}

fn run_sim() -> Result<String, String> {
    let spec = RobotArmSpec::lewansoul_learm_provisional();
    let pose = spec.neutral_pose();
    let goal = VisualServoGoal {
        target: Point3::new(0.04, -0.02, 0.38),
        tolerance_m: 0.01,
    };
    let observed_tool = Point3::new(-0.03, 0.01, 0.45);
    let (next_pose, step) = step_visual_servo(
        &spec,
        &pose,
        observed_tool,
        goal,
        ControllerGains::default(),
    );
    let frame = encode_lobot_servo_move(next_pose.commands(), MoveDurationMs::new(750))
        .map_err(|error| format!("{error:?}"))?;

    Ok(format!(
        "error_m: x={:.3}, y={:.3}, z={:.3}\ncommands: {}\nframe: {}",
        step.error.x_m,
        step.error.y_m,
        step.error.z_m,
        format_commands(next_pose.commands()),
        bytes_to_hex(&frame)
    ))
}

fn run_frame(args: &[String]) -> Result<String, String> {
    let mut duration = MoveDurationMs::new(1000);
    let mut commands = Vec::new();
    let mut index = 0;

    while index < args.len() {
        match args[index].as_str() {
            "--ms" => {
                let value = args
                    .get(index + 1)
                    .ok_or_else(|| "--ms requires a value".to_string())?;
                duration = MoveDurationMs::new(parse_u16(value, "duration ms")?);
                index += 2;
            }
            command => {
                commands.push(parse_joint_command(command)?);
                index += 1;
            }
        }
    }

    let frame =
        encode_lobot_servo_move(&commands, duration).map_err(|error| format!("{error:?}"))?;

    Ok(bytes_to_hex(&frame))
}

fn parse_joint_command(value: &str) -> Result<JointCommand, String> {
    let (id, pulse) = value
        .split_once(':')
        .ok_or_else(|| format!("joint command '{value}' must look like id:pulse"))?;

    Ok(JointCommand::new(
        parse_u16(id, "servo id")? as u8,
        parse_u16(pulse, "pulse")?,
    ))
}

fn parse_u16(value: &str, name: &'static str) -> Result<u16, String> {
    value
        .parse::<u16>()
        .map_err(|_| format!("{name} '{value}' is not a u16"))
}

fn format_commands(commands: &[JointCommand]) -> String {
    commands
        .iter()
        .map(|command| format!("{}:{}", command.id, command.pulse.0))
        .collect::<Vec<_>>()
        .join(", ")
}

fn help() -> String {
    [
        "toy-robot-arm",
        "",
        "Commands:",
        "  sim                         Run one visual-servo simulation step",
        "  frame [--ms N] id:pulse...  Encode a Lobot/Hiwonder servo move frame",
        "",
        "Examples:",
        "  toy-robot-arm sim",
        "  toy-robot-arm frame --ms 750 1:1500 2:1450 3:1600",
    ]
    .join("\n")
}
