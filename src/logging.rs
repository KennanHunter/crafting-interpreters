#[cfg(target_family = "wasm")]
#[wasm_bindgen::prelude::wasm_bindgen(module = "/web/src/log.ts")]
extern "C" {
    #[wasm_bindgen]
    pub fn pushToLog(s: &str);
}

#[cfg(target_family = "wasm")]
#[macro_export]
macro_rules! report {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => ($crate::logging::pushToLog(&format_args!($($t)*).to_string()))
}

#[cfg(not(target_family = "wasm"))]
#[macro_export]
macro_rules! report {
    ($($arg:tt)*) => {{
        eprintln!($($arg)*);
    }};
}

#[cfg(target_family = "wasm")]
#[macro_export]
macro_rules! report_error {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => ($crate::logging::pushToLog(&("Error: ".to_owned() + &format_args!($($t)*).to_string())))
}

#[cfg(not(target_family = "wasm"))]
#[macro_export]
macro_rules! report_error {
    () => {
        eprint!("Error: ");
        eprintln!($($arg)*);
    };
    ($($arg:tt)*) => {{
        eprint!("Error: ");
        eprintln!($($arg)*);
    }};
}

#[cfg(target_family = "wasm")]
#[macro_export]
macro_rules! report_progress {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => ($crate::logging::pushToLog(&("Progress: ".to_owned() + &format_args!($($t)*).to_string())))
}

#[cfg(not(target_family = "wasm"))]
#[macro_export]
macro_rules! report_progress {
    () => {
        eprint!("Progress: ");
        eprintln!($($arg)*);
    };
    ($($arg:tt)*) => {{
        eprint!("Progress: ");
        eprintln!($($arg)*);
    }};
}

pub use report;
pub use report_error;
pub use report_progress;
