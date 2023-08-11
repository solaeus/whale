use eframe::{
    egui::{CentralPanel, Context, Layout},
    emath::Align,
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
        let argument = argument.as_table()?.clone();
        let native_options = NativeOptions::default();

        run_native(
            "MyApp",
            native_options,
            Box::new(|cc| Box::new(TableDisplay::new(argument))),
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
    fn update(&mut self, ctx: &Context, frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            let mut table = TableBuilder::new(ui)
                .striped(true)
                .resizable(true)
                .cell_layout(Layout::left_to_right(Align::Center))
                .column(Column::auto())
                .column(Column::initial(100.0).range(40.0..=300.0))
                .column(Column::initial(100.0).at_least(40.0).clip(true))
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
                                    ui.label(cell_data.to_string());
                                });
                            }
                        });
                    }
                });
        });
    }
}
