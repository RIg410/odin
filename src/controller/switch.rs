//use controller::{ActionType, DeviceBox};
//use controller::Device;
//
//pub type Action = Fn(ActionType) + Sync + Send + 'static;
//
//pub struct Switch {
//    id: Arc<String>,
//    act: Arc<Action>,
//}
//
//impl Switch {
//    pub fn empty<'a, ID>(id: ID) -> Switch
//        where ID: Into<Cow<'a, str>>, {
//        Switch { id: Arc::new(id.into().to_string()), act: Arc::new(|_| {}) }
//    }
//
//    pub fn lambda<'a, ID, ACT>(id: ID, act: ACT) -> Switch
//        where ID: Into<Cow<'a, str>>,
//              ACT: Fn(ActionType) + Sync + Send + 'static {
//        Switch { id: Arc::new(id.into().to_string()), act: Arc::new(act) }
//    }
//
//    pub fn device<'a, ID>(id: ID, dev: DeviceBox) -> Switch
//        where ID: Into<Cow<'a, str>> {
//        Switch { id: Arc::new(id.into().to_string()), act: Arc::new(move |t| dev.switch(&t)) }
//    }
//
//    pub fn device_toggle<'a, ID>(id: ID, dev: DeviceBox) -> Switch
//        where ID: Into<Cow<'a, str>> {
//        Switch { id: Arc::new(id.into().to_string()), act: Arc::new(move |t| dev.toggle()) }
//    }
//
//    pub fn devices2<'a, ID>(id: ID, dev_1: DeviceBox, dev_2: DeviceBox) -> Switch
//        where ID: Into<Cow<'a, str>> {
//        Switch {
//            id: Arc::new(id.into().to_string()),
//            act: Arc::new(move |t| {
//                dev_1.switch(&t);
//                dev_2.switch(&t);
//            }),
//        }
//    }
//
//    pub fn devices2_toggle<'a, ID>(id: ID, dev_1: DeviceBox, dev_2: DeviceBox) -> Switch
//        where ID: Into<Cow<'a, str>> {
//        Switch {
//            id: Arc::new(id.into().to_string()),
//            act: Arc::new(move |t| {
//                dev_1.toggle();
//                dev_2.toggle();
//            }),
//        }
//    }
//
//    pub fn devices3<'a, ID>(id: ID, dev_1: DeviceBox, dev_2: DeviceBox, dev_3: DeviceBox) -> Switch
//        where ID: Into<Cow<'a, str>> {
//        Switch {
//            id: Arc::new(id.into().to_string()),
//            act: Arc::new(move |t| {
//                dev_1.switch(&t);
//                dev_2.switch(&t);
//                dev_3.switch(&t);
//            }),
//        }
//    }
//
//    pub fn devices3_toggle<'a, ID>(id: ID, dev_1: DeviceBox, dev_2: DeviceBox, dev_3: DeviceBox) -> Switch
//        where ID: Into<Cow<'a, str>> {
//        Switch {
//            id: Arc::new(id.into().to_string()),
//            act: Arc::new(move |t| {
//                dev_1.toggle();
//                dev_2.toggle();
//                dev_3.toggle();
//            }),
//        }
//    }
//
//    pub fn id(&self) -> &str {
//        &self.id
//    }
//
//    pub fn on(&self) {
//        (self.act)(ActionType::On)
//    }
//
//    pub fn off(&self) {
//        (self.act)(ActionType::Off)
//    }
//}
//
//impl Clone for Switch {
//    fn clone(&self) -> Self {
//        Switch {
//            id: self.id.clone(),
//            act: self.act.clone(),
//        }
//    }
//}