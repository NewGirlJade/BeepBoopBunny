use alloc::vec::Vec;

struct Counter {
    pressed: bool,
    duration: usize,
    buffer: Vec<()>, //(pressed/released, duration)
}
impl Counter {
    fn new() -> Counter {
        Counter {
            pressed: true,
            duration: 0,
            buffer: Vec::new(),
        }
    }
}

fn should_start_counting(input: bool) {
    let mut counter = Counter::new();
    while true {
        frame_counter(input, &mut counter);
        if !counter.pressed && counter.duration > 60 {
            flush_counter();
            break;
        }
    }
}

fn frame_counter(input: bool, counter: &mut Counter) {
    //don't start counting until all buttons are released
    if counter.buffer.is_empty() && input {
        return;
    }
    if input == counter.pressed {
        counter.duration += 1;
    } else {
        counter.pressed = !counter.pressed;
        counter.buffer.push((counter.pressed, counter.duration));
        counter.duration = 0;
    }
}

fn flush_counter() {}

#[cfg(test)]
mod tests {
    use super::*;
}
