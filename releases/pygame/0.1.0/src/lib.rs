//! Portable software rendering and game primitives; no OS or Python dependencies.
pub type Color = [u8; 4];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Rect { pub x: i32, pub y: i32, pub w: u32, pub h: u32 }
impl Rect {
    pub fn contains(self, x: i32, y: i32) -> bool {
        let (x,y) = (i64::from(x), i64::from(y));
        x >= i64::from(self.x) && x < i64::from(self.x)+i64::from(self.w)
            && y >= i64::from(self.y) && y < i64::from(self.y)+i64::from(self.h)
    }
    pub fn intersection(self, other: Self) -> Option<Self> {
        let x = self.x.max(other.x);
        let y = self.y.max(other.y);
        let r = (i64::from(self.x)+i64::from(self.w)).min(i64::from(other.x)+i64::from(other.w));
        let b = (i64::from(self.y)+i64::from(self.h)).min(i64::from(other.y)+i64::from(other.h));
        if r <= i64::from(x) || b <= i64::from(y) { return None; }
        Some(Self { x,y,w:(r-i64::from(x)) as u32,h:(b-i64::from(y)) as u32 })
    }
    pub fn collides(self, other: Self) -> bool { self.intersection(other).is_some() }
}
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Vec2 { pub x: f64, pub y: f64 }
impl Vec2 {
    pub fn length(self) -> f64 { self.x.hypot(self.y) }
    pub fn normalized(self) -> Self {
        let n = self.length();
        if n == 0.0 || !n.is_finite() { Self::default() } else { Self {x:self.x/n,y:self.y/n} }
    }
}
#[derive(Clone, Debug)]
pub struct Surface { width: u32, height: u32, pixels: Vec<Color> }
impl Surface {
    pub fn new(width: u32, height: u32) -> Result<Self, &'static str> {
        if width == 0 || height == 0 || width > i32::MAX as u32 || height > i32::MAX as u32 { return Err("invalid dimensions"); }
        let n = (width as usize).checked_mul(height as usize).ok_or("surface too large")?;
        if n > 16_777_216 { return Err("surface too large"); }
        let mut pixels = Vec::new();
        pixels.try_reserve_exact(n).map_err(|_| "allocation failed")?;
        pixels.resize(n,[0,0,0,0]);
        Ok(Self {width,height,pixels})
    }
    pub fn size(&self) -> (u32,u32) { (self.width,self.height) }
    pub fn pixels(&self) -> &[Color] { &self.pixels }
    fn bounds(&self) -> Rect { Rect{x:0,y:0,w:self.width,h:self.height} }
    fn index(&self,x:i32,y:i32) -> Option<usize> {
        if self.bounds().contains(x,y) { Some(y as usize*self.width as usize+x as usize) } else { None }
    }
    pub fn get(&self,x:i32,y:i32) -> Option<Color> { self.index(x,y).map(|i|self.pixels[i]) }
    pub fn set(&mut self,x:i32,y:i32,c:Color) { if let Some(i)=self.index(x,y) { self.pixels[i]=c; } }
    pub fn fill(&mut self,c:Color) { self.pixels.fill(c); }
    pub fn rect(&mut self,r:Rect,c:Color) {
        if let Some(r)=self.bounds().intersection(r) {
            for y in r.y as usize..r.y as usize+r.h as usize {
                let start=y*self.width as usize+r.x as usize;
                self.pixels[start..start+r.w as usize].fill(c);
            }
        }
    }
    pub fn circle(&mut self,cx:i32,cy:i32,radius:u32,color:Color) {
        let r=i64::from(radius); let cx=i64::from(cx); let cy=i64::from(cy);
        let l=(cx-r).max(0); let t=(cy-r).max(0);
        let right=(cx+r).min(i64::from(self.width)-1); let bottom=(cy+r).min(i64::from(self.height)-1);
        for y in t..=bottom { for x in l..=right {
            let dx=i128::from(x-cx); let dy=i128::from(y-cy);
            if dx*dx+dy*dy <= i128::from(r)*i128::from(r) { self.set(x as i32,y as i32,color); }
        }}
    }
    /// Copies RGBA pixels verbatim, with clipping (no alpha blending).
    pub fn blit(&mut self,source:&Surface,x:i32,y:i32) {
        let dst=Rect{x,y,w:source.width,h:source.height};
        if let Some(r)=self.bounds().intersection(dst) {
            for dy in r.y as usize..r.y as usize+r.h as usize {
                let sy=(dy as i64-i64::from(y)) as usize;
                let sx=(i64::from(r.x)-i64::from(x)) as usize;
                let a=sy*source.width as usize+sx;
                let b=dy*self.width as usize+r.x as usize;
                self.pixels[b..b+r.w as usize].copy_from_slice(&source.pixels[a..a+r.w as usize]);
            }
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub enum Event { Quit, KeyDown(String), KeyUp(String), PointerMove(i32,i32), PointerDown(u8), PointerUp(u8) }
#[derive(Default)]
pub struct EventQueue(std::collections::VecDeque<Event>);
impl EventQueue {
    pub fn push(&mut self,event:Event) { self.0.push_back(event); }
    pub fn poll(&mut self)->Option<Event> { self.0.pop_front() }
}
/// Host supplies elapsed seconds, so the clock works even without OS timers.
pub struct Clock { step:f64, accumulator:f64 }
impl Clock {
    pub fn new(fps:u32)->Option<Self> { if fps==0 { None } else { Some(Self{step:1.0/f64::from(fps),accumulator:0.0}) } }
    pub fn advance(&mut self,seconds:f64)->u32 {
        if !seconds.is_finite() || seconds<0.0 { return 0; }
        self.accumulator=(self.accumulator+seconds).min(self.step*8.0);
        let count=(self.accumulator/self.step).floor() as u32;
        self.accumulator-=f64::from(count)*self.step;
        count
    }
    pub fn step_seconds(&self)->f64 { self.step }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn collision_edges() { let a=Rect{x:0,y:0,w:5,h:5}; assert!(!a.collides(Rect{x:5,..a})); assert!(a.collides(Rect{x:4,..a})); assert!(!a.collides(Rect{w:0,..a})); }
    #[test] fn clipped_rendering() { let mut s=Surface::new(4,4).unwrap(); let red=[255,0,0,255]; s.rect(Rect{x:-2,y:-2,w:4,h:4},red); assert_eq!(s.get(0,0),Some(red)); assert_eq!(s.get(2,2),Some([0;4])); s.circle(3,3,1,red); assert_eq!(s.get(3,3),Some(red)); assert_eq!(s.get(-1,0),None); }
    #[test] fn blit_clips() { let mut a=Surface::new(2,2).unwrap(); a.set(1,1,[1,2,3,4]); let mut b=Surface::new(2,2).unwrap(); b.blit(&a,-1,-1); assert_eq!(b.get(0,0),Some([1,2,3,4])); }
    #[test] fn queue_and_clock() { let mut q=EventQueue::default(); q.push(Event::Quit); assert_eq!(q.poll(),Some(Event::Quit)); assert_eq!(q.poll(),None); let mut c=Clock::new(10).unwrap(); assert_eq!(c.advance(0.05),0); assert_eq!(c.advance(0.05),1); assert_eq!(c.advance(100.0),8); }
}
