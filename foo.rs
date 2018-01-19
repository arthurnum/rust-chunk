use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::string::String;
use std::thread;

#[derive(Clone, Copy)]
struct Bunch {
    _a: u64,
    _b: u64,
    _c: u64,
    _d: u64
}

fn sleep_nop(dur: u32) {
    std::thread::sleep(duration(dur));
}

fn duration(millis: u32) -> std::time::Duration {
    std::time::Duration::from_millis(millis as u64)
}

fn wait_for<T>(h_thread: std::thread::JoinHandle<T>) {
    h_thread.join().expect("Couldn't join on the associated thread");
}

fn foo(dur: u32) -> u32 {
    sleep_nop(dur);
    return 80;
}

fn spawn_my_thread<T>(thread_list: &mut Vec<thread::JoinHandle<()>>, shared: Arc<T>, f: fn(Arc<T>))
where T: Send + 'static + Sync {
    println!("Spawn new thread!");
    let h_thread = std::thread::spawn(move || {
        f(shared);
    });
    thread_list.push(h_thread);
}

fn main() {
    let mut thread_list: Vec<thread::JoinHandle<()>> = Vec::new();
    let cache: HashMap<String, Bunch> = HashMap::new();
    let shared = Arc::new(Mutex::new(cache));
    let shared1 = shared.clone();
    let shared2 = shared.clone();
    let var = 100;

    spawn_my_thread(&mut thread_list, shared.clone(), {
        fn tmp(shared: Arc<Mutex<HashMap<String, Bunch>>>)
        {
            for i in 1..10 {
                foo(1000);
                println!("THREAD_CLOSURE_1 runs every 1000 ms. {}", i);
                let mut hana = shared.lock().unwrap();
                hana.insert(
                    format!("TC1 {}", i),
                    Bunch {
                        _a: 1000000,
                        _b: 1000000,
                        _c: 1000000,
                        _d: 1000000
                    }
                );
            }
        }
        tmp
    });

    let thread_1 = std::thread::spawn(move || {
        for i in 1..10 {
            println!("THREAD_1 runs every 1000 ms. {}, {}", i, foo(1000) + var);
            let mut hana = shared1.lock().unwrap();
            hana.insert(
                format!("T1 {}", i),
                Bunch {
                    _a: 1000000,
                    _b: 1000000,
                    _c: 1000000,
                    _d: 1000000
                }
            );
        }
    });
    thread_list.push(thread_1);

    let thread_2 = std::thread::spawn(move || {
        for i in 1..10 {
            println!("THREAD_2 runs every 500 ms. {}, {}", i, foo(500) + var);
            let mut hana = shared2.lock().unwrap();
            hana.insert(
                format!("T2 {}", i),
                Bunch {
                    _a: 1000000,
                    _b: 1000000,
                    _c: 1000000,
                    _d: 1000000
                }
            );
        }
    });
    thread_list.push(thread_2);

    for thread in thread_list {
        wait_for(thread);
    }

    println!("Main process!");

    for key in shared.lock().unwrap().keys() {
        println!("{}", key);
    }
}
