use super::Switchable;

pub struct Lighting {

}

pub struct Stop {

}

struct StopState {

}

impl Switchable for Stop {
    fn is_on(&self) {
        unimplemented!()
    }

    fn is_off(&self) {
        unimplemented!()
    }

    fn on(&self) {
        unimplemented!()
    }

    fn off(&self) {
        unimplemented!()
    }

    fn toggle(&self) {
        unimplemented!()
    }
}