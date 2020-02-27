use actix_web::{http, server, App, Json, Path, Result as WebResult, State};
use chrono::{DateTime, Utc};
use home::Runner;
use io::Input;
use sensors::ActionType;
use serde_json::Value;
use web::AppState;

pub fn run_web_service(state: AppState) {
    server::new(move || {
        App::with_state(state.clone())
            .prefix("/odin/api")
            .resource("switch/{switch}/{state}", |r| {
                r.method(http::Method::GET).with(toggle_hndl)
            })
            .resource("reg-device/{ids}/{base_url}", |r| {
                r.method(http::Method::GET).with(reg_device)
            })
            .resource("v1/devices/list", |r| {
                r.method(http::Method::GET).with(devices_list)
            })
            .resource("v1/device/{device}/update", |r| {
                r.method(http::Method::POST).with(update_device)
            })
            .resource("v1/device/{device}/info", |r| {
                r.method(http::Method::GET).with(get_device)
            })
            .resource("v1/switch/{switch}/{state}", |r| {
                r.method(http::Method::GET).with(switch_hndl)
            })
            .resource("v1/script/{name}", |r| {
                r.method(http::Method::POST).with(run_script)
            })
            .resource("v1/time", |r| r.method(http::Method::GET).with(get_time))
    })
    .bind("0.0.0.0:1884")
    .expect("Can not bind to port 1884")
    .run();
}

fn toggle_hndl((params, state): (Path<(String, String)>, State<AppState>)) -> WebResult<String> {
    if let Err(err) = state.io.act(&state.home, &params.0, ActionType::Toggle) {
        println!("toggle switch:{} err: {}", &params.0, err);
        Ok(err)
    } else {
        println!("toggle switch:{} ok", &params.0);
        Ok("Ok".to_owned())
    }
}

fn switch_hndl((params, state): (Path<(String, String)>, State<AppState>)) -> WebResult<String> {
    let act_type = match params.1.as_str() {
        "On" => ActionType::On,
        "Off" => ActionType::Off,
        _ => return Ok("Unknown action type".to_owned()),
    };

    if let Err(err) = state.io.act(&state.home, &params.0, act_type) {
        println!("switch:{} err: {}", &params.0, err);
        Ok(err)
    } else {
        println!("switch:{} ok", &params.0);
        Ok("Ok".to_owned())
    }
}

fn update_device(
    (params, state, value): (Path<String>, State<AppState>, Json<Value>),
) -> WebResult<String> {
    println!("update device:{}, value: {:?}", &params, &value);
    if let Err(err) = state.update_device(&params, value.0) {
        println!("update device err: {}", err);
        Ok(err)
    } else {
        Ok("Ok".to_owned())
    }
}

fn devices_list(state: State<AppState>) -> WebResult<Json<Vec<String>>> {
    Ok(Json(state.devices_list()))
}

fn get_device((params, state): (Path<String>, State<AppState>)) -> WebResult<Json<Value>> {
    match state.get_device(&params) {
        Ok(val) => Ok(Json(val)),
        Err(err) => {
            println!("get device err: {}", err);
            Ok(Json(json!({ "err": err })))
        }
    }
}

/// 0 - ids (id_1:id_2:id_3)
/// 1 - base_url (host:port)
fn reg_device((params, state): (Path<(String, String)>, State<AppState>)) -> WebResult<String> {
    println!("reg device id:{:?}, ip: {}", &params.0, &params.1);
    let ids = params
        .0
        .split(':')
        .map(|s| s.to_owned())
        .collect::<Vec<_>>();
    let host = params.1.to_owned();

    state.io.reg_web_devices(ids, host);
    Ok("Ok".to_owned())
}

fn get_time(_state: State<AppState>) -> WebResult<Json<DateTime<Utc>>> {
    Ok(Json(Utc::now()))
}

fn run_script(
    (params, state, value): (Path<String>, State<AppState>, Json<Value>),
) -> WebResult<String> {
    println!("run script:{:?}[{:?}]", &params, value.0);
    Ok(match state.home.run_script(&params, value.0) {
        Ok(_) => "Ok".to_string(),
        Err(err) => err,
    })
}
