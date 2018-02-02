mod lighting;
pub use controller::lighting::Lighting;

pub trait Switchable {
    fn is_on(&self);
    fn is_off(&self);
    fn on(&self);
    fn off(&self);
    fn toggle(&self);
}