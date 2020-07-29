pub struct EnvSettings {
    pub crossover_prob: f32,
    pub swap_homologous_prob: f32,
    pub mutate_prob: f32,
    pub spawn_percentage: f32
}

impl Default for EnvSettings {
    fn default() -> Self {
        EnvSettings {
            crossover_prob: 0.5,
            swap_homologous_prob: 0.25,
            mutate_prob: 0.1,
            spawn_percentage: 0.25
        }
    }
}

pub struct EnvSettingsBuilder {
    settings: EnvSettings
}

impl EnvSettingsBuilder {
    pub fn new() -> Self {
        EnvSettingsBuilder {
            settings: EnvSettings::default()
        }
    }

    pub fn crossover_prob(mut self, probability: f32) -> Self {
        self.settings.crossover_prob = probability;
        self
    }

    pub fn mutate_prob(mut self, probability: f32) -> Self {
        self.settings.mutate_prob = probability;
        self
    }

    pub fn build(self) -> EnvSettings {
        self.settings
    }
}