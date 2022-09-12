pub struct Monitor {
    width: u16,
    height: u16
}

impl Monitor {
    pub fn new(w: u16, h: u16) -> Self {
        Monitor {width: w, height: h}
    }

    pub fn width(&self) -> u16 {self.width}
    pub fn height(&self) -> u16 {self.height}
}