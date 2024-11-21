
#[derive(Debug, Clone, Copy)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8
}
impl Color {
    pub fn new_hex(val: u32) -> Self {
        let g = (val >> 16 & 0xff) as u8;
        let b = (val >> 8 & 0xff) as u8;
        let a = (val & 0xff) as u8;
        let r = (val >> 24) as u8;
        Self { r, g, b, a }
    }
    pub fn new_argb(a: u8, r:u8, g: u8, b: u8) -> Self {
        Self {a,r,g,b}
    }
    pub fn new_rgbo(r: u8, g: u8, b: u8, o: f64) -> Self {
        assert!(o > 0.0 && o < 1.0);
        Self::new_argb((o * 255.0) as u8, r, g, b)
    }
    pub fn alpha(&self) -> u8 {
        self.a
    }
    pub fn red(&self) -> u8 {
        self.r
    }
    pub fn green(&self) -> u8 {
        self.g
    }
    pub fn blue(&self) -> u8 {
        self.b
    }
    pub fn opacity(&self) -> f64 {
        (self.alpha() as f64) / 255.0
    }
    pub fn with_alpha(mut self, alpha: u8) -> Self {
        self.a = alpha;
        self
    }
    pub fn with_red(mut self, red: u8) -> Self{
        self.r = red;
        self
    }
    pub fn with_green(mut self, green: u8) -> Self {
        self.g = green;
        self
    }
    pub fn with_blue(mut self, blue: u8) -> Self {
        self.b = blue;
        self
    }
    pub fn with_opacity(self, opacity: f64) -> Self {
        assert!(opacity > 0.0 && opacity < 1.0);
        self.with_alpha((opacity * 255.0) as u8)
    }
    pub fn lerp(self, other: Self, t: f64) -> Self {
        let lerp_int = |a: u8, b: u8| ((a as f64) * t + (b as f64) * (1.0-t)) as u8;
        Self::new_argb(
            lerp_int(self.a, other.a), 
            lerp_int(self.r, other.r),
            lerp_int(self.g, other.g),
            lerp_int(self.b, other.b)
        )
    }
    pub fn value(self) -> u32 {
        ((self.r as u32) << 24) | ((self.g as u32) << 16) | ((self.b as u32) << 8) | (self.a as u32)
    }
}