## All code  is from tutorial of Tokio official document

1. Hello Tokio
    - async fn
    - #[totio::main]
    - .await

2. Spawning
    - tokio::spwan
    - 'static bound  
    T: 'static means T is bounded by 'static lifetime
    - send bound

3. Sharing data
    - std::sync::Arc
    - std::sync::Mutex 
    - tokio::sync::Mutex An async mutex is a mutex that is locked across calls to .await
    - parking_lot::Mutex eventual fairness
    - current_thread runtime flavor 
    - If contention on a synchronous mutex becomes a problem
        - Switching to a dedicated task to manage state and use message passing.
        - Shard the mutex.
        - Restructure the code to avoid the mutex.

