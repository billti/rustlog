use js_sys::Function;
use log::{debug, error, info, LevelFilter, Record};
use std::cell::OnceCell;
use std::cell::RefCell;
use std::sync::Once;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

mod telemetry;
use crate::telemetry::{is_telemetry_enabled, log_telemetry, set_telemetry_logger, LogTelemetry};

// ******** Test functions *********

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console, js_name=log)]
    fn js_log(msg: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello there, {}", name));
}

#[wasm_bindgen(js_name=doSomething)]
pub fn do_something() {
    debug!("About to log some telemetry");
    log_telemetry("event=something, action=did");
    debug!("Telemetry sent!");
    error!("Are you sure about this!");
}


// ******** LOGGING WebAssembly code *********

static MY_LOGGER: MyLogger = MyLogger;

// We're in Wasm, so only one thread anyway, but needed to avoid errors without Sync trait on RefCell
thread_local! {
    // Will hold a reference to the JS logging function that was passed in
    static LOG_JS_FN: RefCell<Option<Function>> = RefCell::new(None);
}

struct MyLogger;

impl log::Log for MyLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= log::max_level()
    }

    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }
        // We only get here if logging is enabled, and thus there is a function to call, so a
        // call to the JavaScript side is definitely going to happen here. Hence the relative
        // perf cost of unwrapping the thread_local RefCell is probably negligible.
        LOG_JS_FN.with(|f| {
            let fnborrow = f.borrow();
            if let Some(js_fn) = fnborrow.as_ref() {
                let msg = format!("{}", record.args());
                let _ = js_fn.call1(&JsValue::NULL, &JsValue::from(msg));
            }
        });
    }

    fn flush(&self) {}
}

pub fn hook(_info: &std::panic::PanicInfo) {
    // TODO: See example at https://github.com/rustwasm/console_error_panic_hook/blob/master/src/lib.rs#L97
    // Should write panic details to the configured logger
}

#[wasm_bindgen(js_name=initLogging)]
pub fn init_logging(callback: JsValue, level: i32) -> Result<(), JsValue> {
    if !callback.is_function() {
        return Err(JsError::new("Invalid callback").into());
    }

    if !(0..=5).contains(&level) {
        return Err(JsError::new("Invalid logging level").into());
    }

    let thefn: Function = callback.dyn_into().unwrap(); // Already checked it was a function
    LOG_JS_FN.with(|f| {
        *f.borrow_mut() = Option::Some(thefn);
    });

    // TODO: Maybe replace this with just checking if the static option is Some.
    // (And do that first and fail if already set)
    use std::sync::Once;
    static INIT_ONCE: Once = Once::new();
    INIT_ONCE.call_once(|| {
        log::set_logger(&MY_LOGGER).unwrap();
        std::panic::set_hook(Box::new(hook));
    });

    set_log_level(level);
    Ok(())
}

#[wasm_bindgen(js_name=setLogLevel)]
pub fn set_log_level(level: i32) {
    log::set_max_level(match level {
        1 => LevelFilter::Error,
        2 => LevelFilter::Warn,
        3 => LevelFilter::Info,
        4 => LevelFilter::Debug,
        5 => LevelFilter::Trace,
        _ => LevelFilter::Off,
    });
    info!("Log level set to {}", level);
}


// ************ TELEMETRY WebAssembly code **************

// Holds a reference to the JavaScript function to call (which must be thread specific)
thread_local! {
    static TELEM_FN: OnceCell<Function> = OnceCell::new();
}

// The global logger that delegates to the thread local JS function (if present and enabled)
struct WasmTelemetryLogger;
impl LogTelemetry for WasmTelemetryLogger {
    fn log(&self, msg: &str) {
        if is_telemetry_enabled() {
            TELEM_FN.with(|f| {
                if let Some(jsfn) = f.get() {
                    let _ = jsfn.call1(&JsValue::NULL, &JsValue::from_str(msg));
                }
            });
        }
    }
}
static WASM_TELEMETRY_LOGGER: WasmTelemetryLogger = WasmTelemetryLogger;

#[wasm_bindgen(js_name=initTelemetry)]
pub fn init_telemetry(callback: JsValue) -> Result<(), JsValue> {
    // Ensure a function was passed, and set it in the thread local storage
    if !callback.is_function() {
        return Err(JsError::new("Invalid callback").into());
    }

    let thefn: Function = callback.dyn_into().unwrap();
    TELEM_FN.with(|f| f.set(thefn))?;

    // Ensure that the global logger is set (at most once).
    static INIT_ONCE: Once = Once::new();
    INIT_ONCE.call_once(|| {
        set_telemetry_logger(&WASM_TELEMETRY_LOGGER);
    });

    Ok(())
}
