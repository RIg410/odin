use io::IOBuilder;
use devices::{SerialSwitch, WebBeam, SerialDimmer, WebSwitch, Switch as SwitchTrait};
use sensors::{Switch, ActionType};
use timer::Timer;
use std::sync::{RwLock, Arc, Mutex};
use std::time::{Duration, SystemTime};
use chrono::{Local, Timelike};
use std::thread;
use std::thread::JoinHandle;
use std::sync::mpsc::{Sender, channel, Receiver};
use std::collections::HashMap;
use serde::export::fmt::Debug;
use home::scripts::Script;
use std::sync::atomic::{AtomicBool, Ordering};

pub trait Runner {
    fn run_script(&self, name: &str) -> Result<(), String>;
}

#[derive(Debug, Clone)]
pub struct Home {
    pub bad_room: Arc<BadRoom>,
    pub living_room: Arc<LivingRoom>,
    pub kitchen: Arc<Kitchen>,
    pub balcony: Arc<Balcony>,
    pub corridor: Arc<Corridor>,
    pub toilet: Arc<Toilet>,
    pub bathroom: Arc<Bathroom>,
    pub scripts: Arc<HashMap<String, Script>>,
}

impl Home {
    pub fn new(io: &mut IOBuilder) -> Home {
        Home {
            bad_room: Arc::new(BadRoom::new(io)),
            living_room: Arc::new(LivingRoom::new(io)),
            kitchen: Arc::new(Kitchen::new(io)),
            balcony: Arc::new(Balcony::new(io)),
            corridor: Arc::new(Corridor::new(io)),
            toilet: Arc::new(Toilet::new(io)),
            bathroom: Arc::new(Bathroom::new(io)),
            scripts: Arc::new(scripts::scripts_map()),
        }
    }
}

///Living room controller.
#[derive(Debug)]
pub struct LivingRoom {
    //main light
    chandelier: SerialSwitch,
    //light on the treadmill
    cupboard_lamp: SerialSwitch,
    // 2 channel beam.
    beam: WebBeam,
    switch_1: Switch,
    switch_2: Switch,
}

impl LivingRoom {
    pub fn new(io: &mut IOBuilder) -> LivingRoom {
        LivingRoom {
            chandelier: SerialSwitch::new(io, "living_room_lamp", 0x02),
            cupboard_lamp: SerialSwitch::new(io, "cupboard_lamp", 0x06),
            beam: WebBeam::new(io, "lounge_beam"),
            switch_1: Switch::new(io, "lounge_1", LivingRoom::on_switch_1),
            switch_2: Switch::new(io, "lounge_2", LivingRoom::on_switch_2),
        }
    }

    //beam switch
    fn on_switch_1(home: &Home, is_on: bool) -> Result<(), String> {
        home.living_room.beam.switch(is_on);
        Ok(())
    }

    fn on_switch_2(home: &Home, is_on: bool) -> Result<(), String> {
        home.living_room.chandelier.switch(is_on);
        Ok(())
    }
}

///Kitchen room controller.
#[derive(Debug)]
pub struct Kitchen {
    beam: WebBeam,
    kitchen_lamp: SerialDimmer,
    switch_1: Switch,
    switch_2: Switch,
}

impl Kitchen {
    pub fn new(io: &mut IOBuilder) -> Kitchen {
        Kitchen {
            beam: WebBeam::new(io, "kitchen_beam"),
            kitchen_lamp: SerialDimmer::new(io, "kitchen_lamp", 0x04, 1, 100),
            switch_1: Switch::new(io, "kitchen_1", Kitchen::on_kitchen_switch_1),
            switch_2: Switch::new(io, "kitchen_2", Kitchen::on_kitchen_switch_2),
        }
    }

    fn on_kitchen_switch_1(home: &Home, is_on: bool) -> Result<(), String> {
        home.kitchen.kitchen_lamp.set_power(100);
        home.kitchen.kitchen_lamp.switch(is_on);
        Ok(())
    }

