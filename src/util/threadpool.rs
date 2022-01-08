use crossbeam_channel::{bounded, unbounded, Receiver, Sender};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;
use std::time::Duration;

trait Task: Send {
    fn run(self: Box<Self>);
}

impl<F: FnOnce() + Send> Task for F {
    #[inline]
    fn run(self: Box<Self>) {
        (*self)()
    }
}

enum Message {
    Task(Box<dyn Task>),
    Exit,
}

#[derive(Clone)]
struct MessageQueue {
    sender: Sender<Message>,
    receiver: Receiver<Message>,
}

impl MessageQueue {
    fn new() -> Self {
        let (sender, receiver) = unbounded();
        Self { sender, receiver }
    }

    fn new_bounded(cap: usize) -> Self {
        let (sender, receiver) = bounded(cap);
        Self { sender, receiver }
    }

    #[inline]
    fn insert(&self, msg: Message) {
        let _ = self.sender.send(msg);
    }

    #[inline]
    fn remove_all(&self) {
        while self.receiver.recv_timeout(Duration::from_nanos(1)).is_ok() {}
    }

    #[inline]
    fn remove(&self) -> Option<Message> {
        self.receiver.recv().ok()
    }
}

struct Worker {
    queue: MessageQueue,
    exit_sender: Sender<()>,
}

impl Worker {
    fn new(queue: MessageQueue, exit_sender: Sender<()>) -> Self {
        Self { queue, exit_sender }
    }

    fn start(self) {
        thread::spawn(move || {
            while let Some(message) = self.queue.remove() {
                match message {
                    Message::Task(job) => job.run(),
                    Message::Exit => break,
                }
            }

            let _ = self.exit_sender.send(());
        });
    }
}

pub struct Builder {
    num_workers: usize,
    max_num_tasks: Option<usize>,
    on_terminate: Option<Box<dyn Fn()>>,
}

impl Builder {
    pub fn new(num_workers: usize) -> Self {
        Self {
            num_workers,
            max_num_tasks: None,
            on_terminate: None,
        }
    }

    #[must_use]
    pub fn with_bound(mut self, max_num_tasks: Option<usize>) -> Self {
        self.max_num_tasks = max_num_tasks;
        self
    }

    #[must_use]
    pub fn with_on_terminate(mut self, on_terminate: Option<Box<dyn Fn()>>) -> Self {
        self.on_terminate = on_terminate;
        self
    }

    #[must_use]
    pub fn build(self) -> Threadpool {
        Threadpool::new(self.num_workers, self.max_num_tasks, self.on_terminate)
    }
}

pub struct Threadpool {
    num_workers: usize,
    num_active_workers: AtomicUsize,
    queue: MessageQueue,
    exit_sender: Sender<()>,
    exit_receiver: Receiver<()>,
    on_terminate: Option<Box<dyn Fn()>>,
}

impl Threadpool {
    /// Creates a new threadpool.
    ///
    /// # Arguments
    /// - `num_workers`: The number of workers/threads in the pool
    /// - `max_num_jobs`: The optional cap of tasks waiting inside the pool.
    ///                   When full, [Self::execute] blocks until insertion is possible.
    /// - `on_terminate`: The function to call *first* when [Self::terminate] is called.
    pub fn new(
        num_workers: usize,
        max_num_tasks: Option<usize>,
        on_terminate: Option<Box<dyn Fn()>>,
    ) -> Self {
        let (exit_sender, exit_receiver) = bounded(0);

        let queue = match max_num_tasks {
            None => MessageQueue::new(),
            Some(cap) => MessageQueue::new_bounded(cap),
        };

        for _ in 0..num_workers {
            let worker = Worker::new(queue.clone(), exit_sender.clone());
            worker.start();
        }

        Self {
            num_workers,
            num_active_workers: AtomicUsize::new(num_workers),
            queue,
            exit_sender,
            exit_receiver,
            on_terminate,
        }
    }

    /// The number of tasks waiting for execution.
    pub fn waitings_tasks(&self) -> usize {
        self.queue.receiver.len()
    }

    /// The number of workers.
    pub fn workers(&self) -> usize {
        self.num_workers
    }

    /// The number of currently active workers.
    pub fn active_workers(&self) -> usize {
        self.num_active_workers.load(Ordering::SeqCst)
    }

    /// [Self::join] execution and restart all workers.
    pub fn restart(&self) {
        self.join();
        for _ in 0..self.num_workers {
            let worker = Worker::new(self.queue.clone(), self.exit_sender.clone());
            worker.start();
        }
    }

    /// [Self::terminate] execution and restart all workers.
    pub fn force_restart(&self) {
        self.terminate();
        for _ in 0..self.num_workers {
            let worker = Worker::new(self.queue.clone(), self.exit_sender.clone());
            worker.start();
        }
    }

    /// Inserts the given function for execution at the next possible point (LIFO).
    ///
    /// # Arguments
    /// - `f`: The function for execution
    #[inline]
    pub fn execute<F>(&self, f: F)
    where
        F: 'static + FnOnce() + Send,
    {
        let job = Box::new(f);
        let message = Message::Task(job);
        self.queue.insert(message);
    }

    /// Clears all submitted tasks and waits for all workers to exit.
    ///
    /// Calls the `on_terminate` function first, if given during creation of the threadpool.
    pub fn terminate(&self) {
        if let Some(t) = &self.on_terminate {
            t();
        }

        self.queue.remove_all();
        self.join();
    }

    /// Waits for all tasks and workers to finish and exit.
    pub fn join(&self) {
        let num_workers = self.num_active_workers.swap(0, Ordering::SeqCst);
        for _ in 0..num_workers {
            self.queue.insert(Message::Exit);
        }
        for _ in 0..num_workers {
            let _ = self.exit_receiver.recv();
        }
    }
}

impl Drop for Threadpool {
    fn drop(&mut self) {
        self.terminate();
    }
}
