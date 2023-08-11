use eframe::{
    egui::{CentralPanel, Context, Layout, Style},
    emath::Align,
    epaint::Color32,
    run_native, NativeOptions,
};
use egui_extras::{Column, TableBuilder};

use crate::{Macro, MacroInfo, Result, Table, Value};

pub struct Gui;

impl Macro for Gui {
    fn info(&self) -> MacroInfo<'static> {
        MacroInfo {
            identifier: "gui",
            description: "Display a value in a window.",
            group: "gui",
        }
    }

    fn run(&self, argument: &Value) -> Result<Value> {
        let table = Table::from(argument);
        let native_options = NativeOptions::default();

        run_native(
            "MyApp",
            native_options,
            Box::new(|_cc| Box::new(TableDisplay::new(table))),
        )
        .unwrap();

        Ok(Value::Empty)
    }
}

struct TableDisplay(Table);

impl TableDisplay {
    fn new(table: Table) -> Self {
        TableDisplay(table)
    }
}

impl eframe::App for TableDisplay {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            let table = TableBuilder::new(ui)
                .striped(true)
                .resizable(true)
                .cell_layout(Layout::left_to_right(Align::Center))
                .columns(Column::auto(), self.0.column_names().len())
                .column(Column::remainder())
                .min_scrolled_height(0.0);

            table
                .header(20.0, |mut header| {
                    for column_name in self.0.column_names() {
                        header.col(|ui| {
                            ui.strong(column_name.to_string());
                        });
                    }
                })
                .body(|mut body| {
                    for row_data in self.0.rows() {
                        body.row(20.0, |mut row| {
                            for cell_data in row_data {
                                row.col(|ui| {
                                    let color = match cell_data {
                                        Value::String(_) => Color32::GREEN,
                                        Value::Float(_) => todo!(),
                                        Value::Integer(_) => Color32::DARK_RED,
                                        Value::Boolean(_) => todo!(),
                                        Value::List(_) => todo!(),
                                        Value::Map(_) => todo!(),
                                        Value::Table(_) => todo!(),
                                        Value::Function(_) => todo!(),
                                        Value::Empty => todo!(),
                                    };

                                    ui.colored_label(color, cell_data.to_string());
                                });
                            }
                        });
                    }
                });
        });
    }
}
