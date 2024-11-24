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
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            game: game::Game::new(10, 10),
        }
    }
}

impl epi::App for MyApp {
    fn update(&mut self, ctx: &egui::CtxRef, _frame: &epi::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Game of Life");
            let grid = &mut self.game.grid;
            for y in 0..grid.height {
                for x in 0..grid.width {
                    let cell = grid.get(x, y);
                    let button = ui.button(if cell { "■" } else { "□" });
                    if button.clicked() {
                        grid.set(x, y, !cell);
                    }
                }
                ui.end_row();
            }
            if ui.button("Next Generation").clicked() {
                self.game.update();
            }
        });
    }

    fn name(&self) -> &str {
        "Game of Life"
    }
}