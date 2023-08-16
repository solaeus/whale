//! Command line interface for the whale programming language.
use clap::Parser;
use eframe::{
    egui::{CentralPanel, Direction, Layout, RichText, TextStyle},
    emath::Align,
    epaint::{Color32, Stroke},
    run_native, App, NativeOptions,
};
use egui_extras::{Size, StripBuilder};
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

use whale_lib::{
    eval, eval_with_context, Macro, MacroInfo, Result, Value, VariableMap, MACRO_LIST,
};

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

    #[arg(short, long)]
    gui: bool,
}

fn main() {
    let args = Args::parse();

    let eval_result = if let Some(path) = args.path {
        let file_contents = read_to_string(path).unwrap();
        eval(&file_contents)
    } else if let Some(command) = args.command {
        eval(&command)
    } else if args.gui {
        return run_gui_shell();
    } else {
        return run_cli_shell();
    };

    match eval_result {
        Ok(value) => {
            if !value.is_empty() {
                println!("{value}");
            }
        }
        Err(error) => eprintln!("{error}"),
    }
}

pub struct Gui {
    text_edit_buffer: String,
    whale_context: VariableMap,
    eval_results: Vec<Result<Value>>,
}

impl Gui {
    pub fn new() -> Self {
        Gui {
            text_edit_buffer: String::new(),
            whale_context: VariableMap::new(),
            eval_results: Vec::new(),
        }
    }
}

impl App for Gui {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        self.eval_results.truncate(9);

        CentralPanel::default().show(ctx, |ui| {
            ui.style_mut().override_text_style = Some(TextStyle::Heading);

            ui.with_layout(
                Layout {
                    main_dir: Direction::TopDown,
                    main_wrap: false,
                    main_align: Align::Center,
                    main_justify: false,
                    cross_align: Align::Center,
                    cross_justify: true,
                },
                |ui| {
                    ui.text_edit_multiline(&mut self.text_edit_buffer);
                    ui.horizontal(|ui| {
                        let clear = ui.button("clear");
                        let submit = ui.button("submit");

                        if clear.clicked() {
                            self.text_edit_buffer.clear();
                        }

                        if submit.clicked() {
                            let eval_result =
                                eval_with_context(&self.text_edit_buffer, &mut self.whale_context);

                            self.eval_results.insert(0, eval_result);
                        }
                    });
                },
            );
            ui.separator();

            StripBuilder::new(ui)
                .sizes(
                    Size::Absolute {
                        initial: 30.0,
                        range: (1.0, 100.0),
                    },
                    20,
                )
                .vertical(|mut strip| {
                    for result in &self.eval_results {
                        strip.empty();
                        match result {
                            Ok(value) => {
                                strip.cell(|ui| {
                                    let mut rectangle = ui.available_rect_before_wrap();
                                    rectangle.set_height(50.0);

                                    ui.painter().rect_stroke(
                                        rectangle,
                                        1.0,
                                        Stroke::new(2.0, Color32::from_rgb(50, 50, 150)),
                                    );
                                    ui.label(RichText::new(value.to_string()).size(16.0));
                                });
                            }
                            Err(error) => {
                                strip.cell(|ui| {
                                    let mut rectangle = ui.available_rect_before_wrap();
                                    rectangle.set_height(50.0);

                                    ui.painter().rect_stroke(
                                        rectangle,
                                        1.0,
                                        Stroke::new(2.0, Color32::from_rgb(150, 150, 50)),
                                    );
                                    ui.label(RichText::new(error.to_string()).size(16.0));
                                });
                            }
                        }
                    }
                });
        });
    }
}

fn run_gui_shell() {
    run_native(
        "Whale",
        NativeOptions {
            ..Default::default()
        },
        Box::new(|_cc| Box::new(Gui::new())),
    )
    .unwrap();
}

fn run_cli_shell() {
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
                    Ok(value) => println!("{value}"),
                    Err(error) => eprintln!("{error}"),
                }
            }
            Ok(Signal::CtrlD) | Ok(Signal::CtrlC) => {
                println!("\nExit");
                break;
            }
            signal => {
                println!("Unhandled signal: {:?}", signal);
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
            .map(|r#macro| {
                let MacroInfo {
                    identifier,
                    description,
                    group,
                } = r#macro.info();

                let description = format!("{description} | {group}");

                Suggestion {
                    value: identifier.to_string(),
                    description: Some(description),
                    extra: Some(vec![group.to_string()]),
                    ..Default::default()
                }
            })
            .collect();

        self.macro_list
            .sort_by_key(|suggestion| suggestion.extra.clone());

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
        if path.starts_with('\"') {
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
                    value: format!("\"{path}\""),
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
        let start = pos.saturating_sub(current_word.len());
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
            .with_columns(1)
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
