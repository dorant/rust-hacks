use std::sync::{mpsc, Arc, Mutex};

pub struct ThreadPool {
    _handles: Vec<std::thread::JoinHandle<()>>,
    sender: mpsc::Sender<Box<dyn Fn() + Send + 'static>>,
}

impl ThreadPool {
    pub fn new(num_threads: u8) -> Self {
        let (sender, receiver) = mpsc::channel::<Box<dyn Fn() + Send + 'static>>();
        let mutex = Arc::new(Mutex::new(receiver));

        let _handles = (0..num_threads)
            .map(|n| {
                println!("## Create thread: {}", n);
                let rx = mutex.clone();
                std::thread::spawn(move || loop {
                    let received = rx.lock().unwrap().recv();

                    match received {
                        Ok(work) => {
                            println!("## Run thread: {}", n);
                            work();
                        }
                        Err(_) => break,
                    }
                })
            })
            .collect();
        Self { _handles, sender }
    }

    // Takes a function, send:able across threads, with static lifetime
    pub fn execute<T: Fn() + Send + 'static>(&self, work: T) {
        println!("## Execute called");
        self.sender.send(Box::new(work)).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let pool = ThreadPool::new(2);
        let func1 = || {
            println!("FUNCTION 1 RUNNING!");
            std::thread::sleep(std::time::Duration::from_millis(50));
            println!("FUNCTION 1 DONE")
        };
        let func2 = || {
            println!("FUNCTION 2 RUNNING!");
            std::thread::sleep(std::time::Duration::from_millis(50));
            println!("FUNCTION 2 DONE")
        };
        pool.execute(func1);
        pool.execute(func2);
        std::thread::sleep(std::time::Duration::from_millis(1000))
    }
}
