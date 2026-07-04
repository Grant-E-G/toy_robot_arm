#!/usr/bin/env python3
"""Verify USB serial controller assumptions without transmitting commands.

This script is intentionally read-only. It lists likely Linux serial devices,
optionally opens one at the expected Hiwonder/Lobot LSC baud rate, and can listen
for unsolicited bytes. It never writes to the controller, so it should not move
any servo.
"""

from __future__ import annotations

import argparse
import fcntl
import os
from pathlib import Path
import select
import termios
import time


DEFAULT_BAUD = 9600
TTY_GLOBS = ("/dev/ttyUSB*", "/dev/ttyACM*", "/dev/serial/by-id/*")

BAUD_TO_TERMIOS = {
    9600: termios.B9600,
    19200: termios.B19200,
    38400: termios.B38400,
    57600: termios.B57600,
    115200: termios.B115200,
}


def main() -> int:
    parser = argparse.ArgumentParser(
        description="Inspect and open the robot-arm USB serial controller without sending bytes."
    )
    parser.add_argument(
        "--device",
        help="serial device to open, for example /dev/ttyUSB0 or /dev/serial/by-id/...",
    )
    parser.add_argument(
        "--baud",
        type=int,
        default=DEFAULT_BAUD,
        help=f"serial baud rate to configure while opening the port; default {DEFAULT_BAUD}",
    )
    parser.add_argument(
        "--listen-seconds",
        type=float,
        default=0.0,
        help="read unsolicited bytes for this many seconds; still does not transmit",
    )
    args = parser.parse_args()

    print("Controller assumption check")
    print(f"expected protocol: 0x55 0x55 framed Hiwonder/Lobot LSC-style serial")
    print(f"expected baud: {args.baud}")
    print("transmit: disabled; this verifier never writes to the serial device")
    print()

    devices = discover_devices()
    if devices:
        print("Candidate serial devices:")
        for device in devices:
            print(f"  {device}")
    else:
        print("No /dev/ttyUSB*, /dev/ttyACM*, or /dev/serial/by-id/* devices found.")

    print()
    print("USB tty details:")
    print_tty_details()

    if args.device:
        print()
        verify_open(args.device, args.baud, args.listen_seconds)
    else:
        print()
        print("Pass --device PATH to verify the controller port can be opened.")

    return 0


def discover_devices() -> list[str]:
    devices: list[str] = []
    for pattern in TTY_GLOBS:
        devices.extend(str(path) for path in sorted(Path("/").glob(pattern.lstrip("/"))))
    return unique(devices)


def unique(values: list[str]) -> list[str]:
    seen: set[str] = set()
    result: list[str] = []
    for value in values:
        resolved = str(Path(value).resolve()) if Path(value).exists() else value
        key = f"{value}->{resolved}"
        if key not in seen:
            result.append(value)
            seen.add(key)
    return result


def print_tty_details() -> None:
    sys_tty = Path("/sys/class/tty")
    rows = []
    for tty in sorted(sys_tty.glob("ttyUSB*")) + sorted(sys_tty.glob("ttyACM*")):
        device = (tty / "device").resolve()
        rows.append(
            {
                "name": tty.name,
                "dev": f"/dev/{tty.name}",
                "driver": read_driver_name(device),
                "vendor": read_first(device, ("idVendor", "../idVendor", "../../idVendor")),
                "product": read_first(device, ("idProduct", "../idProduct", "../../idProduct")),
                "manufacturer": read_first(
                    device, ("manufacturer", "../manufacturer", "../../manufacturer")
                ),
                "product_name": read_first(device, ("product", "../product", "../../product")),
            }
        )

    if not rows:
        print("  no USB/ACM tty entries visible in /sys/class/tty")
        return

    for row in rows:
        print(f"  {row['dev']}")
        print(f"    driver: {row['driver'] or 'unknown'}")
        print(f"    USB VID:PID: {row['vendor'] or '????'}:{row['product'] or '????'}")
        if row["manufacturer"]:
            print(f"    manufacturer: {row['manufacturer']}")
        if row["product_name"]:
            print(f"    product: {row['product_name']}")


def read_driver_name(device: Path) -> str | None:
    driver = device / "driver"
    if not driver.exists():
        return None
    try:
        return driver.resolve().name
    except OSError:
        return None


def read_first(base: Path, relative_paths: tuple[str, ...]) -> str | None:
    for relative_path in relative_paths:
        value = read_text(base / relative_path)
        if value:
            return value
    return None


def read_text(path: Path) -> str | None:
    try:
        return path.read_text(encoding="utf-8").strip()
    except OSError:
        return None


def verify_open(device: str, baud: int, listen_seconds: float) -> None:
    baud_const = BAUD_TO_TERMIOS.get(baud)
    if baud_const is None:
        supported = ", ".join(str(value) for value in sorted(BAUD_TO_TERMIOS))
        raise SystemExit(f"unsupported baud {baud}; supported values: {supported}")

    fd = os.open(device, os.O_RDWR | os.O_NOCTTY | os.O_NONBLOCK)
    try:
        configure_raw_serial(fd, baud_const)
        print(f"Opened {device} at {baud} baud in raw, non-blocking mode.")
        print("No bytes have been transmitted.")
        if listen_seconds > 0:
            listen(fd, listen_seconds)
    finally:
        os.close(fd)


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


def listen(fd: int, seconds: float) -> None:
    print(f"Listening for unsolicited controller bytes for {seconds:.1f}s...")
    deadline = time.monotonic() + seconds
    chunks: list[bytes] = []
    while time.monotonic() < deadline:
        readable, _, _ = select.select([fd], [], [], min(0.1, deadline - time.monotonic()))
        if fd in readable:
            chunk = os.read(fd, 4096)
            if chunk:
                chunks.append(chunk)

    data = b"".join(chunks)
    if data:
        print(f"Received {len(data)} byte(s): {to_hex(data)}")
    else:
        print("No unsolicited bytes received. That is normal for many servo controllers.")


def to_hex(data: bytes) -> str:
    return " ".join(f"{byte:02X}" for byte in data)


if __name__ == "__main__":
    raise SystemExit(main())
