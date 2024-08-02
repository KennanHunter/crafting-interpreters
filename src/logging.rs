#[wasm_bindgen::prelude::wasm_bindgen]
#[cfg(target_family = "wasm")]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

#[cfg(target_family = "wasm")]
#[macro_export]
macro_rules! report {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => ($crate::logging::log(&format_args!($($t)*).to_string()))
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
    ($($t:tt)*) => ($crate::logging::log(&("Error:".to_owned() + &format_args!($($t)*).to_string())))
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
    ($($t:tt)*) => ($crate::logging::log(&("Progress:".to_owned() + &format_args!($($t)*).to_string())))
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
