<script lang="ts">
  import { onMount } from 'svelte'
  import { bootstrapTheme } from '../themes/bootstrap'
  import Sidebar from '../lib/components/Sidebar.svelte'
  import { readPlatformProfile, type PlatformProfile } from '../lib/platform'
  import { goals } from '../lib/stores/goals.svelte'
  import { settings } from '../lib/stores/settings.svelte'
  import { tasks } from '../lib/stores/tasks.svelte'

  let { children } = $props()
  let profile = $state<PlatformProfile | null>(null)

  onMount(() => {
    void bootstrapTheme()
    void settings.load()
    void goals.load()
    void tasks.load()
    profile = readPlatformProfile()

    const updateProfile = () => {
      profile = readPlatformProfile()
    }
    window.addEventListener('resize', updateProfile)

    return () => {
      window.removeEventListener('resize', updateProfile)
    }
  })
</script>

<div
  class="app-shell"
  class:android={profile?.isAndroid}
  class:phone={profile?.formFactor === 'phone'}
  class:tablet={profile?.formFactor === 'tablet'}
>
  <Sidebar />
  <main class="content">
    {@render children()}
  </main>
</div>

<style>
  .app-shell {
    display: flex;
    min-height: 100vh;
    min-height: 100dvh;
    background: var(--color-bg);
    color: var(--color-fg);
  }

  .content {
    min-width: 0;
    flex: 1;
    height: 100vh;
    height: 100dvh;
    overflow: auto;
  }

  @media (width <= 760px), (orientation: landscape) and (width <= 900px) and (height <= 560px) {
    .content {
      height: 100dvh;
      padding-bottom: calc(64px + env(safe-area-inset-bottom));
    }
  }

  @media (orientation: landscape) and (width <= 900px) and (height <= 560px) {
    .content {
      height: calc(100dvh - 64px);
      padding-bottom: env(safe-area-inset-bottom);
    }
  }
</style>
