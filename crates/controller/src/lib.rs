//! Native controller prototype for AniCraft's Smart Controller feature request.
//!
//! This crate owns deterministic controller mapping, independently of the UI and plug-in
//! runtime. The first prototype maps a rig bone's evaluated angle to a nested symbol instance's
//! requested display frame. The host is responsible for resolving the source angle and applying
//! the returned frame to player/evaluation state; this crate does not mutate authored timelines.
//!
//! The plug-ins in the separate `forma-deform` repository are proof-of-concept explorations,
//! not this native implementation.

#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable)]

use anicraft_doc::armature::BoneId;
use anicraft_doc::player::InstancePath;
use serde::{Deserialize, Serialize};

/// A finite scalar input interval, such as a bone's joint angle in degrees.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct InputRange {
    pub min: f32,
    pub max: f32,
}

/// A symbol timeline frame interval. Endpoints are inclusive and may be reversed.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct FrameRange {
    pub first: u32,
    pub last: u32,
}

/// A document-authored relationship between one rig bone and one nested symbol instance.
///
/// This is the initial feature-request prototype shape. Durable document storage, authoring UI,
/// source-angle resolution, and integration with the player/evaluator are host responsibilities
/// and are deliberately not implemented here yet.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ControllerBinding {
    pub id: String,
    pub source_bone: BoneId,
    /// The full instance path from the main timeline to the target symbol instance.
    pub target_instance: InstancePath,
    pub input: InputRange,
    pub output: FrameRange,
}

/// The frame requested for a target instance by one controller evaluation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ControllerOutput {
    pub binding_id: String,
    pub target_instance: InstancePath,
    pub frame: u32,
}

/// Invalid configuration or input supplied to the controller mapper.
#[derive(Clone, Copy, Debug, PartialEq, Eq, thiserror::Error)]
pub enum ControllerError {
    #[error("controller input range must be finite and min must be less than max")]
    InvalidInputRange,
    #[error("controller target must identify a nested symbol instance")]
    EmptyTarget,
    #[error("controller input must be finite")]
    NonFiniteInput,
}

impl ControllerBinding {
    /// Checks the fields required by the first scalar-to-frame mapping prototype.
    pub fn validate(&self) -> Result<(), ControllerError> {
        if !self.input.min.is_finite()
            || !self.input.max.is_finite()
            || self.input.min >= self.input.max
        {
            return Err(ControllerError::InvalidInputRange);
        }
        if self.target_instance.is_empty() {
            return Err(ControllerError::EmptyTarget);
        }
        Ok(())
    }

    /// Maps an evaluated source value to an inclusive target frame range.
    ///
    /// Values outside the input interval are clamped. The result is rounded to the nearest
    /// integer frame, making evaluation deterministic and independent of playback timing.
    pub fn evaluate(&self, input: f32) -> Result<ControllerOutput, ControllerError> {
        self.validate()?;
        if !input.is_finite() {
            return Err(ControllerError::NonFiniteInput);
        }

        let t = ((input - self.input.min) / (self.input.max - self.input.min)).clamp(0.0, 1.0);
        let span = self.output.last as f64 - self.output.first as f64;
        let frame = (self.output.first as f64 + span * t as f64).round();
        let frame = frame.clamp(0.0, u32::MAX as f64) as u32;

        Ok(ControllerOutput {
            binding_id: self.id.clone(),
            target_instance: self.target_instance.clone(),
            frame,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anicraft_doc::ElementId;

    fn binding() -> ControllerBinding {
        ControllerBinding {
            id: "hand-to-mouth".into(),
            source_bone: BoneId(7),
            target_instance: vec![ElementId(10), ElementId(22)],
            input: InputRange { min: -45.0, max: 45.0 },
            output: FrameRange { first: 0, last: 8 },
        }
    }

    #[test]
    fn maps_input_endpoints_to_frame_endpoints() {
        let binding = binding();
        assert_eq!(binding.evaluate(-45.0).map(|o| o.frame), Ok(0));
        assert_eq!(binding.evaluate(45.0).map(|o| o.frame), Ok(8));
    }

    #[test]
    fn clamps_out_of_range_input() {
        let binding = binding();
        assert_eq!(binding.evaluate(-90.0).map(|o| o.frame), Ok(0));
        assert_eq!(binding.evaluate(90.0).map(|o| o.frame), Ok(8));
    }

    #[test]
    fn supports_reversed_output_ranges() {
        let mut binding = binding();
        binding.output = FrameRange { first: 8, last: 0 };
        assert_eq!(binding.evaluate(-45.0).map(|o| o.frame), Ok(8));
        assert_eq!(binding.evaluate(45.0).map(|o| o.frame), Ok(0));
    }

    #[test]
    fn rejects_invalid_ranges_and_non_finite_inputs() {
        let mut binding = binding();
        binding.input.max = binding.input.min;
        assert_eq!(binding.validate(), Err(ControllerError::InvalidInputRange));

        let binding = binding();
        assert_eq!(binding.evaluate(f32::NAN), Err(ControllerError::NonFiniteInput));
    }

    #[test]
    fn rejects_a_missing_target_instance() {
        let mut binding = binding();
        binding.target_instance.clear();
        assert_eq!(binding.validate(), Err(ControllerError::EmptyTarget));
    }
}
