use std::thread;

use super::{executor::Executor, generator::Generator, generator_thread_safe::GeneratorThreadSafe};

pub struct Runtime {
    nr_of_threads: usize,
}

impl Runtime {
    pub fn new(nr_of_threads: usize) -> Self {
        Self { nr_of_threads }
    }

    pub fn wait_for_results<G, E>(&self, generator: G, executor: E) -> Vec<E::Item>
    where
        G: Generator + Clone + Send + 'static,   // G: Generator
        E: Executor<G> + Clone + Send + 'static, // E: Executor who executes the task
    {
        // Spawn multiple threads
        let mut handlers = Vec::new();
        let generator_thread_safe = GeneratorThreadSafe::new(generator);

        for _ in 0..self.nr_of_threads {
            let exec = executor.clone();
            let gen = generator_thread_safe.clone();
            let handler = thread::spawn(move || exec.execute(gen));
            handlers.push(handler);
        }

        // Collect results from all the threads
        let mut results: Vec<E::Item> = Vec::new();

        for handler in handlers {
            let result = handler.join().unwrap_or_else(|err| {
                panic!("Failed to join thread with error '{:?}'", err);
            });
            results.push(result);
        }

        results
    }
}
