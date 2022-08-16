use winapi::um::processthreadsapi::{OpenProcess};
use winapi::um::winnt::{PROCESS_SUSPEND_RESUME};
use ntapi::ntpsapi::{NtSuspendProcess, NtResumeProcess};
use sysinfo::{ProcessExt, System, SystemExt};
use winapi::shared::minwindef::{FALSE};
use sysinfo::{Pid, PidExt};
use std::{thread, time};
use std::sync::{Arc, Mutex};
use inputbot::{KeybdKey::*, handle_input_events};
use colored::*;
use promptly::{prompt_default};

pub fn enable_virtual_terminal_processing() {
    // https://stackoverflow.com/questions/63526130/why-do-ansi-escape-codes-sometimes-work-in-cmd
    use winapi_util::console::Console;

    if let Ok(mut term) = Console::stdout() {
        let _ = term.set_virtual_terminal_processing(true);
    }
    if let Ok(mut term) = Console::stderr() {
        let _ = term.set_virtual_terminal_processing(true);
    }
}

fn suspend(pid: Pid) {
    unsafe {
        let handle = OpenProcess(PROCESS_SUSPEND_RESUME, FALSE, pid.as_u32());
        
        if handle == (0 as std::os::windows::io::RawHandle) {
            println!("Failed to open handle to process {}", pid);
        }

        if NtSuspendProcess(handle) != 0 {
            println!("Failed to suspend process {}", pid);
        }
    }   
}

fn resume(pid: Pid) {
    unsafe {
        let handle = OpenProcess(PROCESS_SUSPEND_RESUME, FALSE, pid.as_u32());
        
        if handle == (0 as std::os::windows::io::RawHandle) {
            println!("Failed to open handle to process {}", pid);
        }

        if NtResumeProcess(handle) != 0 {
            println!("Failed to resume process {}", pid);
        }
    }   
}

fn main() {
    enable_virtual_terminal_processing();
    let threshold : usize = prompt_default("Enter CPU usage threshold (0-100): ", 5).unwrap();
    let in_overlay = Arc::new(Mutex::new(false));
    let mut system = System::new();
    let core_count = num_cpus::get();
    let mut suspended : Vec<Pid> = Vec::new();

    {
        let in_overlay = Arc::clone(&in_overlay);
        F2Key.bind(move || {
            if F2Key.is_pressed() {
                let mut overlay_lock = in_overlay.lock().unwrap();
                *overlay_lock = !*overlay_lock;
            }
        });      
    }

    std::thread::spawn(move || {
        handle_input_events();
    }).thread();
    
    loop {
        system.refresh_processes();
        print!("\x1B[2J\x1B[1;1H");
        thread::sleep(time::Duration::from_millis(1000));
        println!("THRESHOLD: {}", threshold);
        println!("PAUSED: {} (PAUSE WITH F2)", if *in_overlay.lock().unwrap() { "YES".green() } else { "NO".red() });
        for process in system.processes_by_exact_name("UplayWebCore.exe") {
            let mut status = "OK".green();
            let cpu = process.cpu_usage() as usize / core_count;
            let overlay = *in_overlay.lock().unwrap();
            if !overlay {
                if cpu > threshold {
                    if suspended.contains(&process.pid()) {
                        status = "SUSPENDED".red();
                    } else {
                        suspend(process.pid());
                        suspended.push(process.pid());
                        status = "SUSPENDING".red().bold();
                    }
                } 

            } else {
                if suspended.contains(&process.pid()) {
                    status = "RESUMING".yellow();
                    resume(process.pid());
                    suspended.retain(|&x| x != process.pid());
                }
            }
            
            println!("{}\t-\t{}%\t\t{}", process.pid(), cpu, status);
        }
    }
}
