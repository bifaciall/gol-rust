use eframe::egui;
use eframe::epi;

mod grid;
mod game;

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        Box::new(MyApp::default()),
        options,
    );
}

struct MyApp {
    game: game::Game,
    running: bool,
    cols: usize,
    rows: usize,
    generation_delay: u64,
    color_map: bool,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            game: game::Game::new(10, 10),
            running: false,
            cols: 10,
            rows: 10,
            generation_delay: 500,
            color_map: false,
        }
    }
}

impl MyApp {
    fn randomize_starting_conditions(&mut self) {
        self.game.randomize_grid();
    }
    fn clear_grid(&mut self) {
        self.game.clear_grid();
    }
}

impl epi::App for MyApp {
    fn update(&mut self, ctx: &egui::CtxRef, _frame: &mut epi::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Game of Life");

            let game = &mut self.game;
            let cell_size = 20.0;

            // for y in 0..grid.height {
            //     for x in 0..grid.width {
            //         let cell = grid.get(x, y);
            //         let rect = egui::Rect::from_min_size(
            //             egui::pos2(x as f32 * cell_size, y as f32 * cell_size),
            //             egui::vec2(cell_size, cell_size),
            //         );

            //         let color = if cell {
            //             egui::Color32::WHITE
            //         } else {
            //             egui::Color32::BLACK
            //         };

            //         let response = ui.allocate_rect(rect, egui::Sense::click_and_drag());
            //         if response.clicked() || response.dragged() {
            //             grid.set(x, y, !cell);
            //         }

            //         ui.painter().rect_stroke(rect, 0.0, egui::Stroke::new(1.0, egui::Color32::GRAY));
            //         ui.painter().rect_filled(rect, 0.0, color);
            //     }
            // }
            for y in 0..game.grid.height {
                for x in 0..game.grid.width {
                    let cell = game.grid.get(x, y);
                    let next_state = game.get_next_state(x, y);
                    let rect = egui::Rect::from_min_size(
                        egui::pos2(x as f32 * cell_size, y as f32 * cell_size),
                        egui::vec2(cell_size, cell_size),
                    );

                    let color = match (cell, next_state, self.color_map) {
                        (true, true, true) => egui::Color32::from_rgb(0, 255, 0), // Survives: green
                        (false, true, true) => egui::Color32::from_rgb(0, 128, 0), // Replicates: real green
                        (true, false, true) => egui::Color32::from_rgb(255, 0, 0), // Dies: red
                        (_, _, false) => if cell { egui::Color32::WHITE } else { egui::Color32::BLACK }, // Default colors
                        _ => egui::Color32::BLACK, // Fallback case
                    };

                    let response = ui.allocate_rect(rect, egui::Sense::click_and_drag());
                    if response.clicked() || response.dragged() {
                        game.grid.set(x, y, !cell);
                    }

                    ui.painter().rect_stroke(rect, 0.0, egui::Stroke::new(1.0, egui::Color32::GRAY));
                    ui.painter().rect_filled(rect, 0.0, color);
                }
            }
            

            if ui.button(if self.running { "Stop" } else { "Start" }).clicked() {
                self.running = !self.running;
            }
            if ui.button("Randomize Starting Conditions").clicked() {
                self.randomize_starting_conditions();
            }
            if ui.button("Clear Grid").clicked() {
                self.clear_grid();
            }
            ui.checkbox(&mut self.color_map, "Activate Color Map"); // Add checkbox for color map toggle


            if self.running {
                self.game.update();
                std::thread::sleep(std::time::Duration::from_millis(self.generation_delay));
            }

            ui.horizontal(|ui| {
                ui.label("Columns:");
                ui.add(egui::Slider::new(&mut self.cols, 5..=50));
            });
            ui.horizontal(|ui| {
                ui.label("Rows:");
                ui.add(egui::Slider::new(&mut self.rows, 5..=50));
            });
            ui.horizontal(|ui| {
                ui.label("Generation Delay (ms):");
                ui.add(egui::Slider::new(&mut self.generation_delay, 50..=1000)); // Add generation delay slider
            });

            if self.cols != self.game.grid.width || self.rows != self.game.grid.height {
                self.game = game::Game::new(self.cols, self.rows);
            }
        });

        if self.running {
            ctx.request_repaint();
        }

        if ctx.input().key_pressed(egui::Key::C) && ctx.input().modifiers.ctrl {
            std::process::exit(0);
        }
    }

    fn name(&self) -> &str {
        "Game of Life"
    }
}