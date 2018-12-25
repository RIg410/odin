extern crate serial as uart;
extern crate actix_web;
extern crate futures;

mod controller;
mod serial;
mod web;

use serial::SerialChannel;
use controller::{SerialDimmer, WebDimmer, Switch, SwitchHandler, DeviceHandler, WebLed, ActionType};
use actix_web::{server, App, http, Path, State, Result as WebResult};
use web::WebController;

fn main() {
    let web_controller = WebController::new();
    let devices = init_devices(&web_controller);
    let switch_handler = init_switch(devices.clone());

    server::new(move || {
        App::with_state(AppState { switch: switch_handler.clone(), devices: devices.clone(), web_controller: web_controller.clone() })
            .prefix("/odin/api")
            .resource("switch/{switch}/{state}", |r| r.method(http::Method::GET).with(switch_hndl))
            .resource("device/{device}/{state}/{power}", |r| r.method(http::Method::GET).with(device_hndl))
            .resource("reg-device/{ids}/{base_url}", |r| r.method(http::Method::GET).with(reg_device))
    })
        .bind("0.0.0.0:1884")
        .expect("Can not bind to port 1884")
        .run();
}

pub struct AppState {
    pub switch: SwitchHandler,
    pub devices: DeviceHandler,
    pub web_controller: WebController,
}

fn switch_hndl((params, state): (Path<(String, String)>, State<AppState>)) -> WebResult<String> {
    println!("switch:{}, state:{}", &params.0, &params.1);
    if let Ok(action_type) = params.1.parse() {
        state.switch.switch(&params.0, action_type);
    } else {
        println!("Unknown state: {}", params.1);
    }

    Ok("Ok".to_owned())
}

fn device_hndl((params, state): (Path<(String, String, u8)>, State<AppState>)) -> WebResult<String> {
    println!("device:{}, state:{}, pow: {}", &params.0, &params.1, &params.2);
    if let Ok(action_type) = params.1.parse() {
        state.devices.set_state(&params.0, action_type, params.2);
    } else {
        println!("Unknown state: {}", params.1);
    }
    Ok("Ok".to_owned())
}

/// 0 - ids (id_1:id_2:id_3)
/// 1 - base_url (host:port)
fn reg_device((params, state): (Path<(String, String)>, State<AppState>)) -> WebResult<String> {
    println!("reg device id:{:?}, ip: {}", &params.0, &params.1);
    let ids = params.0.split(":")
        .map(|s| s.to_owned())
        .collect::<Vec<_>>();
    let host = params.1.to_owned();

    state.web_controller.reg_device(ids, host);
    Ok("Ok".to_owned())
}

fn init_devices(web_controller: &WebController) -> DeviceHandler {
    let mut devices = DeviceHandler::new();
    let serial_channel = SerialChannel::new();

    devices += SerialDimmer::new("bathroom_lamp", 0x01, serial_channel.clone(), true);//25-100
    devices += SerialDimmer::new("corridor_lamp", 0x03, serial_channel.clone(), true); //0-100
    devices += SerialDimmer::new("toilet_lamp", 0x02, serial_channel.clone(), true);//25-100
    devices += SerialDimmer::new("kitchen_lamp", 0x04, serial_channel.clone(), true);//0-100
    devices += SerialDimmer::new("bedroom_lamp", 0x01, serial_channel.clone(), false);
    devices += SerialDimmer::new("lounge_lamp", 0x02, serial_channel.clone(), false);
    devices += SerialDimmer::new("device_3", 0x03, serial_channel.clone(), false);
    devices += SerialDimmer::new("bathroom_fun", 0x04, serial_channel.clone(), false);
    devices += SerialDimmer::new("device_5", 0x05, serial_channel.clone(), false);
    devices += SerialDimmer::new("lounge_cupboard_lamp", 0x06, serial_channel.clone(), false);
    devices += WebDimmer::new("bedroom_beam_bed_lamp", web_controller.clone());
    devices += WebDimmer::new("bedroom_beam_table_lamp", web_controller.clone());
    devices += WebDimmer::new("corridor_beam_lamp", web_controller.clone());
    devices += WebDimmer::new("kitchen_beam_lamp", web_controller.clone());
    devices += WebDimmer::new("lounge_beam_bar_lamp", web_controller.clone());
    devices += WebDimmer::new("lounge_beam_main_lamp", web_controller.clone());
    devices += WebLed::new("bedroom_beam_led", web_controller.clone());
    devices += WebLed::new("corridor_beam_led", web_controller.clone());
    devices += WebLed::new("kitchen_beam_led", web_controller.clone());
    devices += WebLed::new("lounge_beam_bar_led", web_controller.clone());
    devices += WebLed::new("lounge_beam_main_led", web_controller.clone());

    devices
}

fn init_switch(devices: DeviceHandler) -> SwitchHandler {
    let exit_devices = devices.clone();
    SwitchHandler::new(vec![
        Switch::empty("corridor_2"),
        Switch::device("toilet", devices.dev("toilet_lamp")),
        Switch::device("bathroom", devices.dev("bathroom_lamp")),
        Switch::device("bedroom_1", devices.dev("bedroom_lamp")),
        Switch::devices2("bedroom_2", devices.dev("bedroom_beam_bed_lamp"), devices.dev("bedroom_beam_table_lamp")),
        Switch::device("lounge_1", devices.dev("lounge_lamp")),
        Switch::device("lounge_2", devices.dev("lounge_beam_main_lamp")),
        Switch::devices2("kitchen_1", devices.dev("kitchen_lamp"), devices.dev("lounge_beam_bar_lamp")),
        Switch::device("kitchen_2", devices.dev("kitchen_beam_lamp")),
        Switch::empty("balcony_1"),
        Switch::empty("balcony_2"),
        Switch::devices2("exit_1", devices.dev("corridor_lamp"), devices.dev("corridor_beam_lamp")),
        Switch::new("exit_2", move |_| exit_devices.switch_all(ActionType::Off)),
    ])
}