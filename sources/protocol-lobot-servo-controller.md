# Provisional Servo Controller Protocol

This repo currently encodes a common LewanSoul/Hiwonder/Lobot-style binary
servo controller frame for moving multiple PWM servos together.

## Servo Move Frame

Byte layout:

```text
55 55 LEN CMD COUNT TIME_L TIME_H [ID POS_L POS_H]...
```

Where:

- `55 55`: fixed frame header.
- `LEN`: parameter byte count plus 2.
- `CMD`: `03` for servo move in the provisional implementation.
- `COUNT`: number of servo commands.
- `TIME_L TIME_H`: little-endian movement duration in milliseconds.
- `ID`: servo channel ID.
- `POS_L POS_H`: little-endian target pulse width.

Example for servo 1 to `1500`, servo 2 to `1600`, over `1000 ms`:

```text
55 55 0B 03 02 E8 03 01 DC 05 02 40 06
```

## Safety Notes

- Treat this as a working hypothesis until matched against the actual board or
  vendor software bundled with the arm.
- Do not send live frames until servo IDs, pulse limits, and power supply are
  verified.
- Keep the first live tests slow and within a small range around neutral.
