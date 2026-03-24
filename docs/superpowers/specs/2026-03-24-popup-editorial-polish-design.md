# Popup Editorial Polish Design

**Date:** 2026-03-24

**Goal:** Refine the split popup UI so it feels like a polished, professional writing tool rather than a generic AI assistant interface.

## Summary

The popup already has the correct structural layout in split mode:

- top full-width output area
- bottom-left options/tools area
- bottom-right input area
- fixed full-width action bar at the bottom

The remaining issue is visual language. The current UI feels too much like a default AI tool because it relies on dark surfaces, generic panel styling, and utilitarian spacing. The revised direction will move the split popup toward a restrained editorial desktop-tool aesthetic.

## Chosen Direction

The approved visual direction is:

- magazine-lite / editorial product feel
- restrained and professional rather than flashy
- warm white paper-toned surfaces
- closer to a premium writing or document review tool
- explicitly avoid glassmorphism, neon highlights, and “AI dashboard” styling

## Scope

This design applies only to the split popup mode.

In scope:

- visual styling of the split popup shell
- output panel visual hierarchy
- left tool panel styling
- right input panel styling
- fixed action bar styling
- spacing, borders, background tone, and typography emphasis

Out of scope:

- vertical popup layout behavior
- MCP request/response logic
- popup layout structure changes
- tool behavior, template behavior, or context append logic
- major animation changes

## Visual System

### Color Direction

The split popup should shift from dark UI framing toward a warm document-tool palette:

- app background: muted warm neutral instead of near-black
- primary surfaces: warm white / paper tones
- secondary surfaces: slightly darker cream or parchment neutrals
- borders: soft warm gray-brown separators
- emphasis: restrained ink-like dark text and modest accent usage

The palette should feel more like an editorial workspace than a chat console.

### Surface Treatment

The design should rely on:

- subtle elevation
- clear but soft borders
- strong surface separation
- low-gloss presentation

It should not rely on:

- frosted glass
- glow effects
- aggressive gradients
- highly saturated interactive states

### Typography Tone

Typography should communicate a document-product feel:

- output headings and section labels should feel more editorial
- body content should prioritize readability and calm rhythm
- tool labels should remain compact and functional
- avoid oversized “AI app” badges or decorative labels

## Component Design

### Top Output Panel

The output panel is the visual anchor of the popup.

Target treatment:

- present like a pinned manuscript or review sheet
- warm white reading surface
- more generous internal spacing
- clearer type hierarchy for long-form reading
- quote blocks and markdown areas should feel like document content, not chat bubbles

This panel should visually dominate the split layout without becoming flashy.

### Bottom Left Tool Panel

The left column should behave like a quiet tool rail.

Target treatment:

- lower visual priority than the output panel
- grouped controls on a unified side panel surface
- predefined content, quick templates, and context append should look like editing tools, not generic controls
- option items should feel like compact editorial actions rather than default checkbox rows

This area should feel dense but calm.

### Bottom Right Input Panel

The input panel should feel like a writing or annotation surface.

Target treatment:

- cleaner text area presentation
- more document-tool styling than chat-input styling
- attachments visually separated from the text area
- generous spacing around the input surface

The user should feel like they are drafting a structured response, not chatting into a bot box.

### Fixed Bottom Action Bar

The bottom action bar remains structurally unchanged:

- fixed full-width strip
- same actions and behaviors

Visual treatment should be refined so it feels like a desktop tool footer:

- quieter button hierarchy
- reduced “AI action bar” feel
- clearer primary/secondary distinction

## Interaction Style

Interaction states should remain understated:

- hover: light background or border shift
- selected: clear but restrained
- focus: professional editor-like focus ring or border emphasis
- transitions: short and subtle only

No animated glow, bouncing, or attention-seeking motion should be introduced.

## Implementation Notes

The likely implementation should concentrate on:

- `src/frontend/components/popup/McpPopup.vue`
- `src/frontend/components/popup/PopupInput.vue`
- any directly related popup child styling that affects output readability or action-bar tone

The work should preserve the existing split layout structure and only change the visual layer and small presentational wrappers as needed.

## Validation

Success criteria:

- split mode no longer reads as a generic AI tool
- output area feels like the primary reading surface
- left panel feels like a professional tool rail
- right panel feels like a writing surface
- fixed bottom action bar still reads clearly but more quietly
- existing popup behavior remains unchanged

Implementation validation should include:

- frontend build passes
- split popup remains usable in current dimensions
- no regression to vertical layout behavior
