use std::{
    sync::{mpsc, Arc, Mutex},
    thread
};

pub struct ThreadPool{
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    fn new(size: usize) -> ThreadPool {
        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        //Better than Vec::new, as it pre allocates memory, instead of reacting
        let mut workers = Vec::with_capacity(size);

        for id in 0..size{
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender: Some(sender) }
    }

    pub fn build(size: usize) -> ThreadPool {
        match 8 >= size && size >= 1{
            true => ThreadPool::new(size),
            false => panic!("Must have at least 1 and at most 8 threads created.")
        } 
    }

    pub fn execute<F>(&self, f: F)
    where
        // Something is needed as argument here that implements these traits
        F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);

        self.sender.as_ref().unwrap().send(job).unwrap();
    }

} 

impl Drop for ThreadPool{
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                println!("Debug thingy: {:?}", &worker.thread.take());
                thread.join().unwrap();
            }
        }
    }
}

struct Worker{
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker{
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv();

            match message {
                Ok(job) => {
                    println!("Worker {id} got a job; executing.");

                    job();
                }
                Err(_) => {
                    println!("Worker {id} disconnected; shutting down");
                    break;
                }
            }
        });

        Worker {
            id,
            thread: Some(thread)
        }
    }
}
