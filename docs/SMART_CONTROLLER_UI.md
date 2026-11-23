# Forma Deform — Native UI Foundation

**Status:** M1 foundation and the read-only M2 selection inspector are implemented on the AniCraft host branch. Binding and evaluation are not implemented.

## Product boundary

Forma Deform's native UI is part of AniCraft. It reuses AniCraft's Stage, Properties panel, timeline, document model, command system, and undo stack. It must not create a second editor, timeline, renderer, document, or undo system.

## M1 — Smart Controller tool foundation

The current slice provides:

- A native Smart Controller entry in AniCraft's existing tool strip.
- Stage-space creation and selection of temporary controller handles.
- Drag-to-move editing for an existing transient handle, with live coordinates in Properties.
- A Stage overlay rendered above artwork and outside the document render pipeline.
- A Properties section showing the selected handle's stage coordinates.
- Explicit removal and clearing of temporary handles.
- Keyboard/screen-reader naming through the existing tool registry and accessible button system.
- Unit coverage for nearest-handle hit testing, valid handle movement, and invalid/non-finite inputs.

### State and safety

Handle positions and selection are transient application view state. They are not serialized, do not dirty the document, and do not participate in undo/redo. The Properties panel labels this state as a draft and explicitly states that binding, evaluation, target application, persistence, and live playback are not connected.

No Save, Apply, Bake, or Preview action is shown because there is no verified host-backed operation for those actions in this milestone.

## M1 limitations

- Handles can be created, selected, dragged, removed, and cleared; all edits remain transient view state.
- M2 reads AniCraft's current selected bone and artwork identities in Properties; selection is made with the existing Bone and Select tools.
- The M2 inspector is read-only. It does not establish controller compatibility, create a source/target binding, or capture a neutral value.
- No target capability contract or controller-input snapshot API exists.
- No mapping editor, validation graph, or evaluation breakdown exists.
- No document persistence or migration exists.
- No controller evaluation, target application, or live playback exists.
- Smart Controller remains independent of Form Deform pin evaluation.

## Next UI milestones

1. **M2 — Target-aware authoring:** selection identity inspection is implemented; verified capability queries and explicit neutral capture remain blocked on a host/core contract.
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
- [x] Dragging updates only the selected transient handle and rejects invalid coordinates/indices.
- [x] M2 Properties inspector reports the selected bone ID/name/layer and selected artwork ID/name/type/layer using existing host selections.
- [x] M2 clearly labels the inspector as read-only and states that compatibility, neutral capture, binding, and saving are unavailable.
- [ ] AniCraft native Windows build and host CI pass on the exact PR head.
- [ ] Manual Windows smoke test confirms tool switching, click-to-create, drag-to-move, live Properties coordinates, and no document dirty-state changes.

M1 must not be described as a complete deformation/controller authoring UI until the later milestones and their host acceptance tests pass.


---

## M2 host/core contract baseline

The read-only host contract is recorded in [SMART_CONTROLLER_HOST_CONTRACT.md](SMART_CONTROLLER_HOST_CONTRACT.md), and the core/adapter contract is recorded in [Forma Deform's contract](https://github.com/Mujshanan/forma-deform/blob/main/docs/contracts/SMART_CONTROLLER_HOST_CONTRACTS.md).

- `bone.info` snapshots must specify an explicit zero-based frame; `angle` is constrained keyed pose in degrees and `swing` is the separately reported spring contribution.
- `warp.info` identifies pins using `pins[].pin`, not `pins[].id`; a supported geometry target requires an exact element/pin match on live non-bitmap artwork.
- `ElementId`, `BoneId`, and `PinId` are document-scoped. AniCraft currently exposes no stable document UUID, so they are not sufficient as globally unique persistent binding keys.
- `warp.pin_move` remains mutating and must not be called outside a verified host preview/undo transaction.
- No neutral capture, binding creation, or persistence controls are introduced by these contracts. CCC-01.5 remains unchanged.
