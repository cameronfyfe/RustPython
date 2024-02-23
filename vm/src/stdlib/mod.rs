#[cfg(feature = "rustpython-ast")]
pub(crate) mod ast;
pub mod atexit;
pub mod builtins;
mod codecs;
mod collections;
pub mod errno;
mod functools;
mod imp;
pub mod io;
mod itertools;
mod marshal;
mod operator;
// TODO: maybe make this an extension module, if we ever get those
// mod re;
mod sre;
mod string;
#[cfg(feature = "rustpython-compiler")]
mod symtable;
mod sysconfigdata;
#[cfg(feature = "threading")]
pub mod thread;
pub mod time;
pub mod warnings;
mod weakref;

#[cfg(any(not(any(target_arch = "wasm32", feature = "wasm-like")), target_os = "wasi"))]
#[macro_use]
pub mod os;
#[cfg(windows)]
pub mod nt;
#[cfg(unix)]
pub mod posix;
#[cfg(any(not(any(target_arch = "wasm32", feature = "wasm-like")), target_os = "wasi"))]
#[cfg(not(any(unix, windows)))]
#[path = "posix_compat.rs"]
pub mod posix;

#[cfg(windows)]
pub(crate) mod msvcrt;
#[cfg(all(unix, not(any(target_os = "android", target_os = "redox"))))]
mod pwd;
#[cfg(not(any(target_arch = "wasm32", feature = "wasm-like")))]
pub(crate) mod signal;
pub mod sys;
#[cfg(windows)]
mod winapi;
#[cfg(windows)]
mod winreg;

use crate::{builtins::PyModule, PyRef, VirtualMachine};
use std::{borrow::Cow, collections::HashMap};

pub type StdlibInitFunc = Box<py_dyn_fn!(dyn Fn(&VirtualMachine) -> PyRef<PyModule>)>;
pub type StdlibMap = HashMap<Cow<'static, str>, StdlibInitFunc, ahash::RandomState>;

pub fn get_module_inits() -> StdlibMap {
    macro_rules! modules {
        {
            $(
                #[cfg($cfg:meta)]
                { $( $key:expr => $val:expr),* $(,)? }
            )*
        } => {{
            let modules = [
                $(
                    $(#[cfg($cfg)] (Cow::<'static, str>::from($key), Box::new($val) as StdlibInitFunc),)*
                )*
            ];
            modules.into_iter().collect()
        }};
    }
    modules! {
        #[cfg(all())]
        {
            "atexit" => atexit::make_module,
            "_codecs" => codecs::make_module,
            "_collections" => collections::make_module,
            "errno" => errno::make_module,
            "_functools" => functools::make_module,
            "itertools" => itertools::make_module,
            "_io" => io::make_module,
            "marshal" => marshal::make_module,
            "_operator" => operator::make_module,
            "_sre" => sre::make_module,
            "_string" => string::make_module,
            "time" => time::make_module,
            "_weakref" => weakref::make_module,
            "_imp" => imp::make_module,
            "_warnings" => warnings::make_module,
            sys::sysconfigdata_name() => sysconfigdata::make_module,
        }
        // parser related modules:
        #[cfg(feature = "rustpython-ast")]
        {
            "_ast" => ast::make_module,
        }
        // compiler related modules:
        #[cfg(feature = "rustpython-compiler")]
        {
            "symtable" => symtable::make_module,
        }
        #[cfg(any(unix, target_os = "wasi"))]
        {
            "posix" => posix::make_module,
            // "fcntl" => fcntl::make_module,
        }
        // disable some modules on WASM
        #[cfg(not(any(target_arch = "wasm32", feature = "wasm-like")))]
        {
            "_signal" => signal::make_module,
        }
        #[cfg(feature = "threading")]
        {
            "_thread" => thread::make_module,
        }
        // Unix-only
        #[cfg(all(unix, not(any(target_os = "android", target_os = "redox"))))]
        {
            "pwd" => pwd::make_module,
        }
        // Windows-only
        #[cfg(windows)]
        {
            "nt" => nt::make_module,
            "msvcrt" => msvcrt::make_module,
            "_winapi" => winapi::make_module,
            "winreg" => winreg::make_module,
        }
    }
}
