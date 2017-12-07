use std::collections::HashMap;
use ecs::{EcsEvent, Assembly, EventFlag, LoopStageFlag, SystemTrait};
use super::super::event::{CONSOLE_COMMAND, EngineEvent};

#[derive(Debug, Fail)]
pub enum DebugShellError {
    #[fail(display = "'{}' is not a recognized builtin or command.", _0)]
    CommandNotFound(String),
}

type ShellResult = Result<Option<Vec<EngineEvent>>, DebugShellError>;

/// Represents a basic shell command.
pub trait CustomCommand {
    /// Executes the command given a set of command line arguments. The first argument refers to
    /// the command name.
    fn run(&self, args: &[String]) -> ShellResult;
}

/// The `DebugShell` listens for ConsoleCommand events and interprets them as commands. The shell
/// provides both builtin commands and the ability to register custom commands through the
/// `CustomCommand` trait.
pub struct DebugShell {
    registry: HashMap<String, Box<CustomCommand>>,
}

impl DebugShell {
    /// Creates a new, default `DebugShell`.
    pub fn new() -> DebugShell {
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
        if args.len() > 0 {
            match args[0].as_str() {
                "help" => self.help(),
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
                 \nexit\tShuts down the engine.");
        Ok(None)
    }
    /// Sends the shutdown event to the bus to exit the engine.
    fn exit(&self) -> ShellResult {
        Ok(Some(vec![EcsEvent::Shutdown.into()]))
    }
}

impl SystemTrait<EngineEvent> for DebugShell {
    fn get_loop_stage_filter(&self) -> LoopStageFlag {
        LoopStageFlag::HANDLE_EVENT
    }
    fn get_event_filter(&self) -> EventFlag {
        CONSOLE_COMMAND
    }
    fn handle_event(&mut self, _: &mut Assembly, event: &EngineEvent) -> Option<Vec<EngineEvent>> {
        match *event {
            EngineEvent::ConsoleCommand(ref c) => self.interpret(&c).unwrap_or_else(|e| {println!("{}", e); None}),
            _ => None,
        }
    }
}
