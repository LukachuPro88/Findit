//! # gui
//!
//! Desktop GUI interface for findit using [`eframe`] and [`egui`].

use crate::findit::crawler::crawler;
use crate::findit::filter::filter;
use eframe::egui;
use std::path::Path;
use std::sync::mpsc::{Receiver, channel};

/// The search mode selected in the GUI.
#[derive(PartialEq, Clone, Copy)]
pub enum SearchMode {
    /// Search for directories by name.
    Dir,
    /// Search for files by name.
    File,
    /// Search for a word within a file.
    Word,
}

/// The main GUI application state.
pub struct Gui {
    /// The starting path for the search.
    search_start: String,
    /// The keyword or word to search for.
    search_keyword: String,
    /// The list of results from the last search.
    search_result: Vec<String>,
    /// The current search mode.
    search_mode: SearchMode,
    rx: Option<Receiver<Vec<String>>>,
    is_searching: bool,
}

/// Performs a search based on `mode` and returns the results as strings.
///
/// - [`SearchMode::Dir`] and [`SearchMode::File`] return matching paths.
/// - [`SearchMode::Word`] returns matching lines formatted as `Line N: content`.
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
            let raw_lines = crawler::traverse_words(path);
            filter::filter_words(raw_lines, keyword)
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
            rx: None,
            is_searching: false,
        }
    }
}

impl eframe::App for Gui {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        if let Some(Ok(results)) = self.rx.as_ref().map(|rx| rx.try_recv()) {
            self.search_result = results;
            self.is_searching = false;
            self.rx = None;
        }

        egui::CentralPanel::default().show_inside(ui, |ui| {
            ui.heading("Findit");
            ui.add_space(8.0);

            ui.horizontal(|ui| {
                ui.label("Start Directory:");
                ui.add(
                    egui::TextEdit::singleline(&mut self.search_start).desired_width(f32::INFINITY),
                );
            });

            ui.horizontal(|ui| {
                ui.label("Search Keyword:");
                ui.add(
                    egui::TextEdit::singleline(&mut self.search_keyword)
                        .desired_width(f32::INFINITY),
                );
            });

            ui.add_space(8.0);

            ui.horizontal(|ui| {
                ui.radio_value(&mut self.search_mode, SearchMode::Dir, "Directory");
                ui.radio_value(&mut self.search_mode, SearchMode::File, "File");
                ui.radio_value(&mut self.search_mode, SearchMode::Word, "Word");
            });

            ui.add_space(10.0);

            ui.horizontal(|ui| {
                ui.add_enabled_ui(!self.is_searching, |ui| {
                    if ui.button("Search").clicked() {
                        if !self.search_start.is_empty() {
                            let (tx, rx) = channel();
                            self.rx = Some(rx);
                            self.is_searching = true;

                            let mode = self.search_mode;
                            let keyword = self.search_keyword.clone();
                            let start = self.search_start.clone();

                            std::thread::spawn(move || {
                                let results = get_search_result(mode, &keyword, &start);
                                let _ = tx.send(results);
                            });
                        } else {
                            self.search_result =
                                vec!["Please specify a starting directory.".to_string()];
                        }
                    }
                });

                if self.is_searching {
                    ui.add(egui::Spinner::new());
                    ui.label("Searching...");
                }
            });

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
