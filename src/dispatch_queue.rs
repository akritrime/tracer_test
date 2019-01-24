use std::collections::VecDeque;
use std::thread::{ self, JoinHandle };
use std::sync::{ Mutex, Arc, Condvar };
// use std::time::Duration;

type ThreadVec = Vec<JoinHandle<()>>;

type Queue = VecDeque<Option<Box<Fn() + Send>>>;

type Q = Arc<(Mutex<Queue>, Condvar)>;
// type Flag = Arc<(Condvar, Mutex<bool>)>;

pub struct DispatchQueue {
    threads: ThreadVec,
    pub name: String,
    q: Q
}

impl DispatchQueue {
    pub fn new(name: &str, size: usize) -> DispatchQueue {
        let q: Q = (Mutex::new(Queue::new()), Condvar::new()).into();
        let threads = (0..size)
            .map(|_| q.clone())
            .map(|q| thread::spawn(|| Self::dispatch_thread_handler(q)))
            .collect();
        
        DispatchQueue {
            threads,
            name: name.into(),
            q
        }
    }

    fn dispatch_thread_handler(q: Q) {
        let &(ref q, ref cvar) = &*q;
        loop {
            let op = {
                let mut q = q.lock().unwrap();
                q.pop_front()
            };

            match op {
                Some(op) => match op {
                    Some(op) => op(),
                    None => return ()
                },
                None => {
                    let lock = q.lock().unwrap();
                    let _ = cvar.wait(lock).unwrap();
                }
            };
        }
    }

    pub fn dispatch<F: Fn() + Send + 'static>(&self, op: F) {
        let &(ref q, ref cvar) = &*self.q;
        {
            let mut q = q.lock().unwrap();
            q.push_back(Some(Box::new(op)));
        }
        cvar.notify_all()
    }
}

impl Drop for DispatchQueue {
    fn drop(&mut self) {
        {   
            let &(ref q, ref cvar) = &*self.q;
            let mut q = q.lock().unwrap();
            
            self.threads.iter()
                .for_each(|_| q.push_back(None));
            cvar.notify_all();
        }

        // println!("Shutting down {}", self.name);
        // println!("Signal threads to wrap up.");
        self.threads
            .drain(..)
            .for_each(|t| t.join().unwrap());
        
        // println!("Done")
    }
}
