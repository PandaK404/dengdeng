# Popup Editorial Polish Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Refine split popup mode so it feels like a restrained, premium document-review tool with warm paper tones and calmer hierarchy.

**Architecture:** Preserve the existing split structure and all interaction logic. Apply the new visual direction by restyling the split popup shell, output panel, left tool rail, right input panel, and fixed action bar, while leaving vertical mode behavior intact.

**Tech Stack:** Vue 3 SFCs, TypeScript, UnoCSS, Naive UI, Vite

---

### Task 1: Polish the Split Popup Shell

**Files:**
- Modify: `src/frontend/components/popup/McpPopup.vue`

- [ ] Update split-mode outer wrappers to use warm paper-toned backgrounds and calmer panel separation.
- [ ] Make the top output area feel like the primary reading surface rather than a generic panel.
- [ ] Restyle the fixed bottom action bar container so it reads like a desktop tool footer.
- [ ] Leave vertical mode markup and behavior unchanged.

### Task 2: Restyle the Left Tool Rail and Right Input Surface

**Files:**
- Modify: `src/frontend/components/popup/PopupInput.vue`

- [ ] Apply the editorial styling only in split mode.
- [ ] Restyle the left pane so predefined content, quick templates, and context append controls feel like compact editing tools.
- [ ] Restyle the right pane so the text area reads like a writing surface rather than a chat input.
- [ ] Adjust labels, spacing, borders, and section hierarchy to match the spec.
- [ ] Preserve all emit/update/image-paste/template/context behaviors.

### Task 3: Improve Output Readability and Footer Tone

**Files:**
- Modify: `src/frontend/components/popup/PopupContent.vue`
- Modify: `src/frontend/components/popup/PopupActions.vue`

- [ ] Restyle output content container typography and quote action so it feels like document content, not a chat bubble.
- [ ] Soften footer status line and action buttons to reduce the “AI tool” feel while keeping action clarity.
- [ ] Keep button semantics and shortcuts unchanged.

### Task 4: Verify Build Safety

**Files:**
- Verify: `src/frontend/components/popup/McpPopup.vue`
- Verify: `src/frontend/components/popup/PopupInput.vue`
- Verify: `src/frontend/components/popup/PopupContent.vue`
- Verify: `src/frontend/components/popup/PopupActions.vue`

- [ ] Run `export PATH=/home/panjk/.nvm/versions/node/v24.11.1/bin:$PATH && pnpm build`.
- [ ] Run `git diff --check`.
- [ ] Review the diff to ensure only popup visual presentation changed.
- [ ] Commit the work once verification passes.
