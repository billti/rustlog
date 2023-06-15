use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::OnceLock;

pub trait LogTelemetry: Sync + Send {
    fn log(&self, msg: &str);
}

// Use the Atomic bool for low-overhead checking if telemetry is enabled before unwrapping the logger
static TELEM_ENABLED: AtomicBool = AtomicBool::new(false);
static TELEM_GLOBAL: OnceLock<&dyn LogTelemetry> = OnceLock::new();

pub fn set_telemetry_logger(logger: &'static dyn LogTelemetry) -> Result<(), &str> {
    TELEM_GLOBAL.set(logger).map_err(|_| {"attempted to set a telemetry logger after it was already initialized"})?;
    TELEM_ENABLED.store(true, Ordering::Relaxed);
    Ok(())
}

#[inline]
pub fn is_telemetry_enabled() -> bool {
    TELEM_ENABLED.load(Ordering::Relaxed)
}

pub fn log_telemetry(msg: &str) {
    if is_telemetry_enabled() {
        if let Some(logger) = TELEM_GLOBAL.get() {
            logger.log(msg);
        }
    }
}
