<script lang="ts">
  import { progressBlocks } from '../time'

  let { value, total = 12 }: { value: number; total?: number } = $props()
</script>

<span class="progress motion-progress" aria-label={`${Math.round(value * 100)} percent`}>
  {progressBlocks(value, total)}
</span>

<style>
  .progress {
    color: var(--color-accent);
    font-family: var(--font-mono);
    font-size: 12px;
    font-weight: 900;
    letter-spacing: 2px;
    white-space: nowrap;
  }
</style>
