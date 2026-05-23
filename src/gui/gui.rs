// src/gui.rs
use crate::findit::crawler::crawler;
use crate::findit::filter::filter;
use eframe::egui;
use std::path::Path;

#[derive(PartialEq, Clone, Copy)]
pub enum SearchMode {
    Dir,
    File,
    Word,
}

pub struct Gui {
    search_start: String,
    search_keyword: String,
    search_result: Vec<String>,
    search_mode: SearchMode,
}

pub fn get_search_result(mode: SearchMode, keyword: &str, start: &str) -> Vec<String> {
    let path: &Path = Path::new(start);

    match mode {
        SearchMode::Dir => {
            let dirs = crawler::traverse_dirs(path);
            let filtered_dirs = filter::filter(dirs, keyword);

            filtered_dirs
                .into_iter()
                .map(|p| p.to_string_lossy().into_owned())
                .collect()
        }
        SearchMode::File => {
            let files = crawler::traverse_files(path);
            let filtered_files = filter::filter(files, keyword);

            filtered_files
                .into_iter()
                .map(|p| p.to_string_lossy().into_owned())
                .collect()
        }
        SearchMode::Word => {
            let words = crawler::traverse_words(path);

            words
                .iter()
                .enumerate()
                .filter(|(_, line)| line.contains(keyword))
                .map(|(line_num, line_text)| format!("Line {}: {}", line_num + 1, line_text.trim()))
                .collect()
        }
    }
}

impl Default for Gui {
    fn default() -> Self {
        Self {
            search_start: String::new(),
            search_keyword: String::new(),
            search_result: Vec::new(),
            search_mode: SearchMode::Dir,
        }
    }
}

impl eframe::App for Gui {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show_inside(ui, |ui| {
            ui.heading("Findit");
            ui.add_space(8.0);

            ui.horizontal(|ui| {
                ui.label("Start Directory:");
                ui.text_edit_singleline(&mut self.search_start);
            });

            ui.horizontal(|ui| {
                ui.label("Search Keyword:");
                ui.text_edit_singleline(&mut self.search_keyword);
            });

            ui.add_space(8.0);

            ui.horizontal(|ui| {
                ui.radio_value(&mut self.search_mode, SearchMode::Dir, "Directory");
                ui.radio_value(&mut self.search_mode, SearchMode::File, "File");
                ui.radio_value(&mut self.search_mode, SearchMode::Word, "Word");
            });

            ui.add_space(10.0);

            if ui.button("Search").clicked() {
                if !self.search_start.is_empty() {
                    self.search_result = get_search_result(
                        self.search_mode,
                        &self.search_keyword,
                        &self.search_start,
                    );
                } else {
                    self.search_result = vec!["Please specify a starting directory.".to_string()];
                }
            }

            ui.add_space(10.0);
            ui.separator();
            ui.label("Finds");

            egui::ScrollArea::vertical().show(ui, |ui| {
                for result in &self.search_result {
                    ui.label(result);
                }
            });
        });
    }
}
