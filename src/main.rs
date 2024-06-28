use std::{
    sync::{Arc, Mutex, Weak},
    thread::{self, spawn, JoinHandle},
    time::Duration,
};

type DiyFn = dyn Fn() + Sync + Send + 'static;

struct EventCallBack {
    pub callbacks: Mutex<Vec<Arc<DiyFn>>>,
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

    //generate thread
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
                        callback();
                    }
                }
            }
            thread::sleep(Duration::from_secs(1));
        })
    }

    //push new Fn() into EventCallBack.callbacks directly
    fn push<T: Fn() + Send + Sync + 'static>(&mut self, callback: T) {
        let callback_arc = Arc::new(callback);
        let callback_clone = Arc::clone(&callback_arc);
        if let Ok(mut tasks) = self.event_callback.callbacks.lock() {
            tasks.push(callback_clone);
        }
    }
}

fn main() {
    let mut executor = Executor::new();

    thread::sleep(Duration::from_secs(3));
    executor.push(|| {
        println!("hello world!");
    });

    thread::sleep(Duration::from_secs(3));
    executor.push(|| {
        println!("hello world 2!");
    });

    thread::sleep(Duration::from_secs(3600));
}
