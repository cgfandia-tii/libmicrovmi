use std::io;
use std::io::Write;
use std::panic::catch_unwind;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

use colored::*;
use env_logger;

use common::context::init_context;
use tests::IntegrationTest;

mod common;
mod tests;

fn main() {
    // init logger
    env_logger::builder().is_test(true).init();
    // for each test
    for test in inventory::iter::<IntegrationTest> {
        print!("Test {} ... ", test.name);
        io::stdout().flush().expect("Failed to flush stdout");
        // get setup / teardown context
        let ctx = Arc::new(Mutex::new(init_context()));
        // setup environment before test
        ctx.lock().unwrap().setup();

        // configure test execution in a thread
        let (done_tx, done_rx) = mpsc::channel();
        let thread_ctx = ctx.clone();
        let handle = thread::spawn(move || {
            let val = catch_unwind(move || {
                let (driver, cfg) = {
                    let ctx_lock = thread_ctx.lock().unwrap();
                    (ctx_lock.init_driver(), ctx_lock.config().clone())
                };
                (test.test_fn)(driver, cfg);
            });
            done_tx.send(()).expect("Unable to send completion signal");
            val
        });

        // wait for test to complete until timeout
        let timeout = Duration::from_secs(ctx.lock().unwrap().config().timeout);
        let timeout_result = done_rx.recv_timeout(timeout).map(|_| handle.join());
        // cleanup test environment
        ctx.lock().unwrap().teardown();
        // check results
        match timeout_result {
            Err(_) => println!("{}: {}", "Failed".red(), "Timeout".yellow()),
            Ok(join_result) => match join_result {
                Err(cause) => println!(
                    "{}: test runner failed to join thread: {:?}",
                    "Failed".red(),
                    cause
                ),
                Ok(catch_unwind_result) => match catch_unwind_result {
                    Err(_) => println!("{}", "Failed".red()),
                    Ok(_) => println!("{}", "ok".green()),
                },
            },
        }
    }
}
