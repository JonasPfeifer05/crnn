use core_crnn::genome::Genome;
use core_crnn::thinking_layer::ThinkingLayer;
use game_lib::{Game, GameMetaData, GameSettings};
use rand::distr::weighted::WeightedIndex;
use rand::distr::Distribution;
use rand::{rng};
use rayon::iter::ParallelIterator;
use rayon::iter::{IntoParallelIterator, ParallelDrainRange};
use std::time::Duration;

pub struct ModelTrainer {
    generation: Vec<ThinkingLayer>,
    config: TrainConfig,
    overall_best: Option<TrainResult>,
    last_generation_best: Option<TrainResult>,
}

pub struct TrainResult {
    pub score: f32,
    pub model: ThinkingLayer,
}

pub struct TrainConfig {
    pub epoch_size: usize,
    pub sample_size: usize,
    pub survival_rate: f32,
    pub mutation_probability: f64,
    pub mutation_strength: f64,
}

impl ModelTrainer {
    pub fn new(base_model: ThinkingLayer, config: TrainConfig) -> Self {
        Self {
            generation: (0..config.epoch_size)
                .map(|_| {
                    let mut relative = base_model.clone();
                    relative.mutate(config.mutation_probability, config.mutation_strength);
                    relative
                })
                .collect(),
            config,
            overall_best: None,
            last_generation_best: None,
        }
    }

    pub fn train_next_gen<TrainGame: GameMetaData + Game>(&mut self) {
        let mut model_scores: Vec<_> = self
            .generation
            .par_drain(..)
            .map(|model| {
                let avg_score = (0..self.config.sample_size)
                    .into_par_iter()
                    .map(|_| {
                        let mut game = TrainGame::from_model(model.clone());
                        game.run(GameSettings::default().duration(Duration::from_secs(30)));
                        game.score() / self.config.sample_size as f32
                    })
                    .sum::<f32>();
                (avg_score, model)
            })
            .collect();

        model_scores.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
        let best_model = model_scores.remove(0);
        let min_score = model_scores.last().unwrap().0;

        let mut weights: Vec<_> = model_scores
            .iter()
            .map(|(weight, _)| (*weight - min_score + 1.0))
            .collect();

        match &self.overall_best {
            None => {
                self.overall_best = Some(TrainResult {
                    score: best_model.0.clone(),
                    model: best_model.1.clone(),
                });
            }
            Some(old) => {
                if old.score < best_model.0 {
                    self.overall_best = Some(TrainResult {
                        score: best_model.0.clone(),
                        model: best_model.1.clone(),
                    });
                }
            }
        }
        self.last_generation_best = Some(TrainResult {
            score: best_model.0.clone(),
            model: best_model.1.clone(),
        });

        let rng = &mut rng();
        let mut survivors: Vec<_> = (0
            ..(self.config.epoch_size as f32 * self.config.survival_rate) as usize - 1)
            .map(|_| {
                let random_index = WeightedIndex::new(&weights).unwrap().sample(rng);
                weights.remove(random_index);
                model_scores.remove(random_index)
            })
            .collect();
        survivors.push(best_model);
        survivors.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());

        let min_score = survivors.last().unwrap().0;
        let weights: Vec<_> = survivors
            .iter()
            .map(|(weight, _)| (*weight - min_score))
            .collect();

        let random_survivor_index = WeightedIndex::new(&weights).unwrap();
        let mut new_generation: Vec<_> = (0..(self.config.epoch_size
            - (self.config.epoch_size as f32 * self.config.survival_rate) as usize))
            .flat_map(|_| {
                let parent_a = &survivors[random_survivor_index.sample(rng)].1;
                let parent_b = &survivors[random_survivor_index.sample(rng)].1;

                ThinkingLayer::crossover(parent_a, parent_b, 1)
            })
            .collect();
        new_generation.extend(survivors.into_iter().map(|(_, survivor)| survivor));

        new_generation.iter_mut().for_each(|genome| {
            genome.mutate(
                self.config.mutation_probability,
                self.config.mutation_strength,
            );
        });

        self.generation = new_generation;
    }

    pub fn overall_best(&self) -> &Option<TrainResult> {
        &self.overall_best
    }

    pub fn last_generation_best(&self) -> &Option<TrainResult> {
        &self.last_generation_best
    }
}
