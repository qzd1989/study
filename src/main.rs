use std::{
    cell::RefCell,
    rc::Rc,
    sync::{Arc, Mutex, Weak},
    thread::{self, sleep, spawn, JoinHandle},
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
        if let Ok(mut tasks) = self.event_callback.callbacks.lock() {
            tasks.push(callback_arc);
        }
    }
}

#[derive(Debug)]
struct MyStruct {
    names: Arc<Mutex<Vec<String>>>,
    _thread: Option<JoinHandle<()>>,
}

impl MyStruct {
    fn new() -> Self {
        let names = Arc::new(Mutex::new(Vec::new()));
        let my_struct = Self {
            names,
            _thread: None,
        };
        let _thread = my_struct.thread(&my_struct.names);
        my_struct
    }
    fn thread(&self, names: &Arc<Mutex<Vec<String>>>) -> JoinHandle<()> {
        let names = Arc::clone(names);
        spawn(move || loop {
            if let Ok(names) = names.lock() {
                for name in names.iter() {
                    println!("names contain: {}", name);
                }
            }
            sleep(Duration::from_secs(1));
        })
    }
}

fn main() {
    //version 1 (String)
    let my = MyStruct::new();
    thread::sleep(Duration::from_secs(3));
    my.names.lock().unwrap().push("zhangsan".to_string());
    thread::sleep(Duration::from_secs(3));
    my.names.lock().unwrap().push("lisi".to_string());

    //version 2 (Fn())
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
