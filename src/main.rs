//! Command line interface for the whale programming language.
use clap::Parser;
use nu_ansi_term::{Color, Style};
use reedline::{
    default_emacs_keybindings, ColumnarMenu, Completer, DefaultHinter, DefaultPrompt,
    DefaultPromptSegment, EditCommand, Emacs, FileBackedHistory, KeyCode, KeyModifiers, Reedline,
    ReedlineEvent, ReedlineMenu, Signal, Span, Suggestion,
};
use std::{
    fs::{self, read_to_string},
    path::PathBuf,
};
use whale_lib::{eval, eval_with_context, Macro, MacroInfo, VariableMap, MACRO_LIST};

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
    let mut context = VariableMap::new();
    let mut line_editor = setup_reedline();
    let prompt = DefaultPrompt {
        left_prompt: DefaultPromptSegment::WorkingDirectory,
        right_prompt: DefaultPromptSegment::CurrentDateTime,
    };

    loop {
        let sig = line_editor.read_line(&prompt);

        match sig {
            Ok(Signal::Success(buffer)) => {
                let eval_result = eval_with_context(&buffer, &mut context);

                match eval_result {
                    Ok(value) => {
                        if !value.is_empty() {
                            println!("{value}");
                        }
                    }
                    Err(error) => eprintln!("{error}"),
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

struct WhaleCompeleter {
    macro_list: Vec<Suggestion>,
    files: Vec<Suggestion>,
}

impl WhaleCompeleter {
    pub fn new() -> Self {
        WhaleCompeleter {
            macro_list: Vec::new(),
            files: Vec::new(),
        }
    }

    pub fn set_macro_list(&mut self, macro_list: Vec<&'static dyn Macro>) -> &mut Self {
        self.macro_list = macro_list
            .iter()
            .map(|function| {
                let MacroInfo {
                    identifier,
                    description,
                } = function.info();

                Suggestion {
                    value: identifier.to_string(),
                    description: Some(description.to_string()),
                    ..Default::default()
                }
            })
            .collect();

        self
    }

    pub fn get_suggestions(&mut self, start: usize, end: usize) -> Vec<Suggestion> {
        let macro_suggestions = self
            .macro_list
            .iter()
            .cloned()
            .map(|suggestion| Suggestion {
                span: Span { start, end },
                ..suggestion
            });
        let file_suggestions = self.files.iter().cloned().map(|suggestion| Suggestion {
            span: Span { start, end },
            ..suggestion
        });

        file_suggestions.chain(macro_suggestions).collect()
    }

    pub fn update_files(&mut self, mut path: &str) {
        if path.starts_with("\"") {
            path = &path[1..];
        }

        let path = PathBuf::from(path);

        if !path.is_dir() {
            return;
        }

        self.files = fs::read_dir(path)
            .unwrap()
            .map(|entry| {
                let path = entry.unwrap().path();
                let path = path.to_string_lossy();

                Suggestion {
                    value: format!("\"{path}"),
                    description: None,
                    ..Default::default()
                }
            })
            .collect();
    }
}

impl Completer for WhaleCompeleter {
    fn complete(&mut self, line: &str, pos: usize) -> Vec<Suggestion> {
        let split = line.split(' ');
        let current_word = split.last().unwrap_or("");
        let start = pos - current_word.len();
        let end = line.len();

        self.update_files(current_word);
        self.get_suggestions(start, end)
    }
}

fn setup_reedline() -> Reedline {
    let mut completer = Box::new(WhaleCompeleter::new());

    completer.set_macro_list(MACRO_LIST.to_vec());

    let completion_menu = Box::new(
        ColumnarMenu::default()
            .with_name("completion_menu")
            .with_columns(2)
            .with_text_style(Style {
                foreground: Some(Color::White),
                is_dimmed: false,
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
        KeyModifiers::SHIFT,
        KeyCode::Tab,
        ReedlineEvent::UntilFound(vec![
            ReedlineEvent::Menu("completion_menu".to_string()),
            ReedlineEvent::MenuPrevious,
        ]),
    );
    keybindings.add_binding(
        KeyModifiers::ALT,
        KeyCode::Enter,
        ReedlineEvent::Edit(vec![EditCommand::InsertNewline]),
    );

    let edit_mode = Box::new(Emacs::new(keybindings));
    let history = Box::new(
        FileBackedHistory::with_file(100, "target/history.txt".into())
            .expect("Error configuring shell history file."),
    );
    let mut hinter = DefaultHinter::default();

    hinter = hinter.with_style(Style {
        foreground: Some(Color::Yellow),
        ..Default::default()
    });

    Reedline::create()
        .with_completer(completer)
        .with_menu(ReedlineMenu::EngineCompleter(completion_menu))
        .with_edit_mode(edit_mode)
        .with_history(history)
        .with_hinter(Box::new(hinter))
        .with_partial_completions(true)
        .with_quick_completions(true)
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Whale(whale_lib::Error),
    Io(std::io::Error),
    NotYetImplemented,
}

impl From<whale_lib::Error> for Error {
    fn from(value: whale_lib::Error) -> Self {
        Error::Whale(value)
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error::Io(value)
    }
}
