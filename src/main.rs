#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;
use slint::{ModelRc, VecModel, SharedString};
use slint::Model;

slint::include_modules!();

fn main() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;
    let ui_handle = ui.as_weak();

    let todo_model = std::rc::Rc::new(VecModel::default());

    ui.set_todo_items(ModelRc::from(todo_model.clone()));

    let todo_model_add = todo_model.clone();
    ui.on_add_item(move |new_item| {
        let ui = ui_handle.unwrap();

        let new_item = new_item.trim();
        if !new_item.is_empty() {
            todo_model_add.push(SharedString::from(new_item));

            ui.set_new_item("".into());
        }
    });

    let todo_model_remove = todo_model.clone();
    ui.on_remove_item(move |idx| {
        if idx >= 0 && (idx as usize) < todo_model_remove.row_count() {
            todo_model_remove.remove(idx as usize);
        }
    });

    ui.run()?;

    Ok(())
}