use super::{generator::Generator, generator_thread_safe::GeneratorThreadSafe};

pub trait Executor<G>
where
    G: Generator + Clone,
{
    /// Return value type
    type Item: Send;

    fn execute(&self, generator: GeneratorThreadSafe<G>) -> Self::Item;
}
