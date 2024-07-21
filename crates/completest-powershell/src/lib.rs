use std::ffi::OsString;
use std::path::PathBuf;
use std::time::Duration;

pub use completest::Runtime;
pub use completest::RuntimeBuilder;
pub use completest::Term;

/// Abstract factory for [`PowershellRuntime`]
#[derive(Debug)]
#[non_exhaustive]
pub struct PowershellRuntimeBuilder {}

impl RuntimeBuilder for PowershellRuntimeBuilder {
    type Runtime = PowershellRuntime;

    fn name() -> &'static str {
        unimplemented!()
    }

    fn new(bin_root: PathBuf, home: PathBuf) -> std::io::Result<Self::Runtime> {
        unimplemented!()
    }

    fn with_home(
        bin_root: std::path::PathBuf,
        home: std::path::PathBuf,
    ) -> std::io::Result<Self::Runtime> {
        unimplemented!()
    }
}

/// Powershell runtime
#[derive(Debug)]
#[cfg(windows)]
pub struct PowershellRuntime {
    path: OsString,
    home: PathBuf,
    config: PathBuf,
    timeout: Duration,
}

impl PowershellRuntime {
    /// Initialize a new runtime's home
    pub fn new(bin_root: PathBuf, home: PathBuf) -> std::io::Result<Self> {
        unimplemented!()
    }

    /// Reuse an existing runtime's home
    pub fn with_home(bin_root: PathBuf, home: PathBuf) -> std::io::Result<Self> {
        unimplemented!()
    }
}

impl Runtime for PowershellRuntime {
    fn home(&self) -> &std::path::Path {
        unimplemented!()
    }

    fn register(&mut self, name: &str, content: &str) -> std::io::Result<()> {
        unimplemented!()
    }

    fn complete(&mut self, input: &str, term: &Term) -> std::io::Result<String> {
        unimplemented!()
    }
}
