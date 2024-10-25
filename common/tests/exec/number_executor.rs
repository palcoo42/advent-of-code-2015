use std::{thread::sleep, time::Duration};

use common::exec::{
    executor::Executor, generator::Generator, generator_thread_safe::GeneratorThreadSafe,
};

#[derive(Clone)]
pub struct NumberExecutor {}

impl NumberExecutor {
    pub fn new() -> Self {
        Self {}
    }
}

impl<G> Executor<G> for NumberExecutor
where
    G: Generator<Item = u32> + Clone,
{
    type Item = u32;

    fn execute(&self, mut generator: GeneratorThreadSafe<G>) -> Self::Item {
        let mut value = 0;

        // Fetch data until available
        while let Some(v) = generator.get_next() {
            // Small delay to give all threads time to do something
            sleep(Duration::from_millis(10));
            value = *v.iter().max().unwrap();
        }

        value
    }
}