    fn on_kitchen_switch_2(home: &Home, is_on: bool) -> Result<(), String> {
        home.kitchen.beam.switch(is_on);
        Ok(())
    }
}

///Balcony room controller.
#[derive(Debug)]
pub struct Balcony {
    //main light
    chandelier: SerialSwitch,
    switch_1: Switch,
    switch_2: Switch,
}

impl Balcony {
    pub fn new(io: &mut IOBuilder) -> Balcony {
        Balcony {
            chandelier: SerialSwitch::new(io, "balcony_lamp", 0x05),
            switch_1: Switch::new(io, "balcony_1", Balcony::on_balcony_switch_1),
            switch_2: Switch::new(io, "balcony_2", Balcony::on_balcony_switch_2),
        }
    }

    fn on_balcony_switch_1(home: &Home, is_on: bool) -> Result<(), String> {
        home.balcony.chandelier.switch(is_on);
        Ok(())
    }

    fn on_balcony_switch_2(home: &Home, is_on: bool) -> Result<(), String> {
        let lamp = &home.kitchen.kitchen_lamp;
        lamp.set_power(1);
        lamp.switch(is_on);
        Ok(())
    }
}

#[derive(Debug)]
pub struct Corridor {
    lamp: SerialDimmer,
    beam: WebBeam,
    exit_1: Switch,
    exit_2: Switch,
    ir_sensor_front_door: Switch,
    ir_sensor_front_1_door: Switch,
    ir_sensor_bedroom_door: Switch,
    ir_sensor_middle: Switch,
    ir_sensor_middle_1: Switch,
    ir_sensor_living_room: Switch,
    ir_sensor_living_room_1: Switch,
    ir: IrHolder,
}

impl Corridor {
    pub fn new(io: &mut IOBuilder) -> Corridor {
        let ir_holder = IrHolder::new(Corridor::ir_handler);

        let ir_front_door = ir_holder.clone();
        let ir_front_door_1 = ir_holder.clone();
        let ir_bedroom_door = ir_holder.clone();
        let ir_middle = ir_holder.clone();
        let ir_middle_1 = ir_holder.clone();
        let ir_living_room = ir_holder.clone();
        let ir_living_room_1 = ir_holder.clone();

        Corridor {
            lamp: SerialDimmer::new(io, "corridor_lamp", 0x03, 1, 100),
            beam: WebBeam::new(io, "corridor_beam"),
            exit_1: Switch::new(io, "exit_1", Corridor::on_exit_1),
            exit_2: Switch::new(io, "exit_2", Corridor::on_exit_2),
            ir_sensor_front_door: Switch::new(io, "ir_sensor_front_door", move |home, is_on| ir_front_door.ir_sensor_front_door(home, is_on)),
            ir_sensor_front_1_door: Switch::new(io, "ir_sensor_front_1_door", move |home, is_on| ir_front_door_1.ir_sensor_front_1_door(home, is_on)),
            ir_sensor_bedroom_door: Switch::new(io, "ir_sensor_bedroom_door", move |home, is_on| ir_bedroom_door.ir_sensor_bedroom_door(home, is_on)),
            ir_sensor_middle: Switch::new(io, "ir_sensor_middle", move |home, is_on| ir_middle.ir_sensor_middle(home, is_on)),
            ir_sensor_middle_1: Switch::new(io, "ir_sensor_middle_1", move |home, is_on| ir_middle_1.ir_sensor_middle_1(home, is_on)),
            ir_sensor_living_room: Switch::new(io, "ir_sensor_living_room", move |home, is_on| ir_living_room.ir_sensor_living_room(home, is_on)),
            ir_sensor_living_room_1: Switch::new(io, "ir_sensor_living_room_1", move |home, is_on| ir_living_room_1.ir_sensor_living_room_1(home, is_on)),
            ir: ir_holder,
        }
    }

    pub fn enable_ir(&self) {
        self.ir.enable_ir();
    }

    pub fn disable_ir(&self) {
        self.ir.disable_ir();
    }

    fn on_exit_1(home: &Home, is_on: bool) -> Result<(), String> {
        home.corridor.beam.switch(is_on);
        Ok(())
    }

