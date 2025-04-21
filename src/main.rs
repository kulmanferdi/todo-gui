#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;
use slint::{ModelRc, VecModel, SharedString};
use slint::Model;
use std::{fs, path::Path};
pub use serde::{Serialize, Deserialize};

slint::include_modules!();

#[derive(Serialize, Deserialize)]
struct TodoData {
    items: Vec<String>,
}
fn main() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;
    let ui_handle = ui.as_weak();

    let path = "todo_items.json";
    let todo_items = if Path::new(path).exists() {
        let file_content = fs::read_to_string(path)?;
        let data: TodoData = serde_json::from_str(&file_content)?;
        data.items
    } else {
        vec![]
    };

    let todo_model = std::rc::Rc::new(VecModel::from(
        todo_items.iter().map(|s| SharedString::from(s.as_str())).collect::<Vec<_>>()
    ));
    ui.set_todo_items(ModelRc::from(todo_model.clone()));

    let todo_model_add = todo_model.clone();
    let path_add = path.to_string();
    ui.on_add_item(move |new_item| {
        let ui = ui_handle.unwrap();
        let new_item = new_item.trim();
        if !new_item.is_empty() {
            todo_model_add.push(SharedString::from(new_item));
            save_items_to_file(&todo_model_add, &path_add).unwrap();
            ui.set_new_item("".into());
        }
    });

    let todo_model_remove = todo_model.clone();
    let path_remove = path.to_string();
    ui.on_remove_item(move |idx| {
        if idx >= 0 && (idx as usize) < todo_model_remove.row_count() {
            todo_model_remove.remove(idx as usize);
            save_items_to_file(&todo_model_remove, &path_remove).unwrap();
        }
    });

    ui.run()?;
    Ok(())
}

fn save_items_to_file(model: &VecModel<SharedString>, path: &str) -> Result<(), Box<dyn Error>> {
    let items = (0..model.row_count())
        .map(|i| model.row_data(i).expect("Invalid row").to_string())
        .collect::<Vec<_>>();

    let data = TodoData { items };
    let json = serde_json::to_string_pretty(&data)?;
    fs::write(path, json)?;
    Ok(())
}