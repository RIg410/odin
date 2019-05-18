use io::IOBuilder;
use devices::{SerialSwitch, WebBeam, SerialDimmer, WebSwitch, Switch as SwitchTrait};
use sensors::Switch;
use timer::Timer;
use std::sync::{RwLock, Arc, Mutex};
use std::time::{Duration, SystemTime};
use chrono::{Local, Timelike};
use std::thread;
use std::thread::JoinHandle;
use std::sync::mpsc::{Sender, channel, Receiver};

#[derive(Debug, Clone)]
pub struct Home {
    pub bad_room: Arc<BadRoom>,
    pub living_room: Arc<LivingRoom>,
    pub kitchen: Arc<Kitchen>,
    pub balcony: Arc<Balcony>,
    pub corridor: Arc<Corridor>,
    pub toilet: Arc<Toilet>,
    pub bathroom: Arc<Bathroom>,
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
        home.living_room.cupboard_lamp.switch(is_on);
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
        home.kitchen.beam.switch(is_on);
        Ok(())
    }

    fn on_kitchen_switch_2(home: &Home, is_on: bool) -> Result<(), String> {
        home.kitchen.kitchen_lamp.switch(is_on);
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
        home.kitchen.kitchen_lamp.switch(is_on);
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
    ir_sensor_bedroom_door: Switch,
    ir_sensor_middle: Switch,
    ir_sensor_middle_1: Switch,
    ir_sensor_living_room: Switch,
    ir_sensor_living_room_1: Switch,
}

impl Corridor {
    pub fn new(io: &mut IOBuilder) -> Corridor {
        let ir_holder = IrHolder::new(Corridor::ir_handler);

        let ir_front_door = ir_holder.clone();
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
            ir_sensor_bedroom_door: Switch::new(io, "ir_sensor_bedroom_door", move |home, is_on| ir_bedroom_door.ir_sensor_bedroom_door(home, is_on)),
            ir_sensor_middle: Switch::new(io, "ir_sensor_middle", move |home, is_on| ir_middle.ir_sensor_middle(home, is_on)),
            ir_sensor_middle_1: Switch::new(io, "ir_sensor_middle_1", move |home, is_on| ir_middle_1.ir_sensor_middle_1(home, is_on)),
            ir_sensor_living_room: Switch::new(io, "ir_sensor_living_room", move |home, is_on| ir_living_room.ir_sensor_living_room(home, is_on)),
            ir_sensor_living_room_1: Switch::new(io, "ir_sensor_living_room_1", move |home, is_on| ir_living_room_1.ir_sensor_living_room_1(home, is_on)),
        }
    }

    fn on_exit_1(home: &Home, is_on: bool) -> Result<(), String> {
        home.corridor.beam.switch(is_on);
        Ok(())
    }

    fn on_exit_2(home: &Home, is_on: bool) -> Result<(), String> {
        let corridor = &home.corridor;
        corridor.beam.switch(false);
        corridor.lamp.switch(false);

        let bad_room = &home.bad_room;
        bad_room.beam.switch(false);
        bad_room.chandelier.switch(false);

        let bathroom = &home.bathroom;
        bathroom.lamp.switch(false);

        let toilet = &home.toilet;
        toilet.lamp.switch(false);

        let kitchen = &home.kitchen;
        kitchen.beam.switch(false);
        kitchen.kitchen_lamp.switch(false);

        let balcony = &home.balcony;
        balcony.chandelier.switch(false);

        let living_room = &home.living_room;
        living_room.chandelier.switch(false);
        living_room.beam.switch(false);
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

struct IrState {
    thread: JoinHandle<()>,
    tx: Sender<IrMessage>,
}

#[derive(Clone)]
struct IrHolder {
    state: Arc<Mutex<IrState>>
}

impl IrHolder {
    pub fn new<A>(act: A) -> IrHolder
        where A: Fn(&Home, bool, SensorName) + Sync + Send + 'static {
        let (tx, rx) = channel();
        IrHolder {
            state: Arc::new(Mutex::new(
                IrState {
                    thread: thread::spawn(move || {
                        IrHolder::ir_loop(rx, move |home, is_on, sensor_name| {
                            act(home, is_on, sensor_name)
                        })
                    }),
                    tx,
                }
            ))
        }
    }

    fn ir_loop<A>(rx: Receiver<IrMessage>, act: A)
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

    fn send_msg(&self, home: &Home, is_on: bool, sensor: SensorName) {
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
            switch: Switch::new(io, "toilet", Bathroom::on_switch),
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
            switch_2: Switch::new(io, "bedroom_1", BadRoom::on_switch_2),
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
