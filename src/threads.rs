use std::time::Duration;
use std::thread;

// pub struct Thread {
//     nop_time: Duration
// }
//
// pub fn new() -> Box<Thread> {
//     Box::new(Thread {
//         nop_time: Duration::from_millis(100)
//     })
// }
//
// impl Thread {
//     pub fn run<F>(&self, proc_fn: F)
//         where F: Fn() + Send + 'static {
//         let nop_time = self.nop_time.clone();
//         proc_fn();
//         thread::spawn(move || {
//             loop {
//                 proc_fn();
//                 thread::sleep(nop_time);
//             }
//         });
//     }
    //
    // pub fn proc_fn(&mut self, proc_fn: fn() -> ()) -> &mut Thread {
    //     self.proc_fn = proc_fn;
    //     self
    // }
    //
    // pub fn nop_time(&mut self, ms: u64) -> &mut Thread {
    //     self.nop_time = Duration::from_millis(ms);
    //     self
    // }
// }
