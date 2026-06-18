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

Stylelint enforces these rules for component CSS. Theme CSS is exempt because it defines token values.

## Adding A Theme

1. Copy `src/themes/acid/` to `src/themes/<id>/`.
2. Update `[data-theme='<id>']` token values in `theme.css`.
3. Update `meta.ts` with `id`, `displayName`, `preview`, and `status`.
4. Register the meta in `src/themes/registry.ts`.
5. Keep business components unchanged.
