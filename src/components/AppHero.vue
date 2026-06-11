<script lang="ts" setup>
defineProps<{
  isConnected: boolean;
  connectionStatusLabel: string;
}>();
</script>

<template>
  <section
      class="mb-6 flex flex-col justify-between gap-6 rounded-[28px] border border-rd-border bg-rd-panel p-6 shadow-[0_24px_70px_rgba(0,0,0,0.34)] backdrop-blur-[18px] md:flex-row md:items-start md:p-8"
  >
    <div>
      <p class="mb-2 text-xs font-extrabold uppercase tracking-[0.14em] text-rd-accent">
        Tauri v2 · Rust · Modbus RTU
      </p>

      <h1 class="mb-3 text-[clamp(2.8rem,7vw,5.6rem)] font-bold leading-[0.9] tracking-[-0.08em]">
        RelayDeck Demo
      </h1>

      <p class="mb-0 max-w-3xl text-lg text-rd-subtle">
        A small hardware control panel for showing how a Vue interface can call Rust commands and control a real relay
        device.
      </p>
    </div>

    <div
        :class="{ 'bg-green-800/30 text-green-200': isConnected }"
        class="inline-flex items-center gap-2 self-start whitespace-nowrap rounded-full bg-slate-800/90 px-4 py-2.5 text-slate-300"
    >
      <span
          :class="isConnected ? 'bg-[#22c55e] shadow-[0_0_20px_rgba(34,197,94,0.95)]' : 'bg-slate-500'"
          class="h-2.5 w-2.5 rounded-full"
      />
      {{ connectionStatusLabel }}
    </div>
  </section>
</template>