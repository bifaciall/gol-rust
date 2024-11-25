use eframe::egui;
use eframe::epi;

mod grid;
mod game;
mod presets;

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
    selected_preset: Option<presets::Preset>,
    dragging_preset: Option<(presets::Preset, (f32, f32))>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            game: game::Game::new(10, 10),
            running: false,
            cols: 10,
            rows: 10,
            generation_delay: 500,
            selected_preset: None,
            dragging_preset: None,
        }
    }
}

impl epi::App for MyApp {
    fn update(&mut self, ctx: &egui::CtxRef, _frame: &mut epi::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Game of Life");

            let grid = &mut self.game.grid;
            let cell_size = 20.0;

            for y in 0..grid.height {
                for x in 0..grid.width {
                    let cell = grid.get(x, y);
                    let rect = egui::Rect::from_min_size(
                        egui::pos2(x as f32 * cell_size, y as f32 * cell_size),
                        egui::vec2(cell_size, cell_size),
                    );

                    let color = if cell {
                        egui::Color32::WHITE
                    } else {
                        egui::Color32::BLACK
                    };

                    let response = ui.allocate_rect(rect, egui::Sense::click_and_drag());
                    if response.clicked() || response.dragged() {
                        grid.set(x, y, !cell);
                    }

                    ui.painter().rect_stroke(rect, 0.0, egui::Stroke::new(1.0, egui::Color32::GRAY));
                    ui.painter().rect_filled(rect, 0.0, color);
                }
            }

            if ui.button(if self.running { "Stop" } else { "Start" }).clicked() {
                self.running = !self.running;
            }

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
            egui::SidePanel::right("preset_panel").show(ctx, |ui| {
                ui.heading("Presets");
                for preset in presets::get_presets() {
                    let response = ui.group(|ui| {
                        ui.label(preset.name);
                        let preset_size = 10.0;
                        for &(dx, dy) in &preset.pattern {
                            let rect = egui::Rect::from_min_size(
                                egui::pos2(dx as f32 * preset_size, dy as f32 * preset_size),
                                egui::vec2(preset_size, preset_size),
                            );
                            ui.painter().rect_filled(rect, 0.0, egui::Color32::WHITE);
                        }
                    });
                    if response.response.dragged() {
                        self.dragging_preset = Some((preset, ctx.input().pointer.interact_pos().unwrap().to_vec2().into()));
                    }
                }
            });
        });
            
            if let Some((preset, pos)) = &self.dragging_preset {
                let painter = ctx.layer_painter(egui::LayerId::new(egui::Order::Foreground, egui::Id::new("dragging_preset")));
                for &(dx, dy) in &preset.pattern {
                    let rect = egui::Rect::from_min_size(
                        egui::pos2(pos.0 + dx as f32 * 20.0, pos.1 + dy as f32 * 20.0),
                        egui::vec2(20.0, 20.0),
                    );
                    painter.rect_filled(rect, 0.0, egui::Color32::WHITE);
                }
            }


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