/*

*/
use agb::timer::{Divider, Timer};
const TICKS_PER_SEC: u64 = 16384; // divider1024 ticks at a rate of 16.384kHz
pub struct GameTimer {
    timer_low: Timer,
    timer_hi: Timer, //cascade counter, most significant digits
    last_event: u32,
}
impl GameTimer {
    pub fn new(mut timer_low: Timer, mut timer_hi: Timer) -> GameTimer {
        timer_low.set_enabled(false);
        timer_hi.set_enabled(false);
        timer_low.set_divider(Divider::Divider1024);
        timer_low.set_enabled(true);
        timer_hi.set_cascade(true);
        timer_hi.set_enabled(true);
        GameTimer {
            timer_low,
            timer_hi,
            last_event: 0,
        }
    }
    fn ticks(&self) -> u32 {
        ((self.timer_hi.value() as u32) << 16) | (self.timer_low.value() as u32)
    }
    ///returns miliseconds since previous call without resetting the last_event variable
    pub fn peek_elapsed_ms(&self) -> u32 {
        let now = self.ticks();
        let elapsed_ticks = now.wrapping_sub(self.last_event);
        ((elapsed_ticks as u64 * 1000) / TICKS_PER_SEC) as u32
    }
    ///returns miliseconds since the previous call to this method, or since new() if it's the first call. Sets the last_event variable to the call time
    pub fn new_event(&mut self) -> u32 {
        let now = self.ticks();
        let elapsed_time = self.peek_elapsed_ms();
        self.last_event = now;
        elapsed_time
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use agb::println;
    use agb::timer::Timers;

    struct TestClosure {}
    impl TestClosure {
        fn new() -> TestClosure {
            TestClosure {}
        }
    }

    #[test_case]
    fn test(_gba: &mut agb::Gba) {
        let mut gfx = _gba.graphics.get();
        // let ctx = TestClosure::new();
        let timers = _gba.timers.timers();
        let mut game_timer = GameTimer::new(timers.timer2, timers.timer3);
        let ms_since_last_call = game_timer.new_event();
        println!("{:?}\n", ms_since_last_call);
        let frame = gfx.frame();
        frame.commit();

        let frame = gfx.frame();
        frame.commit();
        let peek_time = game_timer.peek_elapsed_ms();
        println!("{:?}\n", peek_time);

        let frame = gfx.frame();
        frame.commit();
        let ms_since_last_call = game_timer.new_event();
        println!("{:?}\n", ms_since_last_call);
        println!("{:?}\n", game_timer.peek_elapsed_ms());
    }
}
/*
~~~test template~~~
#[test_case]
fn test(_gba:&mut agb::Gba){
let ctx = TestClosure::new();
}

println!("\n{:?}\n",   );
*/
