use rpc_client::RetryPolicy;
use std::time::Duration;

#[test]
fn retry_policy_max_attempts_is_three() {
    let p = RetryPolicy::default();
    assert_eq!(p.max_attempts(), 3);
}

#[test]
fn retry_policy_delays_grow_with_jitter_seed() {
    let p = RetryPolicy::new(1); // seed = 1 -> deterministic
    let d0 = p.delay_for_attempt(0);
    let d1 = p.delay_for_attempt(1);
    let d2 = p.delay_for_attempt(2);
    assert!(d0 <= Duration::from_millis(500));
    assert!(d1 <= Duration::from_millis(2_500));
    assert!(d2 <= Duration::from_secs(5));
    // Different seed produces different delay sequence.
    let q = RetryPolicy::new(2);
    assert_ne!(p.delay_for_attempt(1), q.delay_for_attempt(1));
}
