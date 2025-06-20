use crossbeam_channel::{Receiver, Sender, unbounded};
use crossterm::{
    queue,
    style::{Print, Stylize},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use std::{
    io::{Write, stderr},
    sync::{
        Arc, OnceLock,
        atomic::{AtomicBool, Ordering},
    },
    thread,
    time::Duration,
};

pub static LOGGER: OnceLock<Logger> = OnceLock::new();

pub struct Logger {
    tx: Sender<Record>,
    active: Arc<AtomicBool>,
}

pub struct Record {
    pub target: &'static str,
    pub msg: String,
    pub level: RecordLevel,
}

pub enum RecordLevel {
    Info,
}

impl Logger {
    pub fn new() -> Self {
        let (tx, rx) = unbounded();
        let active = Arc::new(AtomicBool::new(true));

        {
            let active = active.clone();
            thread::spawn(move || {
                let _ = infinite_render(rx, active);
            });
        }

        Self { tx, active }
    }

    pub fn print_startup_info(&self, info: StartupInfo) {
        let res_num = info.requested_resources;
        let res_postfix = if res_num == 1 { "" } else { "s" };
        let remote_num = info.requested_remotes;
        let remote_postfix = if remote_num == 1 { "" } else { "s" };
        let pkg_num = info.loaded_packages;
        let pkg_postfix = if pkg_num == 1 { "" } else { "s" };

        let _ = self.tx.send(Record {
            target: "Requested",
            msg: format!(
                "{} resource{} from {} remote{} ({} package{} loaded)",
                res_num, res_postfix, remote_num, remote_postfix, pkg_num, pkg_postfix,
            ),
            level: RecordLevel::Info,
        });
    }
}

#[derive(Default)]
pub struct StartupInfo {
    pub requested_resources: usize,
    pub requested_remotes: usize,
    pub loaded_packages: usize,
}

impl Drop for Logger {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
    }
}

fn infinite_render(rx: Receiver<Record>, active: Arc<AtomicBool>) -> std::io::Result<()> {
    while active.load(Ordering::SeqCst) {
        let Ok(record) = rx.recv() else {
            return Ok(());
        };
        enable_raw_mode()?;

        match record {
            Record {
                target,
                msg,
                level: RecordLevel::Info,
            } => {
                queue!(
                    stderr(),
                    Print(format!("{: >12} {msg}", target.bold().green()))
                )?;
            }
        }

        stderr().flush()?;
        disable_raw_mode()?;
        thread::sleep(Duration::from_millis(50));
    }
    Ok(())
}