    fn on_exit_2(home: &Home, _is_on: bool) -> Result<(), String> {
        let corridor = &home.corridor;
        corridor.exit_1.act(home, ActionType::Off);

        let bad_room = &home.bad_room;
        bad_room.switch_1.act(home, ActionType::Off);
        bad_room.switch_2.act(home, ActionType::Off);

        let bathroom = &home.bathroom;
        bathroom.switch.act(home, ActionType::Off);

        let toilet = &home.toilet;
        toilet.switch.act(home, ActionType::Off);

        let kitchen = &home.kitchen;
        kitchen.switch_1.act(home, ActionType::Off);
        kitchen.switch_2.act(home, ActionType::Off);

        let balcony = &home.balcony;
        balcony.switch_1.act(home, ActionType::Off);
        balcony.switch_2.act(home, ActionType::Off);

        let living_room = &home.living_room;
        living_room.switch_1.act(home, ActionType::Off);
        living_room.switch_2.act(home, ActionType::Off);

        living_room.cupboard_lamp.switch(false);
        Ok(())
    }

    fn calc_power(home: &Home, sensor: SensorName) -> u8 {
        if sensor == SensorName::FrontDoor || sensor == SensorName::BedroomDoor {
            100
        } else {
            let time = Local::now();
            if time.hour() >= 22 || time.hour() <= 5 {
                1
            } else {
                100
            }
        }
    }

    fn ir_handler(home: &Home, is_on: bool, sensor_name: SensorName) {
        if is_on {
            let power = Corridor::calc_power(home, sensor_name);
            home.corridor.lamp.set_power(power);
            home.corridor.lamp.switch(is_on);
        } else {
            home.corridor.lamp.switch(is_on);
        }
    }
}

fn time() -> u128 {
    SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).ok()
        .map(|d| d.as_secs() as u128 * 1000 + d.subsec_millis() as u128)
        .unwrap_or(0)
}

#[derive(Debug)]
struct IrState {
    thread: JoinHandle<()>,
    tx: Sender<IrMessage>,
}

#[derive(Clone, Debug)]
struct IrHolder {
    state: Arc<Mutex<IrState>>,
    is_ir_enable: Arc<AtomicBool>,
}

