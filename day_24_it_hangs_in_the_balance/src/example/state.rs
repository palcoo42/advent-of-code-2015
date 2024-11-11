#[derive(Debug, Clone)]
pub struct State<'a> {
    pub sum: u32,
    pub quantum_entanglement: u128,
    pub depth: u32,
    pub remaining_weights: &'a [u32],
}
