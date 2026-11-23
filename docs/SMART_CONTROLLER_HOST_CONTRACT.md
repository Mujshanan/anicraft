# Smart Controller host integration contract

**Status:** Read-only contract baseline for M2. No binding, neutral capture, persistence, or target mutation UI is authorized by this document.

This contract records the AniCraft APIs the Smart Controller adapter may rely on. It is intentionally narrower than the full AniCraft editor model.

## 1. Source input: bone pose snapshot

The supported read command is `bone.info`.

### Request

```json
{"layer": 4, "frame": 24}
```

- `layer` is the numeric armature `LayerId`.
- `frame` is the zero-based frame to inspect.
- The deterministic Smart Controller adapter must pass an explicit frame. Omitting it means "current playhead" and is not acceptable for a frame-pinned snapshot.

### Response fields

```json
{
  "frame": 24,
  "armatures": [
    {
      "layer": 4,
      "bones": [
        {"id": 42, "name": "Head", "angle": 30.0, "swing": -5.0}
      ]
    }
  ]
}
```

The relevant semantics are implemented in AniCraft's `bone.info` command:

- `frame` echoes the resolved frame.
- `bones[].id` is the numeric document-scoped `BoneId`.
- `bones[].angle` is the constrained keyed pose angle in degrees.
- `bones[].swing` is `sprung_pose.angle - constrained_keyed_pose.angle`, in degrees.
- `name` is display text, not identity.

The adapter must reject a response whose frame differs from the requested frame, verify that the returned armature layer equals the explicit layer mapping, match the bone by numeric ID in that layer response, and convert the selected angle to radians. Spring swing is included only under an explicit caller-selected policy. AniCraft does not choose wrapped/unwrapped semantics for Smart Controller; the caller supplies that metadata.

### Snapshot guarantees and limitations

The canonical snapshot provider passes an explicit frame and validates both the echoed frame and armature layer. The older V1 Slider command remains a separate current-playhead, on-demand preview path; it is not a frame-pinned canonical snapshot.

A successful read is a point-in-time value for one invocation, not a live subscription or playback callback. AniCraft does not promise that a response remains fresh after the command returns. The adapter owns the immutable snapshot and must not cache it across invocations. A failed or mismatched query is unavailable; missing/malformed bone data is invalid. No default angle is supplied.

## 2. Target capability: geometry pin discovery

The supported read command is `warp.info`.

### Request

```json
{"element": 17, "frame": 24}
```

### Live-warp response

```json
{
  "element": 17,
  "layer": 4,
  "warped": true,
  "bitmap": false,
  "pins": [
    {"pin": 12, "at": [120.0, 80.0], "rest": [100.0, 70.0]}
  ]
}
```

### No-live-warp response

```json
{"element": 17, "layer": 4, "warped": false, "bitmap": false, "stale": false}
```

- `element` is the numeric document-scoped `ElementId`.
- `layer` is the owning `LayerId`.
- `warped` must be `true` for a live geometry-pin target.
- `bitmap` must be `false`.
- Each pin identity is `pins[].pin`, the numeric document-scoped `PinId`. It is not named `id`.
- `at` and `rest` are descriptive coordinates only; they are never identifiers.

The adapter must classify an exact element/pin match on live non-bitmap artwork as a geometry target with a maximum capability of **preview/undo-only**. Bitmap artwork, unwarped artwork, or a missing pin is Unsupported. Query failure, malformed response, or element mismatch is Unavailable. A selection or matching display name alone is not capability evidence.

## 3. Mutation boundary

AniCraft documents `warp.pin_move` as a mutating command. It edits the pin at the requested frame and may key it within a tween. The command must not be called directly by the Smart Controller inspector or preview calculator.

Any future application must be routed through AniCraft's command-dialog preview/undo transaction and tested for one-gesture/one-undo behavior. Until that integration is verified, target assessment is descriptive only.

No verified generic property setter or symbol displayed-frame setter is established by this contract.

## 4. Identity scope

AniCraft `ElementId`, `LayerId`, `BoneId`, and `PinId` are document-scoped numeric identities. The inspected `Document` structure has a next-ID counter but no stable document UUID. Consequently, numeric IDs cannot safely identify targets across unrelated documents or duplicated files.

The current M2 selection inspector may display these IDs as context. Durable Smart Controller bindings must wait for a host-owned stable document identity or equivalent collision-safe scope, including defined behavior for Save As, duplication, import, and reopen.

## 5. Independence

The host contract exposes read-only source and target inspection only. It does not combine Smart Controller outputs with Form Deform pin evaluation. CCC-01.5 remains binding: the two evaluation systems stay independent until a separately approved FDM v2 composition contract defines otherwise.
