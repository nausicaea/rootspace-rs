use std::io::{Read, Error};
use std::string::FromUtf8Error;
use std::sync::mpsc::{channel, Receiver, TryRecvError};
use std::time::Duration;
use std::thread::spawn;
use ecs::{LoopStageFlag, SystemTrait, Assembly, DispatchEvents};
use event::EngineEvent;
use singletons::Singletons;
use common::text_manipulation::split_arguments;

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
    escape_char: char,
    quote_char: char,
}

impl DebugConsole {
    /// Given an input stream (e.g. stdin), creates a new `DebugConsole`.
    pub fn new<S>(mut stream: S) -> Self where S: Read + Send + 'static {
        let (tx, rx) = channel();

        // Spawn a new thread that continuously waits for user input from the specified readable
        // stream. This makes it possible to receive user input in a non-blocking fashion.
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
            escape_char: '\\',
            quote_char: '"',
        }
    }
    /// Polls the worker thread for any complete line of user input.
    fn try_read_line(&self) -> Option<String> {
        match self.worker_rx.try_recv() {
            Ok(Ok(s)) => return Some(s),
            Ok(Err(e)) => warn!("{}", e),
            Err(TryRecvError::Empty) => (),
            Err(e) => error!("{}", e),
        };
        None
    }
}

impl SystemTrait<EngineEvent, Singletons> for DebugConsole {
    /// `DebugConsole` has no requirements wrt. the `Assembly`.
    fn verify_requirements(&self, _: &Assembly) -> bool {
        true
    }
    /// `DebugConsole` subscribes to the update call.
    fn get_loop_stage_filter(&self) -> LoopStageFlag {
        LoopStageFlag::UPDATE
    }
    /// Attempts to retrieve data from the worker thread and emits a `ConsoleCommand` event once a
    /// full line of input has been received. Also performs argument splitting before emitting the
    /// event.
    fn update(&mut self, _: &mut Assembly, _: &mut Singletons, _: &Duration, _: &Duration) -> DispatchEvents<EngineEvent> {
         let event = self.try_read_line()
            .map(|s| split_arguments(&s, self.escape_char, self.quote_char))
            .map(|c| vec![EngineEvent::ConsoleCommand(c)]);

         (None, event)
    }
}