impl IrHolder {
    pub fn new<A>(act: A) -> IrHolder
        where A: Fn(&Home, bool, SensorName) + Sync + Send + 'static {
        let (tx, rx) = channel();
        let is_ir_enable = Arc::new(AtomicBool::new(true));
        let is_ir_enable_clone = is_ir_enable.clone();
        IrHolder {
            state: Arc::new(Mutex::new(
                IrState {
                    thread: thread::spawn(move || {
                        IrHolder::ir_loop(rx, is_ir_enable_clone, move |home, is_on, sensor_name| {
                            act(home, is_on, sensor_name)
                        })
                    }),
                    tx,
                }
            )),
            is_ir_enable,
        }
    }

    pub fn enable_ir(&self) {
        self.is_ir_enable.store(true, Ordering::SeqCst);
    }

    pub fn disable_ir(&self) {
        self.is_ir_enable.store(false, Ordering::SeqCst);
    }

    fn ir_loop<A>(rx: Receiver<IrMessage>, is_ir_enable: Arc<AtomicBool>, act: A)
        where A: Fn(&Home, bool, SensorName) + Sync + Send + 'static {
        let mut off_time = time();
        let mut is_on = false;
        let mut home: Option<Home> = None;
        let mut sensor_name = SensorName::Middle;

        loop {
            if is_on {
                if let Ok(msg) = rx.recv_timeout(Duration::from_secs(1)) {
                    sensor_name = msg.sensor;
                    off_time = time() + msg.duration as u128;
                    home = Some(msg.home);
                }

                if off_time < time() {
                    is_on = false;
                    act(&home.as_ref().unwrap(), is_on, sensor_name.clone());
                }
            } else {
                if let Ok(msg) = rx.recv() {
                    sensor_name = msg.sensor;
                    is_on = true;
                    home = Some(msg.home);
                    off_time = time() + msg.duration as u128;
                    act(&home.as_ref().unwrap(), is_on, sensor_name.clone());
                }
            }
        }
    }

    pub fn ir_sensor_front_door(&self, home: &Home, is_on: bool) -> Result<(), String> {
        self.send_msg(home, is_on, SensorName::FrontDoor);
        Ok(())
    }

    pub fn ir_sensor_front_1_door(&self, home: &Home, is_on: bool) -> Result<(), String> {
        self.send_msg(home, is_on, SensorName::FrontDoor);
        Ok(())
    }

    fn ir_sensor_bedroom_door(&self, home: &Home, is_on: bool) -> Result<(), String> {
        self.send_msg(home, is_on, SensorName::BedroomDoor);
        Ok(())
    }

    fn ir_sensor_middle(&self, home: &Home, is_on: bool) -> Result<(), String> {
        self.send_msg(home, is_on, SensorName::Middle);
        Ok(())
    }

    fn ir_sensor_middle_1(&self, home: &Home, is_on: bool) -> Result<(), String> {
        self.send_msg(home, is_on, SensorName::Middle);
        Ok(())
    }

    fn ir_sensor_living_room(&self, home: &Home, is_on: bool) -> Result<(), String> {
        self.send_msg(home, is_on, SensorName::LivingRoom);
        Ok(())
    }

    fn ir_sensor_living_room_1(&self, home: &Home, is_on: bool) -> Result<(), String> {
        self.send_msg(home, is_on, SensorName::LivingRoom);
        Ok(())
    }

    fn calc_duration(&self, sensor: &SensorName) -> u128 {
        if sensor == &SensorName::FrontDoor || sensor == &SensorName::BedroomDoor {
            Duration::from_secs(5 * 60).as_millis()
        } else {
            Duration::from_secs(2 * 60).as_millis()
        }
    }

    fn send_msg(&self, home: &Home, _is_on: bool, sensor: SensorName) {
        let time = Local::now();
        if time.hour() > 17 || time.hour() < 10 {
            self.state.lock().unwrap().tx
                .send(IrMessage {
                    duration: self.calc_duration(&sensor),
                    sensor,
                    home: home.clone(),
                });
        } else {
            if sensor == SensorName::FrontDoor || sensor == SensorName::BedroomDoor {
                self.state.lock().unwrap().tx
                    .send(IrMessage {
                        duration: self.calc_duration(&sensor),
                        sensor,
                        home: home.clone(),
                    });
            }
        }
    }
}

struct IrMessage {
    duration: u128,
    sensor: SensorName,
    home: Home,
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Clone)]
enum SensorName {
    LivingRoom,
    Middle,
    BedroomDoor,
    FrontDoor,
}

#[derive(Debug)]
pub struct Toilet {
    lamp: SerialDimmer,
    fun: SerialSwitch,
    switch: Switch,
    timer: RwLock<Timer>,
}

impl Toilet {
    pub fn new(io: &mut IOBuilder) -> Toilet {
        Toilet {
            lamp: SerialDimmer::new(io, "toilet_lamp", 0x02, 25, 100),
            fun: SerialSwitch::new(io, "toilet_fun", 0x03),
            switch: Switch::new(io, "toilet", Toilet::on_switch),
            timer: RwLock::new(Timer::new()),
        }
    }

