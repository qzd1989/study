use std::{
    sync::{Arc, Mutex, Weak},
    thread::{self, spawn, JoinHandle},
    time::Duration,
};

type DiyFn = dyn Fn() + Sync + Send + 'static;

struct EventCallBack {
    pub callbacks: Mutex<Vec<Weak<DiyFn>>>,
}

impl EventCallBack {
    fn new() -> Self {
        EventCallBack {
            callbacks: Mutex::new(Vec::new()),
        }
    }
}

struct Executor {
    event_callback: Arc<EventCallBack>,
    _handle: JoinHandle<()>,
}

impl Executor {
    fn new() -> Self {
        let event_callback = Arc::new(EventCallBack::new());
        let handle = Self::thread(Arc::downgrade(&event_callback));
        Self {
            event_callback,
            _handle: handle,
        }
    }

    fn thread(event_call_back: Weak<EventCallBack>) -> JoinHandle<()> {
        spawn(move || loop {
            println!(
                "tasks num: {}",
                event_call_back
                    .upgrade()
                    .unwrap()
                    .callbacks
                    .lock()
                    .unwrap()
                    .len()
            );
            if let Some(event_call_back) = event_call_back.upgrade() {
                if let Ok(callbacks) = event_call_back.callbacks.lock() {
                    for callback in callbacks.iter() {
                        match callback.upgrade() {
                            Some(_) => println!("get callback"),
                            None => println!("get none"),
                        }
                    }
                }
            }
            thread::sleep(Duration::from_secs(1));
        })
    }

    //push Fn() to EventCallBack.callbacks
    fn push<T: Fn() + Send + Sync + 'static>(&mut self, callback: T) {
        let callback_arc = Arc::new(callback);
        if let Ok(mut tasks) = self.event_callback.callbacks.lock() {
            let new_callback = Arc::downgrade(&callback_arc);
            tasks.push(new_callback);
        }
    }
}

fn main() {
    let mut executor = Executor::new();
    thread::sleep(Duration::from_secs(3));
    executor.push(|| {
        println!("hello world!");
    });
    thread::sleep(Duration::from_secs(3600));
}
