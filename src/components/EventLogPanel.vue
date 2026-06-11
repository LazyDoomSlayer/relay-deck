<script lang="ts" setup>
import type {LogEntry} from "../types/relay";

defineProps<{
  logs: LogEntry[];
  isBusy: boolean;
}>();
</script>

<template>
  <section
      class="min-h-56 rounded-[28px] border border-rd-border bg-rd-panel p-6 shadow-[0_24px_70px_rgba(0,0,0,0.34)] backdrop-blur-[18px]"
  >
    <div class="mb-6 flex flex-col justify-between gap-4 sm:flex-row sm:items-center">
      <div>
        <p class="mb-2 text-xs font-extrabold uppercase tracking-[0.14em] text-rd-accent">Step 3</p>
        <h2 class="mb-0 text-xl font-bold">Live Event Log</h2>
      </div>

      <span
          v-if="isBusy"
          class="inline-flex items-center gap-2 self-start whitespace-nowrap rounded-full bg-slate-800/90 px-4 py-2.5 text-slate-300"
      >
        Running...
      </span>
    </div>

    <ul class="m-0 grid list-none gap-2.5 p-0">
      <li
          v-for="log in logs"
          :key="log.id"
          :class="{
          'border-rd-success': log.level === 'success',
          'border-rd-warning': log.level === 'warning',
          'border-red-500': log.level === 'error',
        }"
          class="grid grid-cols-[92px_1fr] gap-3 rounded-2xl border-l-4 border-slate-500 bg-slate-950/45 px-4 py-3"
      >
        <span class="text-rd-muted tabular-nums">{{ log.time }}</span>
        <p class="m-0 text-blue-100">{{ log.message }}</p>
      </li>
    </ul>
  </section>
</template>
