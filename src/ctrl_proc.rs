use nix::sys::signal::{kill, Signal};
use nix::unistd::Pid;
use scheduler::Which::Process;
use procfs::process::all_processes;
use scheduler::set_priority;


pub fn kill_process(pid: i32) -> Result<(), String> {
    send_signal(pid, Signal::SIGKILL)
}

pub fn terminate_process(pid: i32) -> Result<(), String> {
    send_signal(pid, Signal::SIGTERM)
}

pub fn suspend_process(pid: i32) -> Result<(), String> {
    send_signal(pid, Signal::SIGSTOP)
}

pub fn resume_process(pid: i32) -> Result<(), String> {
    send_signal(pid, Signal::SIGCONT)
}

fn send_signal(pid: i32, signal: Signal) -> Result<(), String> {
    let pid = Pid::from_raw(pid);
    match kill(pid, signal) {
        Ok(_) => Ok(()),
        Err(err) => Err(format!("Failed to send signal to process {}: {}", pid, err)),
    }
}



pub fn killall(process_name: &str) -> Result<(), String> {
    let processes = all_processes().map_err(|err| format!("Failed to list processes: {}", err))?;
    let mut errors = Vec::new();

    for process in processes {
        if let Ok(proc) = process {
            if let Ok(stat) = proc.stat() {
                if stat.comm == process_name {
                    if let Err(err) = terminate_process(stat.pid) {
                        errors.push(err);
                    }
                }
            }
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors.join(", "))
    }
}

pub fn change_priority(pid: i32, priority: i32) -> Result<(), String> {
    
    if priority < -20 || priority > 19 {
        return Err(format!("Invalid priority value: {}. Priority must be between -20 and 19.", priority));
    }

    match set_priority(Process,pid, priority) {
        Ok(_) => Ok(()),
        Err(err) => Err(format!("Failed to set priority to process {}: {:?}", pid, err)),
    }
}
