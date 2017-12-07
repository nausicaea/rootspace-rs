use std::collections::HashMap;
use ecs::{EcsEvent, Assembly, EventFlag, HANDLE_EVENT, LoopStageFlag, SystemTrait};
use super::super::event::{CONSOLE_COMMAND, Event};

#[derive(Debug, Fail)]
pub enum DebugShellError {
    #[fail(display = "'{}' is not a recognized builtin or command.", _0)]
    CommandNotFound(String),
}

type ShellResult = Result<Option<Vec<Event>>, DebugShellError>;

pub trait DebugCommand {
    fn run(&self, args: &[String]) -> ShellResult;
}

pub struct DebugShell {
    registry: HashMap<String, Box<DebugCommand>>,
}

impl DebugShell {
    pub fn new() -> DebugShell {
        DebugShell {
            registry: HashMap::new(),
        }
    }
    pub fn add_command<C>(&mut self, name: &str, command: C) where C: DebugCommand + 'static {
        self.registry.insert(name.into(), Box::new(command));
    }
    fn interpret_command(&self, args: &[String]) -> ShellResult {
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
    fn help(&self) -> ShellResult {
        println!("\
                 For more information on a specific command, type COMMAND-NAME --help.\
                 \nhelp\tPrints this message.\
                 \nexit\tShuts down the engine.");
        Ok(None)
    }
    fn exit(&self) -> ShellResult {
        Ok(Some(vec![EcsEvent::Shutdown.into()]))
    }
}

impl SystemTrait<Event> for DebugShell {
    fn get_loop_stage_filter(&self) -> LoopStageFlag {
        HANDLE_EVENT
    }
    fn get_event_filter(&self) -> EventFlag {
        CONSOLE_COMMAND
    }
    fn handle_event(&mut self, _: &mut Assembly, event: &Event) -> Option<Vec<Event>> {
        match *event {
            Event::ConsoleCommand(ref c) => {
                self.interpret_command(&c)
                    .unwrap_or_else(|e| {println!("{}", e); None})
            },
            _ => None,
        }
    }
}
