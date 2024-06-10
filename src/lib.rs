use std::thread;

pub struct ThreadPool{
    threads: Vec<thread::JoinHandle<()>>,
}
pub struct ThreadPoolCreationError{
    pub message: String,
}

impl ThreadPoolCreationError{
    pub fn new(message: String) -> ThreadPoolCreationError {
        ThreadPoolCreationError { message }
    } 
}

impl ThreadPool {
    fn new(size: usize) -> ThreadPool {
        let mut threads = Vec::with_capacity(size); //This is more efficient than just doing
        //Vec::new, as it pre allocates memory, instead of reacting

        for _ in 0..size{
            //add threads;
        }

        ThreadPool { threads }
    }

    pub fn build(size: usize) -> Result<ThreadPool, ThreadPoolCreationError> {
        match size > 0{
            true => Ok(ThreadPool::new(size)),
            false => Err(ThreadPoolCreationError {message: "Must have more than one thread created.".to_string()}),
        } 
    }

    pub fn execute<F>(&self, f: F)
    where
        // Something is needed as argument here that implements these traits
        F: FnOnce() + Send + 'static
    {

    }

}
