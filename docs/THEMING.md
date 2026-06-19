# Theming

## Layers

1. `src/themes/_tokens.ts` lists the token contract.
2. `src/themes/<id>/theme.css` defines concrete values.
3. Components only use `var(--token-name)`.

## Acid Art Constraints

- One accent: `--color-accent` (`#c6ff00` in the Acid theme).
- No component hex colors; component styles use tokens only.
- No rounded component corners.
- Box shadows are hard offset shadows with no blur.
- Use character progress bars instead of smooth decorative fills.
- Keep micro-interactions on top of the existing UI: no new layout shell, no palette swap, no decorative gradients, no soft cards.
- Respect `prefers-reduced-motion`; theme animations must have a reduced-motion branch.

Stylelint enforces these rules for component CSS. Theme CSS is exempt because it defines token values.

## Motion System

The Acid motion system lives in `src/themes/acid/animations.css`.

- `motion-page` gives page and dialog surfaces a short entrance.
- `motion-edge` adds a thin acid scan line to existing bordered surfaces.
- `motion-press` gives buttons and nav items a 1px tactile press.
- `motion-selected` gives selected rows a one-shot pulse.
- `motion-progress` gives text progress bars a subtle tick.

The target feel is "Precision Pulse": short, restrained, and readable during study sessions. Prefer 120-360ms durations and tokenized easing from `src/themes/acid/theme.css`.

## Adding A Theme

1. Copy `src/themes/acid/` to `src/themes/<id>/`.
2. Update `[data-theme='<id>']` token values in `theme.css`.
3. Update `meta.ts` with `id`, `displayName`, `preview`, and `status`.
4. Register the meta in `src/themes/registry.ts`.
5. Keep business components unchanged.
