use io::AppState;
use actix_web::{server, App, http, Path, State, Result as WebResult};
use chrono::Local;

pub fn run_web_service(state: AppState) {
    server::new(move || {
        App::with_state(state.clone())
            .prefix("/odin/api")
            .resource("switch/{switch}/{state}", |r| r.method(http::Method::GET).with(switch_hndl))
            .resource("device/{device}/{state}/{power}", |r| r.method(http::Method::GET).with(device_hndl))
            .resource("dimmer/{device}/{power}", |r| r.method(http::Method::GET).with(dimmer_hndl))
            .resource("reg-device/{ids}/{base_url}", |r| r.method(http::Method::GET).with(reg_device))
            .resource("device", |r| r.method(http::Method::GET).with(get_device))
            .resource("time", |r| r.method(http::Method::GET).with(get_time))
    })
        .bind("0.0.0.0:1884")
        .expect("Can not bind to port 1884")
        .run();
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

fn dimmer_hndl((params, state): (Path<(String, u8)>, State<AppState>)) -> WebResult<String> {
    println!("dimmer:{}, pow: {}", &params.0, &params.1);
    state.devices.set_power(&params.0, params.1);
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

/// 0 - ids (id_1:id_2:id_3)
/// 1 - base_url (host:port)
fn get_device(state: State<AppState>) -> WebResult<String> {
    Ok(format!("{:?}", state.web_controller))
}

fn get_time(state: State<AppState>) -> WebResult<String> {
    let time = Local::now();
    Ok(time.to_rfc2822())
}
