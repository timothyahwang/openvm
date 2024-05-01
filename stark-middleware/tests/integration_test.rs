use config::my_poseidon2_config;
use fib_air::trace::generate_trace_rows;
use p3_baby_bear::BabyBear;
use p3_field::AbstractField;
use p3_middleware::prover::committer::trace::TraceCommitter;
use p3_middleware::prover::types::ProvenMultiMatrixAirTrace;
use p3_middleware::prover::PartitionProver;
use p3_uni_stark::StarkGenericConfig;
use tracing_forest::util::LevelFilter;
use tracing_forest::ForestLayer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, Registry};

use crate::config::MyConfig;
use crate::fib_air::air::FibonacciAir;

mod config;
mod fib_air;

#[test]
fn test_single_fib_stark() {
    // Set up tracing:
    let env_filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .from_env_lossy();

    Registry::default()
        .with(env_filter)
        .with(ForestLayer::default())
        .init();

    let (config, mut challenger) = my_poseidon2_config();

    let a = 0u32;
    let b = 1u32;
    let n = 1usize << 3;

    type Val = BabyBear;
    let pis = [a, b, n as u32].map(BabyBear::from_canonical_u32);

    let trace = generate_trace_rows::<Val>(a, b, n);
    let trace_committer = TraceCommitter::<MyConfig>::new(config.pcs());
    let proven_trace = trace_committer.commit(vec![trace]);
    let proven = ProvenMultiMatrixAirTrace {
        trace_data: &proven_trace,
        airs: vec![&FibonacciAir],
    };

    let prover = PartitionProver::new(config);

    prover.prove(&mut challenger, vec![proven], &pis);
}
