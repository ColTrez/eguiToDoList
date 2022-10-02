#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::vec;
use eframe::egui;

fn main(){
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "To Do List",
        options,
        Box::new(|_cc| Box::new(ToDoList::default())),
    );
}

struct ToDoList {
    list: Vec<String>,
    new_item: String
}

impl Default for ToDoList{
    fn default() -> Self {
        ToDoList {
            list: Vec::new(),
            new_item: String::new()
        }
    }
}

impl eframe::App for ToDoList{
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame){
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("To Do List");
            let mut num = 1;

            //this tuple is to get around ownership blocking me from updating the list in the following closure
            //there is probably a better way to do this but i am a n00b
            let mut remove_item = (false, 0);

            for item in &self.list{
                ui.horizontal(|ui| {
                    if ui.button("x").clicked(){
                        remove_item = (true, num-1);//num starts at 1 so -1 for correct indexing into the vector
                    }
                    ui.label(num.to_string());
                    ui.label(item);
                    num = num + 1;
                });
            }

            if remove_item.0 {
                self.list.remove(remove_item.1);
                remove_item = (false, 0);
            }

            ui.horizontal(|ui| {
                ui.label("Enter item:");
                ui.text_edit_singleline(&mut self.new_item);
            });
            if ui.button("Add Item").clicked() && self.new_item.len() > 0 {
                self.list.push(self.new_item.clone());
                self.new_item = "".to_string();
            }

        });
    }
}


