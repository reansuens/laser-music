#![no_std]
#![no_main]

use core::convert::TryInto;
use esp_idf_sys as _;
use panic_halt as _; // halts on panic // required to initialize ESP-IDF runtime

use esp_idf_svc::hal::prelude::Peripherals;
use esp_idf_svc::log::EspLogger;
use esp_idf_svc::wifi::{BlockingWifi, EspWifi};
use esp_idf_svc::{eventloop::EspSystemEventLoop, nvs::EspDefaultNvsPartition};

use embedded_svc::wifi::{AuthMethod, ClientConfiguration, Configuration};

const SSID: &str = env!("AsiaConnect4G54C987_5G");
const PASSWORD: &str = env!("77416525");

#[no_mangle]
pub extern "C" fn app_main() {
    // link runtime patches
    esp_idf_svc::sys::link_patches();

    // initialize logger
    EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();
    let sys_loop = EspSystemEventLoop::take().unwrap();
    let nvs = EspDefaultNvsPartition::take().unwrap();

    let mut wifi = BlockingWifi::wrap(
        EspWifi::new(peripherals.modem, sys_loop.clone(), Some(nvs)).unwrap(),
        sys_loop,
    )
    .unwrap();

    connect_wifi(&mut wifi);

    let ip_info = wifi.wifi().sta_netif().get_ip_info().unwrap();
    log::info!("Wifi DHCP info: {:?}", ip_info);
    log::info!("Shutting down in 5s...");
    // delay 5s using busy-wait (or esp_idf_hal::delay)
    use esp_idf_hal::delay;

    for _ in 0..5_000_000 {
        delay::delay_us(1);
    }
}

fn connect_wifi(wifi: &mut BlockingWifi<EspWifi<'static>>) {
    let wifi_configuration: Configuration = Configuration::Client(ClientConfiguration {
        ssid: SSID.try_into().unwrap(),
        bssid: None,
        auth_method: AuthMethod::WPA2Personal,
        password: PASSWORD.try_into().unwrap(),
        channel: None,
        ..Default::default()
    });

    wifi.set_configuration(&wifi_configuration).unwrap();
    wifi.start().unwrap();
    log::info!("Wifi started");

    wifi.connect().unwrap();
    log::info!("Wifi connected");

    wifi.wait_netif_up().unwrap();
    log::info!("Wifi netif up");
}
