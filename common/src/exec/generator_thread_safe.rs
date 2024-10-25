use std::sync::{Arc, Mutex};

use super::generator::Generator;

/// G = real generator
#[derive(Clone)]
pub struct GeneratorThreadSafe<G>
where
    G: Generator + Clone,
{
    generator: Arc<Mutex<G>>,
}

impl<G> GeneratorThreadSafe<G>
where
    G: Generator + Clone,
{
    pub fn new(generator: G) -> Self {
        Self {
            generator: Arc::new(Mutex::new(generator)),
        }
    }

    pub fn get_next(&mut self) -> Option<Vec<G::Item>> {
        let mut guard = self
            .generator
            .lock()
            .unwrap_or_else(|err| panic!("Failed to lock generator with error '{:?}'", err));

        guard.next().map(|x| vec![x])
    }
}
