#![allow(dead_code)]

use std::collections::HashMap;
use std::num::ParseIntError;
use clap::{App, Arg, AppSettings};
use ecs::{Assembly, LoopStageFlag, SystemTrait};
use event::{EngineEventFlag, EngineEvent};

#[derive(Debug, Fail)]
pub enum DebugShellError {
    #[fail(display = "'{}' is not a recognized builtin or command", _0)]
    CommandNotFound(String),
    #[fail(display = "The required argument '{}' is missing for command '{}'", _1, _0)]
    MissingArgument(String, String),
    #[fail(display = "{}", _0)]
    ParseError(#[cause] ParseIntError),
}

impl From<ParseIntError> for DebugShellError {
    fn from(value: ParseIntError) -> Self {
        DebugShellError::ParseError(value)
    }
}

type ShellResult = Result<Option<EngineEvent>, DebugShellError>;

/// Represents a basic shell command.
pub trait CustomCommand {
    /// Executes the command given a set of command line arguments. The first argument refers to
    /// the command name.
    fn run(&self, args: &[String]) -> ShellResult;
}

/// The `DebugShell` listens for `ConsoleCommand` events and interprets them as commands. The shell
/// provides both builtin commands and the ability to register custom commands through the
/// `CustomCommand` trait.
pub struct DebugShell {
    registry: HashMap<String, Box<CustomCommand>>,
}

impl DebugShell {
    /// Creates a new, default `DebugShell`.
    pub fn new() -> Self {
        DebugShell {
            registry: HashMap::new(),
        }
    }
    /// Adds a custom command to the registry.
    pub fn add_command<C>(&mut self, name: &str, command: C) where C: CustomCommand + 'static {
        self.registry.insert(name.into(), Box::new(command));
    }
    /// Removes a custom command from the registry.
    pub fn remove_command(&mut self, name: &str) {
        self.registry.remove(name);
    }
    /// Interprets a set of arguments as a command line (first argument specifies the command
    /// name).
    fn interpret(&self, args: &[String]) -> ShellResult {
        if !args.is_empty() {
            match args[0].as_str() {
                "help" => self.help(),
                "reload-shaders" => self.reload_shaders(),
                "speech-bubble" => self.speech_bubble(args),
                "exit" => self.exit(),
                n => match self.registry.get(n) {
                    Some(c) => c.run(args),
                    None => Err(DebugShellError::CommandNotFound(args[0].clone())),
                },
            }
        } else {
            Ok(None)
        }
    }
    /// Displays all known commands.
    fn help(&self) -> ShellResult {
        println!("\
                 For more information on a specific command, type COMMAND-NAME --help.\
                 \nhelp\tPrints this message.\
                 \nreload-shaders\tReloads all OpenGl shaders in use by the engine.\
                 \nspeech-bubble\tSends a speech bubble event.\
                 \nexit\tShuts down the engine.");
        Ok(None)
    }
    /// Sends the reload-shaders event to the bus.
    fn reload_shaders(&self) -> ShellResult {
        Ok(Some(EngineEvent::ReloadShaders))
    }
    /// Sends a speech-bubble event to the bus.
    fn speech_bubble(&self, args: &[String]) -> ShellResult {
        let matches = App::new("speech-bubble")
            .about("Requests a speech-bubble for the specified target with the specified content.")
            .setting(AppSettings::DisableVersion)
            .arg(Arg::with_name("lifetime")
                 .short("l")
                 .long("lifetime")
                 .takes_value(true)
                 .default_value("5")
                 .help("Determines how long the speech-bubble will live (in seconds)"))
            .arg(Arg::with_name("target")
                 .takes_value(true)
                 .help("Determines the name of the target entity"))
            .arg(Arg::with_name("text")
                 .takes_value(true)
                 .help("Determines the speech-bubble text"))
            .get_matches_from_safe(args);

        match matches {
            Ok(m) => {
                let lifetime: u64 = m.value_of("lifetime")
                    .ok_or_else(|| DebugShellError::MissingArgument(args[0].clone(), "lifetime".into()))
                    .and_then(|s| s.parse().map_err(From::from))?;
                let target = m.value_of("target")
                    .ok_or_else(|| DebugShellError::MissingArgument(args[0].clone(), "target".into()))?;
                let text = m.value_of("text")
                    .ok_or_else(|| DebugShellError::MissingArgument(args[0].clone(), "text".into()))?;

                Ok(Some(EngineEvent::SpeechBubble(target.into(), text.into(), lifetime)))
            },
            Err(e) => {
                println!("{}", e);
                Ok(None)
            },
        }
    }
    /// Sends the shutdown event to the bus to exit the engine.
    fn exit(&self) -> ShellResult {
        Ok(Some(EngineEvent::Shutdown))
    }
}

impl SystemTrait<EngineEvent> for DebugShell {
    fn get_loop_stage_filter(&self) -> LoopStageFlag {
        LoopStageFlag::HANDLE_EVENT
    }
    fn get_event_filter(&self) -> EngineEventFlag {
        EngineEventFlag::CONSOLE_COMMAND
    }
    fn handle_event(&mut self, _: &mut Assembly, event: &EngineEvent) -> Option<EngineEvent> {
        match *event {
            EngineEvent::ConsoleCommand(ref c) => self.interpret(c).unwrap_or_else(|e| {println!("{}", e); None}),
            _ => None,
        }
    }
}
