#!/usr/bin/env python3
"""Send a tightly bounded one-channel jog to a Hiwonder/Lobot-style controller.

Default mode is dry-run: it prints the exact bytes that would be sent and does
not open the serial device. To actually transmit, pass both --send and
--i-understand-this-can-move.
"""

from __future__ import annotations

import argparse
import fcntl
import os
import termios
import time


DEFAULT_BAUD = 9600
DEFAULT_DURATION_MS = 750
DEFAULT_DELTA_US = 10
INITIAL_SAFE_MIN_US = 1100
INITIAL_SAFE_MAX_US = 1900

BAUD_TO_TERMIOS = {
    9600: termios.B9600,
    19200: termios.B19200,
    38400: termios.B38400,
    57600: termios.B57600,
    115200: termios.B115200,
}


def main() -> int:
    parser = argparse.ArgumentParser(
        description="Dry-run or send a conservative one-channel servo jog."
    )
    parser.add_argument("--device", help="serial device, for example /dev/ttyUSB0")
    parser.add_argument("--baud", type=int, default=DEFAULT_BAUD)
    parser.add_argument("--channel", type=int, required=True, help="controller output channel 1..6")
    parser.add_argument(
        "--current-pulse",
        type=int,
        required=True,
        help="pulse value you believe the channel is currently holding",
    )
    parser.add_argument(
        "--delta",
        type=int,
        default=DEFAULT_DELTA_US,
        help=f"jog delta in microseconds; default {DEFAULT_DELTA_US}; max +/-20",
    )
    parser.add_argument(
        "--duration-ms",
        type=int,
        default=DEFAULT_DURATION_MS,
        help=f"move duration; default {DEFAULT_DURATION_MS}",
    )
    parser.add_argument(
        "--allow-wide-range",
        action="store_true",
        help="allow 500..2500 us instead of the initial 1100..1900 us safety window",
    )
    parser.add_argument(
        "--send",
        action="store_true",
        help="actually transmit frames; otherwise this is a dry-run",
    )
    parser.add_argument(
        "--i-understand-this-can-move",
        action="store_true",
        help="required with --send",
    )
    args = parser.parse_args()

    validate_args(args)

    target_pulse = args.current_pulse + args.delta
    jog_frame = build_servo_move_packet(args.channel, target_pulse, args.duration_ms)
    return_frame = build_servo_move_packet(args.channel, args.current_pulse, args.duration_ms)

    print("Safe jog plan")
    print(f"channel: {args.channel}")
    print(f"current pulse: {args.current_pulse} us")
    print(f"target pulse: {target_pulse} us")
    print(f"return pulse: {args.current_pulse} us")
    print(f"duration: {args.duration_ms} ms per move")
    print(f"baud: {args.baud}")
    print(f"jog frame:    {to_hex(jog_frame)}")
    print(f"return frame: {to_hex(return_frame)}")

    if not args.send:
        print()
        print("Dry-run only. No serial device was opened and no bytes were sent.")
        return 0

    if not args.device:
        raise SystemExit("--device is required when using --send")
    if not args.i_understand_this_can_move:
        raise SystemExit("--i-understand-this-can-move is required when using --send")

    print()
    print("TRANSMITTING. Keep power cutoff within reach.")
    with SerialPort(args.device, args.baud) as port:
        port.write(jog_frame)
        time.sleep(args.duration_ms / 1000.0 + 0.25)
        port.write(return_frame)
        time.sleep(args.duration_ms / 1000.0 + 0.25)

    print("Done.")
    return 0


def validate_args(args: argparse.Namespace) -> None:
    if not 1 <= args.channel <= 6:
        raise SystemExit("--channel must be in 1..6")
    if args.baud not in BAUD_TO_TERMIOS:
        supported = ", ".join(str(value) for value in sorted(BAUD_TO_TERMIOS))
        raise SystemExit(f"unsupported baud {args.baud}; supported values: {supported}")
    if not 1 <= args.duration_ms <= 30000:
        raise SystemExit("--duration-ms must be in 1..30000")
    if args.delta == 0 or abs(args.delta) > 20:
        raise SystemExit("--delta must be non-zero and no larger than +/-20 us")

    low = 500 if args.allow_wide_range else INITIAL_SAFE_MIN_US
    high = 2500 if args.allow_wide_range else INITIAL_SAFE_MAX_US
    target_pulse = args.current_pulse + args.delta
    for name, pulse in (("current pulse", args.current_pulse), ("target pulse", target_pulse)):
        if not low <= pulse <= high:
            raise SystemExit(
                f"{name} {pulse} is outside {low}..{high} us; "
                "use --allow-wide-range only after calibration"
            )


def build_servo_move_packet(channel: int, pulse_us: int, duration_ms: int) -> bytes:
    """Build 55 55 LEN 03 COUNT TIME_L TIME_H ID POS_L POS_H."""
    packet = bytearray()
    packet.extend((0x55, 0x55))
    packet.append(8)
    packet.append(0x03)
    packet.append(1)
    packet.extend(duration_ms.to_bytes(2, "little"))
    packet.append(channel)
    packet.extend(pulse_us.to_bytes(2, "little"))
    return bytes(packet)


class SerialPort:
    def __init__(self, device: str, baud: int) -> None:
        self.device = device
        self.baud = baud
        self.fd: int | None = None

    def __enter__(self) -> "SerialPort":
        self.fd = os.open(self.device, os.O_RDWR | os.O_NOCTTY | os.O_NONBLOCK)
        configure_raw_serial(self.fd, BAUD_TO_TERMIOS[self.baud])
        return self

    def __exit__(self, exc_type, exc, tb) -> None:
        if self.fd is not None:
            os.close(self.fd)
            self.fd = None

    def write(self, data: bytes) -> None:
        if self.fd is None:
            raise RuntimeError("serial port is not open")
        written = os.write(self.fd, data)
        if written != len(data):
            raise RuntimeError(f"short serial write: {written} of {len(data)} bytes")
        termios.tcdrain(self.fd)


def configure_raw_serial(fd: int, baud_const: int) -> None:
    attrs = termios.tcgetattr(fd)
    attrs[0] = 0
    attrs[1] = 0
    attrs[2] = termios.CLOCAL | termios.CREAD | termios.CS8
    attrs[3] = 0
    attrs[4] = baud_const
    attrs[5] = baud_const
    attrs[6][termios.VMIN] = 0
    attrs[6][termios.VTIME] = 0
    termios.tcsetattr(fd, termios.TCSANOW, attrs)
    termios.tcflush(fd, termios.TCIOFLUSH)

    flags = fcntl.fcntl(fd, fcntl.F_GETFL)
    fcntl.fcntl(fd, fcntl.F_SETFL, flags | os.O_NONBLOCK)


def to_hex(data: bytes) -> str:
    return " ".join(f"{byte:02X}" for byte in data)


if __name__ == "__main__":
    raise SystemExit(main())
