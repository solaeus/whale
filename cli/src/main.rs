//! Command line interface for the whale programming language.
use std::{
    fs::read_to_string,
    ptr::slice_from_raw_parts,
    time::{Duration, Instant},
};

use clap::Parser;
use nu_ansi_term::{Color, Style};
use reedline::{
    default_emacs_keybindings, ColumnarMenu, Completer, DefaultHinter, DefaultPrompt,
    DefaultPromptSegment, Emacs, FileBackedHistory, KeyCode, KeyModifiers, Prompt, Reedline,
    ReedlineEvent, ReedlineMenu, Signal, Suggestion,
};
use whale::{eval, eval_with_context, FunctionInfo, VariableMap};

/// Command-line arguments to be parsed.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Whale source code to evaluate.
    #[arg(short, long)]
    command: Option<String>,

    /// Location of the file to run.
    #[arg(short, long)]
    path: Option<String>,
}

fn main() {
    let args = Args::parse();

    if let Some(path) = args.path {
        let file_contents = read_to_string(path).unwrap();
        let eval_result = eval(&file_contents).unwrap();

        println!("{eval_result}");
    } else if let Some(command) = args.command {
        let eval_result = eval(&command).unwrap();

        println!("{eval_result}");
    } else {
        run_shell()
    }
}

fn run_shell() {
    let mut context = VariableMap::new(None);
    let mut line_editor = setup_reedline();
    let mut prompt = DefaultPrompt::default();
    prompt.left_prompt = DefaultPromptSegment::WorkingDirectory;
    prompt.right_prompt = DefaultPromptSegment::Basic(" <- ".to_string());

    loop {
        let sig = line_editor.read_line(&prompt);

        match sig {
            Ok(Signal::Success(buffer)) => {
                let start = Instant::now();
                let eval_result = eval_with_context(&buffer, &mut context);
                let time = start.elapsed().as_millis();

                match eval_result {
                    Ok(value) => {
                        if !value.is_empty() {
                            println!("Execution took {time} milliseconds.\n");
                            println!("{}", value)
                        }
                    }
                    Err(error) => eprintln!("{}", error),
                }
            }
            Ok(Signal::CtrlD) | Ok(Signal::CtrlC) => {
                println!("\nAborted!");
                break;
            }
            signal => {
                println!("Event: {:?}", signal);
            }
        }
    }
}

struct WhaleCompeleter;

impl Completer for WhaleCompeleter {
    fn complete(&mut self, line: &str, pos: usize) -> Vec<reedline::Suggestion> {
        whale::BUILTIN_FUNCTIONS
            .into_iter()
            .filter_map(|function| {
                let FunctionInfo {
                    identifier,
                    description,
                } = function.info();

                if identifier.starts_with(line) {
                    Some(Suggestion {
                        value: identifier.to_string(),
                        description: Some(description.to_string()),
                        extra: None,
                        span: reedline::Span { start: 0, end: pos },
                        append_whitespace: true,
                    })
                } else {
                    None
                }
            })
            .collect()
    }
}

fn setup_reedline() -> Reedline {
    let completer = Box::new(WhaleCompeleter);

    let completion_menu = Box::new(
        ColumnarMenu::default()
            .with_name("completion_menu")
            .with_columns(2)
            .with_text_style(Style {
                is_dimmed: false,
                foreground: Some(Color::White),
                ..Default::default()
            })
            .with_description_text_style(Style {
                is_dimmed: true,
                ..Default::default()
            })
            .with_selected_text_style(Style {
                is_bold: true,
                background: Some(Color::Black),
                foreground: Some(Color::White),
                ..Default::default()
            }),
    );

    let mut keybindings = default_emacs_keybindings();
    keybindings.add_binding(
        KeyModifiers::NONE,
        KeyCode::Tab,
        ReedlineEvent::UntilFound(vec![
            ReedlineEvent::Menu("completion_menu".to_string()),
            ReedlineEvent::MenuNext,
        ]),
    );
    keybindings.add_binding(
        KeyModifiers::NONE,
        KeyCode::BackTab,
        ReedlineEvent::UntilFound(vec![
            ReedlineEvent::Menu("completion_menu".to_string()),
            ReedlineEvent::MenuPrevious,
        ]),
    );

    let edit_mode = Box::new(Emacs::new(keybindings));
    let history = Box::new(
        FileBackedHistory::with_file(100, "target/history.txt".into())
            .expect("Error configuring history with file."),
    );

    Reedline::create()
        .with_completer(completer)
        .with_menu(ReedlineMenu::EngineCompleter(completion_menu))
        .with_edit_mode(edit_mode)
        .with_history(history)
        .with_hinter(Box::new(DefaultHinter::default()))
        .with_partial_completions(true)
        .with_quick_completions(true)
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Whale(whale::Error),
    Io(std::io::Error),
    NotYetImplemented,
}

impl From<whale::Error> for Error {
    fn from(value: whale::Error) -> Self {
        Error::Whale(value)
    }
}

impl Into<whale::Error> for Error {
    fn into(self) -> whale::Error {
        whale::Error::CustomMessage(format!("{:?}", self))
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error::Io(value)
    }
}