    fn on_switch(home: &Home, is_on: bool) -> Result<(), String> {
        let toilet = &home.toilet;

        if is_on {
            toilet.fun.switch(false);
            toilet.lamp.switch(true);
            toilet.timer.write().unwrap().reset();
        } else {
            toilet.lamp.switch(false);
            toilet.fun.switch(true);
            let fun = toilet.fun.clone();
            toilet.timer.write().unwrap()
                .after(Duration::from_secs(60 * 2), move || {
                    fun.switch(false);
                });
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct Bathroom {
    lamp: SerialDimmer,
    fun: SerialSwitch,
    hot_water: WebSwitch,
    cold_water: WebSwitch,
    return_water: WebSwitch,
    switch: Switch,
}

impl Bathroom {
    pub fn new(io: &mut IOBuilder) -> Bathroom {
        Bathroom {
            lamp: SerialDimmer::new(io, "bedroom_lamp", 0x01, 20, 100),
            fun: SerialSwitch::new(io, "bathroom_fun", 0x04),
            hot_water: WebSwitch::new(io, "hot_water"),
            cold_water: WebSwitch::new(io, "cold_water"),
            return_water: WebSwitch::new(io, "return_water"),
            switch: Switch::new(io, "bathroom", Bathroom::on_switch),
        }
    }

    fn on_switch(home: &Home, is_on: bool) -> Result<(), String> {
        home.bathroom.lamp.switch(is_on);
        Ok(())
    }
}

#[derive(Debug)]
pub struct BadRoom {
    chandelier: SerialSwitch,
    beam: WebBeam,
    switch_1: Switch,
    switch_2: Switch,
}

impl BadRoom {
    pub fn new(io: &mut IOBuilder) -> BadRoom {
        BadRoom {
            chandelier: SerialSwitch::new(io, "bedroom_lamp", 0x01),
            beam: WebBeam::new(io, "bedroom_beam"),
            switch_1: Switch::new(io, "bedroom_1", BadRoom::on_switch_1),
            switch_2: Switch::new(io, "bedroom_2", BadRoom::on_switch_2),
        }
    }

    fn on_switch_1(home: &Home, is_on: bool) -> Result<(), String> {
        home.bad_room.beam.switch(is_on);
        Ok(())
    }

    fn on_switch_2(home: &Home, is_on: bool) -> Result<(), String> {
        home.bad_room.chandelier.switch(is_on);
        Ok(())
    }
}

impl Runner for Home {
    fn run_script(&self, name: &str) -> Result<(), String> {
        self.scripts.get(name)
            .map(|script| script.run(self))
            .ok_or_else(|| format!("Unknown script: {}", name))
    }
}

mod scripts {
    use home::Home;
    use std::collections::HashMap;
    use std::fmt::{Formatter, Error, Debug};
    use devices::{LedState, LedMode};

    pub struct Script {
        inner: Box<dyn Fn(&Home) + Send + Sync + 'static>
    }

    impl Script {
        fn new<A>(act: A) -> Script where A: Fn(&Home) + Send + Sync + 'static {
            Script { inner: Box::new(act) }
        }

        pub fn run(&self, home: &Home) {
            (self.inner)(home)
        }
    }

    impl Debug for Script {
        fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
            write!(f, "action")
        }
    }

    pub fn scripts_map() -> HashMap<String, Script> {
        let mut map = HashMap::new();
        map.insert("default_color_scheme".to_owned(), Script::new(default_color_scheme));
        map.insert("red_color_scheme".to_owned(), Script::new(red_color_scheme));
        map.insert("low_purple_scheme".to_owned(), Script::new(low_purple_scheme));
        map
    }

    fn all_beam(home: &Home, spot: Option<bool>, led: Option<LedState>) {
        home.bad_room.beam.channel_1(spot, led);
        home.bad_room.beam.channel_2(spot, led);

        home.living_room.beam.channel_1(spot, led);
        home.living_room.beam.channel_2(spot, led);

        home.corridor.beam.channel_1(spot, led);
        home.corridor.beam.channel_2(spot, led);

        home.kitchen.beam.channel_1(spot, led);
        home.kitchen.beam.channel_2(spot, led);
    }

    fn default_color_scheme(home: &Home) {
        all_beam(home, Some(true), Some(LedState::default()));
        home.corridor.enable_ir();
    }

    fn red_color_scheme(home: &Home) {
        all_beam(home, Some(false), Some(LedState { is_on: true, mode: LedMode::Color((255, 0, 0)) }));
        home.corridor.disable_ir();
    }

    fn low_purple_scheme(home: &Home) {
        all_beam(home, Some(false), Some(LedState { is_on: true, mode: LedMode::Color((20, 1, 5)) }));
        home.corridor.disable_ir();
    }
}