use std::io::{Read, Error};
use std::string::FromUtf8Error;
use std::sync::mpsc::{channel, Receiver, TryRecvError};
use std::time::Duration;
use std::thread::spawn;
use ecs::{LoopStageFlag, SystemTrait, Assembly};
use event::EngineEvent;

#[derive(Debug, Fail)]
enum DebugConsoleError {
    #[fail(display = "{}", _0)]
    IoError(#[cause] Error),
    #[fail(display = "{}", _0)]
    Utf8Error(#[cause] FromUtf8Error),
}

/// Describes a system that captures user input from a stream in a non-blocking fashion (via
/// separate thread) and dispatches each entered line as a vector of command arguments to the event
/// bus. The line splitting algorithm honours whitespace, quoted and escaped input.
pub struct DebugConsole {
    worker_rx: Receiver<Result<String, DebugConsoleError>>,
}

impl DebugConsole {
    /// Given an input stream (e.g. stdin), creates a new `DebugConsole`.
    pub fn new<S>(mut stream: S) -> Self where S: Read + Send + 'static {
        //let mut stream = stdin();
        let (tx, rx) = channel();

        spawn(move || {
            let mut buf = Vec::new();
            let mut byte = [0u8];

            loop {
                match stream.read(&mut byte) {
                    Ok(0) => (),
                    Ok(_) => {
                        if byte[0] == 0x0A {
                            tx.send(match String::from_utf8(buf.clone()) {
                                Ok(l) => Ok(l),
                                Err(e) => Err(DebugConsoleError::Utf8Error(e)),
                            }).expect("Unable to send input from stdin via mpsc channel");
                            buf.clear()
                        } else {
                            buf.push(byte[0])
                        }
                    },
                    Err(e) => tx.send(Err(DebugConsoleError::IoError(e))).expect("Unable to send error information via mpsc channel"),
                }
            }
        });

        DebugConsole {
            worker_rx: rx,
        }
    }
    /// Polls the worker thread for any completed input line.
    fn try_read_line(&self) -> Option<String> {
        match self.worker_rx.try_recv() {
            Ok(Ok(s)) => return Some(s),
            Ok(Err(e)) => warn!("{}", e),
            Err(TryRecvError::Empty) => (),
            Err(e) => error!("{}", e),
        };
        None
    }
    /// Splits a command line (String) into a vector of arguments.
    fn split_arguments(&self, arg_string: &str) -> Vec<String> {
        let mut args = Vec::new();

        let escape_char = '\\';
        let quote_char = '"';

        let mut escape = false;
        let mut in_quote = false;
        let mut had_quote = false;
        let mut prev_char = '\0';
        let mut current_arg = String::new();

        for c in arg_string.chars() {
            if c == escape_char && !escape {
                // The start of an escaped sequence.
                escape = true;
            } else if (c == escape_char || c == quote_char) && escape {
                // Keep the actual escape character if it appears twice.
                // Keep escaped quotes.
                current_arg.push(c);
                escape = false;
            } else if c == quote_char && !escape {
                // Toggle a quoted section.
                in_quote = !in_quote;
                had_quote = true;
                if in_quote && prev_char == quote_char {
                    // Double quotes behave like double escapes in a quoted range.
                    current_arg.push(c);
                }
            } else if c.is_whitespace() && !in_quote {
                // Add the pending escape character.
                if escape {
                    current_arg.push(escape_char);
                    escape = false;
                }
                // Accept empty arguments only if they are quoted
                if !current_arg.is_empty() || had_quote {
                    args.push(current_arg.clone());
                }
                // Reset the current argument
                current_arg.clear();
                had_quote = false;
            } else {
                if escape {
                    // Add the pending escape character
                    current_arg.push(escape_char);
                    escape = false;
                }
                // Copy the character from input without a special meaning
                current_arg.push(c);
            }
            prev_char = c;
        }
        // Save the last argument
        if !current_arg.is_empty() || had_quote {
            args.push(current_arg.clone());
        }

        args
    }
}

impl<F> SystemTrait<EngineEvent, F> for DebugConsole {
    fn get_loop_stage_filter(&self) -> LoopStageFlag {
        LoopStageFlag::UPDATE
    }
    fn update(&mut self, _: &mut Assembly, _: &mut F, _: &Duration, _: &Duration) -> Option<(Vec<EngineEvent>, Vec<EngineEvent>)> {
        self.try_read_line()
            .map(|s| self.split_arguments(&s))
            .map(|c| (Vec::new(), vec![EngineEvent::ConsoleCommand(c)]))
    }
}
