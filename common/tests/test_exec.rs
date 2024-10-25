mod exec;

use common::exec::runtime::Runtime;
use exec::{number_executor::NumberExecutor, number_generator::NumberGenerator};

#[test]
fn test_runtime() {
    const NR_OF_THREADS: usize = 4;
    const NR_OF_ITEMS: usize = 42;

    let runtime = Runtime::new(NR_OF_THREADS);
    let generator = NumberGenerator::new(NR_OF_ITEMS as u32);
    let executor = NumberExecutor::new();

    let results = runtime.wait_for_results(generator, executor);

    assert_eq!(results.len(), 4);
    assert_eq!(*results.iter().max().unwrap(), NR_OF_ITEMS as u32 - 1);
}
