use crate::debugger_command::DebuggerCommand;
use crate::inferior::Inferior;
use crate::inferior::Status;
use crate::dwarf_data::{DwarfData, Error as DwarfError};
use nix::sys::ptrace;
use rustyline::error::ReadlineError;
use rustyline::Editor;

pub struct Debugger {
    target: String,
    history_path: String,
    readline: Editor<()>,
    inferior: Option<Inferior>,
    debug_data: DwarfData,
}

impl Debugger {
    /// Initializes the debugger.
    pub fn new(target: &str) -> Debugger {
        let debug_data = match DwarfData::from_file(target) {
            Ok(val) => val,
            Err(DwarfError::ErrorOpeningFile) => {
                println!("could not open file {}", target);
                std::process::exit(1);
            },
            Err(DwarfError::DwarfFormatError(err)) => {
                println!("could not load debugging symbols from {}: {:?}", target, err);
                std::process::exit(1);
            }
        };

        let history_path = format!("{}/.deet_history", std::env::var("HOME").unwrap());
        let mut readline = Editor::<()>::new();
        // Attempt to load history from ~/.deet_history if it exists
        let _ = readline.load_history(&history_path);

        Debugger {
            target: target.to_string(),
            history_path,
            readline,
            inferior: None,
            debug_data,
        }
    }

    pub fn print_status(&self, status: Status) {
        match status {
            Status::Exited(exit_code) => {
                println!("target exited (status {})", exit_code);
            }
            Status::Signaled(signal) => {
                println!("target signaled(killed) by {}", signal.as_str());
            }
            Status::Stopped(signal, rip) => {
                println!(
                    "target stopped at {:#x} by signal {} in {} ({})",
                    rip,
                    signal.as_str(),
                    self.debug_data.get_function_from_addr(rip).unwrap(),
                    self.debug_data.get_line_from_addr(rip).unwrap()
                );
            }
        } 
    }

    pub fn run(&mut self) {
        loop {
            match self.get_next_command() {
                DebuggerCommand::Run(args) => {
                    // make sure no previous target exists
                    if self.inferior.is_some() {
                        match self.inferior.as_mut().unwrap().terminate() {
                            Ok(status) => self.print_status(status),
                            Err(err) => println!("failed to terminate previous target, {}", err)
                        }
                    }

                    if let Some(inferior) = Inferior::new(&self.target, &args) {
                        // Create the inferior
                        self.inferior = Some(inferior);
                        match self.inferior.as_mut().unwrap().cont() {
                            Ok(status) => self.print_status(status),
                            Err(err) => {
                                println!("failed to run command, {}", err);
                            }
                        }
                    } else {
                        println!("Error starting subprocess");
                    }
                }
                DebuggerCommand::Continue => match self.inferior.as_mut().unwrap().cont() {
                    Ok(status) => self.print_status(status),
                    Err(err) => {
                        println!("failed to run command, {}", err);
                    }
                },
                DebuggerCommand::BackTrace => {
                    let _ = self.inferior.as_mut().unwrap().print_backtrace(&self.debug_data);
                }
                DebuggerCommand::Quit => {
                    match self.inferior.as_mut().unwrap().terminate() {
                        Ok(status) => self.print_status(status),
                        Err(err) => {
                            println!("failed to terminate target, {}", err);
                        }
                    }
                    return;
                }
            }
        }
    }

    /// This function prompts the user to enter a command, and continues re-prompting until the user
    /// enters a valid command. It uses DebuggerCommand::from_tokens to do the command parsing.
    ///
    /// You don't need to read, understand, or modify this function.
    fn get_next_command(&mut self) -> DebuggerCommand {
        loop {
            // Print prompt and get next line of user input
            match self.readline.readline("(deet) ") {
                Err(ReadlineError::Interrupted) => {
                    // User pressed ctrl+c. We're going to ignore it
                    println!("Type \"quit\" to exit");
                }
                Err(ReadlineError::Eof) => {
                    // User pressed ctrl+d, which is the equivalent of "quit" for our purposes
                    return DebuggerCommand::Quit;
                }
                Err(err) => {
                    panic!("Unexpected I/O error: {:?}", err);
                }
                Ok(line) => {
                    if line.trim().len() == 0 {
                        continue;
                    }
                    self.readline.add_history_entry(line.as_str());
                    if let Err(err) = self.readline.save_history(&self.history_path) {
                        println!(
                            "Warning: failed to save history file at {}: {}",
                            self.history_path, err
                        );
                    }
                    let tokens: Vec<&str> = line.split_whitespace().collect();
                    if let Some(cmd) = DebuggerCommand::from_tokens(&tokens) {
                        return cmd;
                    } else {
                        println!("Unrecognized command.");
                    }
                }
            }
        }
    }
}
