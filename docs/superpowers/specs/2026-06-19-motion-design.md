# Pomodoro Study Motion Design

**Status:** Implemented for v0.3.0

## Goal

Add refined micro-interactions to the desktop and Android mobile apps while preserving the original Pomodoro Study UI.

## Direction

The approved direction is option A, "Precision Pulse". Motion should feel precise, short, and useful. It must not turn the app into a new visual design.

## Boundaries

- Keep the existing Acid UI: black surfaces, acid-lime accent, square controls, hard borders, and current layout.
- Add motion to existing surfaces only.
- Avoid new decorative backgrounds, gradients, glow-heavy effects, rounded cards, or marketing-style animation.
- Use the same motion primitives on desktop, phone, and tablet.
- Respect `prefers-reduced-motion`.

## Motion Primitives

- `motion-page`: short entrance for route content and dialogs.
- `motion-edge`: acid scan line on existing bordered panels, taskbars, rows, metrics, and sections.
- `motion-press`: 1px press feedback on buttons and navigation.
- `motion-selected`: one-shot selected row pulse.
- `motion-progress`: subtle tick on character progress bars.

## Testing

`src/themes/acid/motion-contract.test.ts` verifies the theme primitives and component hooks exist. The focus page regression test verifies that a selected current task remains visible after tasks load asynchronously.
