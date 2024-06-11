use std::{
    sync::{mpsc, Arc, Mutex},
    thread
};

pub struct ThreadPool{
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
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

        ThreadPool { workers, sender }
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

        self.sender.send(job).unwrap();
    }

} 

struct Worker{
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker{
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();
    
            println!("Worker is executing new job {id}");

            job();
        });

        Worker { id, thread }
    }
}
