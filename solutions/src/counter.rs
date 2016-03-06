use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;

#[derive(Clone)]
pub struct ConcurrentCounter(Arc<RwLock<usize>>);

impl ConcurrentCounter {
    // The constructor should not be surprising.
    pub fn new(val: usize) -> Self {
        ConcurrentCounter(Arc::new(RwLock::new(val)))
    }

    pub fn increment(&self, by: usize) {
        let mut counter = self.0.write().unwrap_or_else(|e| e.into_inner());
        *counter = *counter + by;
    }

    pub fn compare_and_inc(&self, test: usize, by: usize) {
        let mut counter = self.0.write().unwrap_or_else(|e| e.into_inner());
        if *counter == test {
            *counter += by;
        }
    }

    pub fn get(&self) -> usize {
        let counter = self.0.read().unwrap_or_else(|e| e.into_inner());
        *counter
    }
}

// Now our counter is ready for action.
pub fn main() {
    let counter = ConcurrentCounter::new(0);

    // We clone the counter for the first thread, which increments it by 2 every 15ms.
    let counter1 = counter.clone();
    let handle1 = thread::spawn(move || {
        for _ in 0..10 {
            thread::sleep(Duration::from_millis(15));
            counter1.increment(2);
        }
    });

    // The second thread increments the counter by 3 every 20ms.
    let counter2 = counter.clone();
    let handle2 = thread::spawn(move || {
        for _ in 0..10 {
            thread::sleep(Duration::from_millis(20));
            counter2.increment(3);
        }
    });

    // Now we want to watch the threads working on the counter.
    for _ in 0..50 {
        thread::sleep(Duration::from_millis(5));
        println!("Current value: {}", counter.get());
    }

    // Finally, wait for all the threads to finish to be sure we can catch the counter's final value.
    handle1.join().unwrap();
    handle2.join().unwrap();
    println!("Final value: {}", counter.get());
}
