pub trait BusDevice {
    fn uuid(&self) -> String;
    fn name(&self) -> String;
}