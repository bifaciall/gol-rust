#[derive(Clone)]
pub struct Preset {
    pub name: &'static str,
    pub pattern: Vec<(usize, usize)>,
}

pub fn get_presets() -> Vec<Preset> {
    vec![
        Preset {
            name: "Glider",
            pattern: vec![(1, 0), (2, 1), (0, 2), (1, 2), (2, 2)],
        },
        // Add more presets here
    ]
}