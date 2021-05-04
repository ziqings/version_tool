
use std::thread;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;


type Job = Box<dyn FnOnce() + Send + 'static>;

enum Message
{
    NewJob(Job),
    Terminate,
}


struct Worker
{
    //id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

#[warn(non_upper_case_globals)]
static mut GLOBAL_WORK_ID: usize = 10;


impl Worker
{
    fn new(recv: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker
    {
        let id;

        unsafe
        {
            id = GLOBAL_WORK_ID;

            GLOBAL_WORK_ID += 1;
        }


        let thread = thread::spawn(move ||{
            loop
            {
                let msg = recv.lock().unwrap().recv().unwrap();
                match msg
                {
                    Message::NewJob(job) => 
                    {
                        println!("worker thread execute job->{}", id);
                        job();
                    },
                    Message::Terminate =>
                    {
                        break;
                    },
                };
            }
        });

        
        return Worker
        {
            //id: id,
            thread: Some(thread),
        };
    }
}


pub struct ThreadPool
{
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

impl ThreadPool
{
    pub fn new(size: usize) -> ThreadPool
    {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for _ in 0 .. size
        {
            workers.push(Worker::new(Arc::clone(&receiver)));
        }

        return ThreadPool
        {
            workers,
            sender,
        };
    }

    pub fn execute<F>(&self, f: F) where F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool
{
    fn drop(&mut self)
    {
        for _ in &mut self.workers
        {
            self.sender.send(Message::Terminate).unwrap();
        }

        for worker in &mut self.workers
        {
            if let Some(thread) = worker.thread.take()
            {
                thread.join().unwrap();
            }
        }
    }
}


