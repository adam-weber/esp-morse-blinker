# Morse Code Blinker Firmware

ESP32 firmware that blinks an LED in a morse code pattern. This firmware is designed to work with the [ESP WebFlash Toolkit](https://github.com/adam-weber/esp-webflash-toolkit) web flasher.

## Features

- Reads configuration from NVS partition (written by web flasher)
- Connects to WiFi (optional)
- Blinks LED in configurable morse code pattern
- Configurable LED GPIO pin and timing

## Supported Targets

Pre-built binaries are automatically built for the following ESP32 targets:

- **ESP32** - Original ESP32 (dual-core Xtensa)
- **ESP32-C3** - RISC-V with USB serial/JTAG
- **ESP32-S2** - Single-core Xtensa with USB OTG
- **ESP32-S3** - Dual-core Xtensa with USB OTG
- **ESP32-C2** - Low-cost RISC-V
- **ESP32-C6** - WiFi 6, Zigbee, Thread support
- **ESP32-H2** - BLE 5, Zigbee, Thread support

## Configuration

The firmware reads the following configuration from NVS (namespace: "config"):

- `wifi_ssid` - WiFi network name (optional)
- `wifi_pass` - WiFi password (optional)
- `led_gpio` - GPIO pin for LED (default: 2)
- `morse_pattern` - Morse code pattern (default: "... --- ..." = SOS)
- `morse_dot_ms` - Dot duration in milliseconds (default: 200)

## Using with Web Flasher (Recommended)

The easiest way to use this firmware is with the web-based flasher:
https://adam-weber.github.io/esp-webflash-toolkit/flasher/?project=morse-code-blinker

The web flasher will:
1. Flash the pre-built firmware to your ESP32
2. Write your configuration (WiFi, LED pin, morse pattern) to the NVS partition
3. The device will automatically read the configuration and start blinking

No installation or build tools required!

## Building from Source

### Prerequisites

1. Install Rust: https://rustup.rs/
2. Install the ESP Rust toolchain:
   ```bash
   cargo install espup
   espup install
   . $HOME/export-esp.sh
   ```
3. Install espflash:
   ```bash
   cargo install espflash
   ```

### Build and Flash

For ESP32 (original):
```bash
cd firmware/morse-blinker
cargo build --release
espflash flash --monitor target/xtensa-esp32-espidf/release/morse-blinker
```

For other targets, update `.cargo/config.toml` to set the appropriate target:
- ESP32-C3: `riscv32imc-esp-espidf`
- ESP32-S2: `xtensa-esp32s2-espidf`
- ESP32-S3: `xtensa-esp32s3-espidf`
- ESP32-C6: `riscv32imac-esp-espidf`

## Pre-built Binaries

Pre-built binaries for all supported targets are available in the [GitHub Actions artifacts](https://github.com/adam-weber/esp-webflash-toolkit/actions/workflows/build-firmware.yml). Download the appropriate binary for your chip and flash with:

```bash
espflash write-bin 0x0 morse-blinker-esp32.bin
```

## Morse Code Format

The morse pattern uses the following characters:
- `.` - Dot (short blink)
- `-` - Dash (long blink, 3x dot duration)
- ` ` - Space (letter gap, 3x dot duration)

Example patterns:
- `... --- ...` - SOS
- `.... .` - HE
- `.- -... -.-.` - ABC

## License

MIT
