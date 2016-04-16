extern crate mio;
use mio::{ Handler, EventLoop };
use std::thread;
pub trait Update {
    fn update(&mut self);
}
#[derive(Debug)]
pub struct TimerHandler<T : Update> {
    pub entity : T,
}
impl<T : Update> TimerHandler<T> {
    fn new(t : T) -> TimerHandler<T> {
        TimerHandler {
            entity : t,
        }
    }
}

impl<T : Update> Handler for TimerHandler<T> {
    type Timeout = u64;
    type Message = ();
    fn timeout(&mut self, _event_loop: &mut EventLoop<TimerHandler<T>>, _timeout: Self::Timeout) {
        self.entity.update();
        let _ = _event_loop.timeout_ms(_timeout, _timeout).unwrap();
    }
}

#[derive(Debug)]
pub struct Entity {
    i : usize,
}
impl Entity {
    pub fn new() -> Entity {
        Entity {
            i : 0,
        }
    }
}
impl Update for Entity {
    fn update(&mut self){
        self.i += 1;
        println!("{:?}", self.i);
    }
}

fn main() {
    let e_thread = thread::spawn(move || {
        let mut handler = TimerHandler::new(Entity::new());
        let mut event_loop = mio::EventLoop::new().unwrap();
        let timeout = 1000u64;
        let _ = event_loop.timeout_ms(timeout, timeout).unwrap();
        println!("running timer handler\n\t");
        event_loop.run(&mut handler).unwrap();
    });
     e_thread.join().unwrap();
}
