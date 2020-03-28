use crate::home::Runner;
use crate::io::Input;
use crate::sensors::ActionType;
use crate::web::backend::homebridge::{dimmer_brightness, dimmer_status, dimmer_switch, dimmer_brightness_status};
use crate::web::AppState;
use actix_web::web::{Data, Json, Path};
use actix_web::{web, App, HttpResponse, HttpServer};
use chrono::Utc;
use serde_json::Value;

pub async fn run_web_service(state: AppState) -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .data(state.clone())
            .service(
                web::scope("/odin/api")
                    .route("switch/{switch}/{state}", web::get().to(toggle_hndl))
                    .route("reg-device/{ids}/{base_url}", web::get().to(reg_device))
                    .route("v1/devices/list", web::get().to(devices_list))
                    .route("v1/device/{device}/update", web::post().to(update_device))
                    .route("v1/device/{device}/info", web::get().to(get_device))
                    .route("v1/switch/{switch}/{state}", web::get().to(switch_hndl))
                    .route("v1/script/{name}", web::post().to(run_script))
                    .route("v1/time", web::get().to(get_time)),
            )
            .service(
                web::scope("/homebridge/api")
                    .route(
                        "dimmer_switch/{device}/{state}",
                        web::get().to(dimmer_switch),
                    )
                    .route(
                        "dimmer_brightness/{device}/{state}",
                        web::get().to(dimmer_brightness),
                    )
                    .route("dimmer_status/{device}", web::get().to(dimmer_status))
                    .route("dimmer_brightness_status/{device}", web::get().to(dimmer_brightness_status)),
            )
    })
    .bind("0.0.0.0:1884")
    .expect("Can not bind to port 1884")
    .run()
    .await
}

async fn toggle_hndl(params: Path<(String, String)>, state: Data<AppState>) -> HttpResponse {
    if let Err(err) = state.io.act(&state.home, &params.0, ActionType::Toggle) {
        error!("toggle switch:{} err: {}", &params.0, err);
        HttpResponse::InternalServerError().json(json!({"err": err.to_string()}))
    } else {
        info!("toggle switch:{} ok", &params.0);
        HttpResponse::Ok().json(json!({"ok:": "ok"}))
    }
}

async fn switch_hndl(params: Path<(String, String)>, state: Data<AppState>) -> HttpResponse {
    let act_type = match params.1.as_str() {
        "On" => ActionType::On,
        "Off" => ActionType::Off,
        _ => return HttpResponse::InternalServerError().json(json!({"err":"Unknown action type"})),
    };

    if let Err(err) = state.io.act(&state.home, &params.0, act_type) {
        error!("switch:{} err: {}", &params.0, err);
        HttpResponse::InternalServerError().json(json!({"err":err.to_string()}))
    } else {
        info!("switch:{} ok", &params.0);
        HttpResponse::Ok().json(json!({"ok:": "ok"}))
    }
}

async fn update_device(
    params: Path<String>,
    state: Data<AppState>,
    value: Json<Value>,
) -> HttpResponse {
    info!("update device:{}, value: {:?}", &params, &value);
    if let Err(err) = state.update_device(&params, value.0) {
        error!("update device err: {}", err);
        HttpResponse::InternalServerError().json(json!({"err": err.to_string()}))
    } else {
        HttpResponse::Ok().json(json!({"ok:": "ok"}))
    }
}

async fn devices_list(state: Data<AppState>) -> HttpResponse {
    HttpResponse::Ok().json(state.devices_list())
}

async fn get_device(params: Path<String>, state: Data<AppState>) -> HttpResponse {
    match state.get_device(&params) {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(err) => {
            error!("get device err: {}", err);
            HttpResponse::InternalServerError().json(json!({ "err": err.to_string() }))
        }
    }
}

/// 0 - ids (id_1:id_2:id_3)
/// 1 - base_url (host:port)
async fn reg_device(params: Path<(String, String)>, state: Data<AppState>) -> HttpResponse {
    info!("reg device id:{:?}, ip: {}", &params.0, &params.1);
    let ids = params
        .0
        .split(':')
        .map(|s| s.to_owned())
        .collect::<Vec<_>>();
    let host = params.1.to_owned();

    state.io.reg_web_devices(ids, host);
    HttpResponse::Ok().json(json!({"ok:": "ok"}))
}

async fn get_time(_state: Data<AppState>) -> HttpResponse {
    HttpResponse::Ok().json(Utc::now())
}

async fn run_script(
    params: Path<String>,
    value: Json<Value>,
    state: Data<AppState>,
) -> HttpResponse {
    info!("run script:{:?}[{:?}]", &params, value.0);
    match state.home.run_script(&params, value.0) {
        Ok(_) => HttpResponse::Ok().json(json!({"ok:": "ok"})),
        Err(err) => {
            error!("Failed to run script: {:?}", err);
            HttpResponse::InternalServerError().json(json!({ "err": err.to_string() }))
        }
    }
}

mod homebridge {
    use crate::web::AppState;
    use actix_web::web::{Data, Path};
    use actix_web::HttpResponse;

    pub async fn dimmer_switch(
        params: Path<(String, String)>,
        state: Data<AppState>,
    ) -> HttpResponse {
        let is_on = match params.1.as_str() {
            "On" => true,
            "Off" => false,
            _ => {
                return HttpResponse::InternalServerError()
                    .json(json!({"err": "Unknown action type"}))
            }
        };

        if let Err(err) = state.update_device(&params.0, json!({ "is_on": is_on })) {
            error!("switch dimmer:{} err: {}", &params.0, err);
            HttpResponse::InternalServerError().json(json!({"err": err.to_string()}))
        } else {
            info!("switch dimmer:{} -> {} ok", params.0, params.1);
            HttpResponse::Ok().json(json!({"ok:": "ok"}))
        }
    }

    pub async fn dimmer_status(name: Path<String>, state: Data<AppState>) -> String {
        let is_on = state
            .get_device(&name)
            .map_err(|err| err.to_string())
            .and_then(|val| {
                val.get("is_on")
                    .and_then(|val| val.as_bool())
                    .ok_or_else(|| format!("Invalid status object:{}", val))
            });
        match is_on {
            Ok(val) => if val { "1" } else { "0" }.to_owned(),
            Err(err) => {
                error!("dimmer_status err: {}", err);
                format!("dimmer_status err: {}", err)
            }
        }
    }

    pub async fn dimmer_brightness(
        params: Path<(String, String)>,
        state: Data<AppState>,
    ) -> HttpResponse {
        let res = params
            .1
            .parse::<u64>()
            .map_err(|err| err.to_string())
            .and_then(|brightness| {
                state
                    .update_device(&params.0, json!({ "brightness": brightness }))
                    .map_err(|err| err.to_string())
            });

        if let Err(err) = res {
            error!("dimmer_brightness:{} err: {}", &params.0, err);
            HttpResponse::InternalServerError().json(json!({"err": err.to_string()}))
        } else {
            info!("dimmer_brightness:{} ok", &params.0);
            HttpResponse::Ok().json(json!({"ok:": "ok"}))
        }
    }

    pub async fn dimmer_brightness_status(name: Path<String>, state: Data<AppState>) -> String {
        let is_on = state
            .get_device(&name)
            .map_err(|err| err.to_string())
            .and_then(|val| {
                val.get("brightness")
                    .and_then(|val| val.as_u64())
                    .ok_or_else(|| format!("Invalid status object:{}", val))
            });
        match is_on {
            Ok(val) => format!("{}", val),
            Err(err) => {
                error!("dimmer_status err: {}", err);
                format!("dimmer_status err: {}", err)
            }
        }
    }
}
