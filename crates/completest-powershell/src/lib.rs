use std::ffi::OsString;
use std::path::PathBuf;
use std::process::Command;
use std::str::FromStr;
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

fn comptest(
    command: Command,
    echo: bool,
    input: &str,
    term: &Term,
    timeout: Duration,
) -> std::io::Result<String> {
    #![allow(clippy::unwrap_used)] // some unwraps need extra investigation

    // spawn a new process, pass it the input was.
    //
    // This triggers completion loading process which takes some time in shell so we should let it
    // run for some time

    let pty_args = winptyrs::PTYArgs {
        cols: term.get_width().into(),
        rows: term.get_height().into(),
        mouse_mode: winptyrs::MouseMode::WINPTY_MOUSE_MODE_NONE,
        timeout: timeout.as_millis() as _,
        agent_config: winptyrs::AgentConfig::WINPTY_FLAG_COLOR_ESCAPES,
    };
    let mut pty = winptyrs::PTY::new_with_backend(&pty_args, winptyrs::PTYBackend::WinPTY).unwrap();

    let mut parser = vt100::Parser::new(term.get_height(), term.get_width(), 0);

    // Spawn a process inside the pseudoterminal.
    pty.spawn(command.get_program().to_os_string(), None, None, None)
        .unwrap();

    let to_write = OsString::from(input);
    pty.write(to_write).unwrap();

    let (snd, rcv) = std::sync::mpsc::channel();
    let shutdown = std::sync::atomic::AtomicBool::new(false);
    let shutdown_ref = &shutdown;

    std::thread::scope(|scope| {
        scope.spawn(move || {
            // since we don't know when exactly shell is done completing the idea is to wait until
            // something at all is produced, then wait for some duration since the last produced chunk.
            rcv.recv().unwrap();
            loop {
                std::thread::sleep(timeout);
                let mut cnt = 0;
                while rcv.try_recv().is_ok() {
                    cnt += 1;
                }
                if cnt == 0 {
                    break;
                }
            }
            shutdown_ref.store(true, std::sync::atomic::Ordering::SeqCst);
        });

        while let Ok(os) = pty.read(2048, false) {
            if shutdown.load(std::sync::atomic::Ordering::SeqCst) {
                // fish clears completions on process teardown
                break;
            }
            let buf = os.to_string_lossy();
            let buf = buf.as_bytes();
            if buf.is_empty() {
                break;
            }
            let _ = snd.send(());
            parser.process(buf);
        }
    });

    let content = parser.screen().contents();
    Ok(content)
}
