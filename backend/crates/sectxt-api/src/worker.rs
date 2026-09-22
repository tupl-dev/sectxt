use crate::state::AppState;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::interval;

pub fn clean_expired_messages(state: Arc<AppState>, delay_mins: u64) {
    tokio::spawn(async move {
        let mut timer = interval(Duration::from_mins(delay_mins));
        loop {
            timer.tick().await;
            match state.message_repo.clean().await {
                Ok(count) => tracing::info!(count = %count, "cleaned messages"),
                Err(e) => tracing::error!(error = %e, "failed to clean messages"),
            }
        }
    });
}
