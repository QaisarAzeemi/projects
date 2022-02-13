use tokio::sync::RwLockWriteGuard;

use crate::test::{run_spec_test, LOCK};

use super::run_v2_test;

#[cfg_attr(feature = "tokio-runtime", tokio::test)]
#[cfg_attr(feature = "async-std-runtime", async_std::test)]
async fn run() {
    let _guard: RwLockWriteGuard<()> = LOCK.run_exclusively().await;
    run_spec_test(&["crud", "v2"], run_v2_test).await;
}
