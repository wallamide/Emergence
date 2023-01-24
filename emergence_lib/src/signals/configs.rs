//! Utilities to manage configuration of signals (colour, decay rate, etc.).

use crate::signals::emitters::Emitter;
use bevy::ecs::system::Resource;
use indexmap::IndexMap;

/// A dictionary of available [`SignalConfig`]s.
///
/// Internally, this uses an [`IndexMap`], so that there is also a notion of order: the order
/// in which elements are inserted into the dictionary. Some notion of order is necessary in order
/// to color tiles consistently.
#[derive(Resource, Default, Clone, Debug)]
pub struct SignalConfigs {
    /// Stores the configuration associated with each emitter.
    configs: IndexMap<Emitter, SignalConfig>,
}

impl SignalConfigs {
    /// Get the signal configuration for the specified [`Emitter`], if present.
    pub fn get(&self, emitter: &Emitter) -> Option<&SignalConfig> {
        self.configs.get(emitter)
    }

    /// Insert a [`SignalConfig`] into the dictionary.
    ///
    /// If one is already associated with the specified [`Emitter`], this function follows
    /// [`HashMap`](std::collections::HashMap::insert) semantics by replacing the pre-existing configuration with the
    /// specified configuration, and then returning the pre-existing configuration.
    pub fn insert(&mut self, emitter: Emitter, config: SignalConfig) -> Option<SignalConfig> {
        self.configs.insert(emitter, config)
    }

    /// Iterate over the signals at this tile, in the order they were inserted.
    pub fn iter(&self) -> impl Iterator<Item = (&Emitter, &SignalConfig)> {
        self.configs.iter()
    }
}

/// Configuration settings for a particular [`Signal`](crate::signals::Signal).
#[derive(Clone, Copy, Debug)]
pub struct SignalConfig {
    /// The factor with which a unit of signal diffuses to a neighboring tile per tick.
    ///
    /// Note that this is not a probability, as it is un-normalized.
    pub diffusion_factor: f32,
    /// The probability with which a signal naturally decays per tick.
    pub decay_probability: f32,
}
