# AniCraft Controller

Native controller prototype for the Smart Controller feature request.

## Purpose

The first vertical slice is intentionally narrow:

**evaluated rig-bone angle → controller mapping → nested symbol instance frame**

The three plug-ins in [Mujshanan/forma-deform](https://github.com/Mujshanan/forma-deform) are proof-of-concept explorations for the feature. They are not the controller implementation in AniCraft.

## Current scope

- A binding identifies a source `BoneId` and a nested target `InstancePath`.
- A finite input interval maps linearly to an inclusive output frame interval.
- Input values are clamped to the configured interval.
- Reversed output intervals are supported.
- Mapping is deterministic and has no dependency on playback time.
- Unit tests cover endpoints, clamping, reversed ranges, and invalid inputs.

## Not implemented yet

- Persisting bindings in the AniCraft document.
- Resolving a bone's effective angle from the current evaluated armature pose.
- Applying controller output to nested symbol playback/evaluation.
- Authoring and inspection UI.
- Multiple input channels, easing curves, conditions, and continuous live playback.
- SQLite. The separately discussed SQLite history belongs to Bone Query only.

## Host integration boundary

This crate computes a requested frame without mutating the document or authored timeline. The AniCraft host will own binding persistence, source resolution, scheduling, and application of target overrides to player/evaluation state. The integration must preserve ordinary timeline playback when no controller is active and must not rewrite authored keyframes.

The first feature-request prototype should demonstrate one bone driving one nested symbol instance's displayed frame. Geometry deformation and warp-pin control are separate concerns.
