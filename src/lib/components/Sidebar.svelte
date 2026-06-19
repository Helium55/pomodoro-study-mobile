<script lang="ts">
  import { page } from '$app/state'
  import { resolve } from '$app/paths'
  import { BarChart3, ListTodo, Palette, Settings, Timer } from '@lucide/svelte'
  import { getCopy } from '../i18n'
  import { settings } from '../stores/settings.svelte'

  const items = [
    { href: '/focus', key: 'focus', icon: Timer },
    { href: '/tasks', key: 'tasks', icon: ListTodo },
    { href: '/stats', key: 'stats', icon: BarChart3 },
    { href: '/theme', key: 'theme', icon: Palette },
    { href: '/settings', key: 'settings', icon: Settings },
  ] as const

  const copy = $derived(getCopy(settings.state.language))
</script>

<aside class="sidebar">
  <div class="brand">
    <span class="mark"></span>
    <strong>POMODORO<br />STUDY</strong>
  </div>

  <nav aria-label={copy.nav.aria}>
    {#each items as item (item.href)}
      {@const Icon = item.icon}
      <a
        href={resolve(item.href)}
        class="motion-press"
        class:active={page.url.pathname === item.href}
        aria-current={page.url.pathname === item.href ? 'page' : undefined}
      >
        <Icon size={17} strokeWidth={2.4} />
        <span>{copy.nav[item.key]}</span>
      </a>
    {/each}
  </nav>
</aside>

<style>
  .sidebar {
    width: 184px;
    min-width: 184px;
    height: 100vh;
    border-right: 1px solid var(--color-border);
    background: var(--color-bg);
    display: flex;
    flex-direction: column;
  }

  .brand {
    min-height: 92px;
    border-bottom: 1px solid var(--color-border);
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 18px;
    font-family: var(--font-display);
    font-size: 15px;
    line-height: 0.94;
  }

  .mark {
    width: 13px;
    height: 13px;
    background: var(--color-accent);
    box-shadow: var(--shadow-hard-sm);
  }

  nav {
    display: flex;
    flex-direction: column;
    padding-top: 10px;
  }

  a {
    min-height: 48px;
    display: flex;
    align-items: center;
    gap: 12px;
    border-bottom: 1px solid var(--color-border);
    color: var(--color-fg-muted);
    font-family: var(--font-mono);
    font-size: 12px;
    font-weight: 800;
    letter-spacing: 2px;
    padding: 0 16px;
    text-decoration: none;
    transition:
      background-color var(--duration-fast) var(--ease-precision),
      color var(--duration-fast) var(--ease-precision);
  }

  a:hover,
  a.active {
    background: var(--color-accent);
    color: var(--color-bg);
  }

  @media (width <= 760px), (orientation: landscape) and (width <= 900px) and (height <= 560px) {
    .sidebar {
      position: fixed;
      left: 0;
      right: 0;
      bottom: 0;
      z-index: 20;
      width: 100%;
      min-width: 0;
      height: auto;
      border-right: 0;
      border-top: 1px solid var(--color-border);
      border-bottom: 0;
      padding-bottom: env(safe-area-inset-bottom);
    }

    .brand {
      display: none;
    }

    nav {
      display: grid;
      grid-template-columns: repeat(4, 1fr);
      padding-top: 0;
    }

    a {
      justify-content: center;
      min-height: 64px;
      padding: 0;
    }

    a[href$='/theme'] {
      display: none;
    }

    a span {
      display: none;
    }
  }
</style>
