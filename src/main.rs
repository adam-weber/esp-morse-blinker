use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::gpio::PinDriver;
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::nvs::EspDefaultNvsPartition;
use esp_idf_svc::wifi::{AuthMethod, BlockingWifi, ClientConfiguration, Configuration, EspWifi};
use esp_idf_sys as _;
use log::*;

fn main() -> anyhow::Result<()> {
    // Initialize ESP-IDF services
    esp_idf_sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    info!("Morse Code Blinker starting...");

    let peripherals = Peripherals::take()?;
    let sysloop = EspSystemEventLoop::take()?;
    let nvs = EspDefaultNvsPartition::take()?;

    // Read configuration from NVS
    let (wifi_ssid, wifi_password) = read_wifi_config(&nvs)?;
    let led_gpio = read_nvs_u8(&nvs, "config", "led_gpio").unwrap_or(2);
    let morse_pattern = read_nvs_string(&nvs, "config", "morse_pattern")
        .unwrap_or_else(|| "... --- ...".to_string());
    let morse_dot_ms = read_nvs_u32(&nvs, "config", "morse_dot_ms").unwrap_or(200);

    info!("Configuration loaded:");
    info!("  LED GPIO: {}", led_gpio);
    info!("  Morse Pattern: {}", morse_pattern);
    info!("  Dot Duration: {}ms", morse_dot_ms);

    // Connect to WiFi if credentials are provided
    if !wifi_ssid.is_empty() {
        info!("Connecting to WiFi SSID: {}", wifi_ssid);
        connect_wifi(wifi_ssid, wifi_password, peripherals.modem, sysloop, nvs)?;
        info!("WiFi connected successfully!");
    } else {
        info!("No WiFi credentials configured, skipping WiFi connection");
    }

    // Initialize LED pin - support common GPIO pins
    let led_pin = match led_gpio {
        2 => peripherals.pins.gpio2.into(),
        4 => peripherals.pins.gpio4.into(),
        5 => peripherals.pins.gpio5.into(),
        12 => peripherals.pins.gpio12.into(),
        13 => peripherals.pins.gpio13.into(),
        14 => peripherals.pins.gpio14.into(),
        15 => peripherals.pins.gpio15.into(),
        16 => peripherals.pins.gpio16.into(),
        17 => peripherals.pins.gpio17.into(),
        18 => peripherals.pins.gpio18.into(),
        19 => peripherals.pins.gpio19.into(),
        21 => peripherals.pins.gpio21.into(),
        22 => peripherals.pins.gpio22.into(),
        23 => peripherals.pins.gpio23.into(),
        25 => peripherals.pins.gpio25.into(),
        26 => peripherals.pins.gpio26.into(),
        27 => peripherals.pins.gpio27.into(),
        32 => peripherals.pins.gpio32.into(),
        33 => peripherals.pins.gpio33.into(),
        _ => {
            error!("Unsupported GPIO pin: {}. Supported pins: 2, 4, 5, 12-19, 21-23, 25-27, 32-33", led_gpio);
            anyhow::bail!("Unsupported GPIO pin: {}", led_gpio);
        }
    };

    let mut led = PinDriver::output(led_pin)?;
    info!("LED initialized on GPIO {}", led_gpio);

    // Blink morse code in a loop
    info!("Starting morse code pattern loop");
    loop {
        blink_morse_code(&mut led, &morse_pattern, morse_dot_ms)?;
        // Pause between repetitions (7 dot durations = standard word gap)
        FreeRtos::delay_ms(morse_dot_ms * 7);
    }
}

fn read_wifi_config(nvs: &EspDefaultNvsPartition) -> anyhow::Result<(String, String)> {
    let ssid = read_nvs_string(nvs, "config", "wifi_ssid").unwrap_or_default();
    let password = read_nvs_string(nvs, "config", "wifi_pass").unwrap_or_default();
    Ok((ssid, password))
}

fn read_nvs_string(
    nvs: &EspDefaultNvsPartition,
    namespace: &str,
    key: &str,
) -> Option<String> {
    use esp_idf_svc::nvs::*;

    let nvs_handle = EspNvs::new(nvs.clone(), namespace, true).ok()?;
    let mut buf = [0u8; 256];
    match nvs_handle.get_str(key, &mut buf) {
        Ok(Some(value)) => Some(value.to_string()),
        _ => None,
    }
}

fn read_nvs_u8(nvs: &EspDefaultNvsPartition, namespace: &str, key: &str) -> Option<u8> {
    use esp_idf_svc::nvs::*;

    let nvs_handle = EspNvs::new(nvs.clone(), namespace, true).ok()?;
    nvs_handle.get_u8(key).ok()?
}

fn read_nvs_u32(nvs: &EspDefaultNvsPartition, namespace: &str, key: &str) -> Option<u32> {
    use esp_idf_svc::nvs::*;

    let nvs_handle = EspNvs::new(nvs.clone(), namespace, true).ok()?;
    nvs_handle.get_u32(key).ok()?
}

fn connect_wifi(
    ssid: String,
    password: String,
    modem: impl esp_idf_hal::peripheral::Peripheral<P = esp_idf_hal::modem::Modem> + 'static,
    sysloop: EspSystemEventLoop,
    nvs: EspDefaultNvsPartition,
) -> anyhow::Result<()> {
    let mut wifi = BlockingWifi::wrap(
        EspWifi::new(modem, sysloop.clone(), Some(nvs))?,
        sysloop,
    )?;

    let wifi_config = Configuration::Client(ClientConfiguration {
        ssid: ssid.as_str().try_into()?,
        password: password.as_str().try_into()?,
        auth_method: AuthMethod::WPA2Personal,
        ..Default::default()
    });

    wifi.set_configuration(&wifi_config)?;
    wifi.start()?;
    wifi.connect()?;
    wifi.wait_netif_up()?;

    Ok(())
}

fn blink_morse_code(
    led: &mut PinDriver<'_, esp_idf_hal::gpio::AnyOutputPin, esp_idf_hal::gpio::Output>,
    pattern: &str,
    dot_ms: u32,
) -> anyhow::Result<()> {
    for ch in pattern.chars() {
        match ch {
            '.' => {
                // Dot: LED on for 1 unit
                led.set_high()?;
                FreeRtos::delay_ms(dot_ms);
                led.set_low()?;
                FreeRtos::delay_ms(dot_ms); // Gap between symbols
            }
            '-' => {
                // Dash: LED on for 3 units
                led.set_high()?;
                FreeRtos::delay_ms(dot_ms * 3);
                led.set_low()?;
                FreeRtos::delay_ms(dot_ms); // Gap between symbols
            }
            ' ' => {
                // Space: 3 unit gap (letter gap)
                // We already have 1 unit gap from previous symbol, add 2 more
                FreeRtos::delay_ms(dot_ms * 2);
            }
            _ => {
                // Ignore unknown characters
                warn!("Ignoring unknown morse character: {}", ch);
            }
        }
    }

    Ok(())
}
