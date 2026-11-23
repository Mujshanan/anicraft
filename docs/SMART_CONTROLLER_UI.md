# Forma Deform — Native UI Foundation

**Status:** M1 foundation implemented on the AniCraft host branch. This is not a completed Smart Controller workflow.

## Product boundary

Forma Deform's native UI is part of AniCraft. It reuses AniCraft's Stage, Properties panel, timeline, document model, command system, and undo stack. It must not create a second editor, timeline, renderer, document, or undo system.

## M1 — Smart Controller tool foundation

The current slice provides:

- A native Smart Controller entry in AniCraft's existing tool strip.
- Stage-space creation and selection of temporary controller handles.
- A Stage overlay rendered above artwork and outside the document render pipeline.
- A Properties section showing the selected handle's stage coordinates.
- Explicit removal and clearing of temporary handles.
- Keyboard/screen-reader naming through the existing tool registry and accessible button system.
- Unit coverage for nearest-handle hit testing and invalid/non-finite inputs.

### State and safety

Handle positions and selection are transient application view state. They are not serialized, do not dirty the document, and do not participate in undo/redo. The Properties panel labels this state as a draft and explicitly states that binding, evaluation, target application, persistence, and live playback are not connected.

No Save, Apply, Bake, or Preview action is shown because there is no verified host-backed operation for those actions in this milestone.

## M1 limitations

- Handles can be created, selected, removed, and cleared; drag editing is not yet implemented.
- No source-bone picker or stable source binding exists.
- No target identity or target capability inspector exists.
- No mapping editor, validation graph, or evaluation breakdown exists.
- No document persistence or migration exists.
- No controller evaluation, target application, or live playback exists.
- Smart Controller remains independent of Form Deform pin evaluation.

## Next UI milestones

1. **M2 — Target-aware authoring:** verified target selection/identity, source selection, explicit neutral capture, and capability-aware inspector states.
2. **M3 — Mapping editor:** supported one-dimensional range/easing controls, validation, and a mapping graph backed by the canonical Forma Deform core.
3. **M4 — Deterministic preview:** on-demand non-mutating evaluation with raw input, normalized input, mapped output, diagnostics, and a read-only Stage overlay.
4. **M5 — Host persistence and commands:** versioned document-owned bindings, explicit commands, migration, and one-step AniCraft undo/redo.
5. **M6 — Playback integration:** only after AniCraft provides and verifies a supported evaluator/scheduling boundary.

## Acceptance criteria for M1

- [x] Tool is integrated into AniCraft's existing tool strip and context-help/keymap registries.
- [x] Handles render as editor overlays, not authored artwork.
- [x] Handle state is transient and is never represented as saved.
- [x] Properties exposes the selected handle and explicit draft cleanup.
- [x] Unsupported binding, preview, apply, persistence, and playback actions are not exposed.
- [x] Hit testing rejects invalid tolerance and non-finite input.
- [ ] AniCraft native Windows build and host CI pass on the exact PR head.
- [ ] Manual Windows smoke test confirms tool switching, stage hit testing, Properties behavior, and no document dirty-state changes.

M1 must not be described as a complete deformation/controller authoring UI until the later milestones and their host acceptance tests pass.
