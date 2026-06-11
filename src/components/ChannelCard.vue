<script lang="ts" setup>
import type {Channel} from "../types/relay";

defineProps<{
  channel: Channel;
  isConnected: boolean;
  isBusy: boolean;
}>();

const emit = defineEmits<{
  turnOn: [channelId: number];
  turnOff: [channelId: number];
}>();
</script>

<template>
  <article
      :class="channel.isOn ? 'border-rd-success/60 bg-[#14532d]/40 shadow-[0_0_24px_rgba(34,197,94,0.18)]' : ''"
      class="flex min-h-36 flex-col justify-between rounded-[22px] border border-rd-border bg-slate-950/50 p-5 transition-all duration-200"
  >
    <div>
      <div class="flex items-center gap-2.5">
        <span
            :class="channel.isOn ? 'bg-rd-success shadow-[0_0_20px_rgba(34,197,94,0.95)]' : 'bg-slate-500'"
            class="h-2.5 w-2.5 rounded-full"
        />
        <h3 class="mb-1 text-base font-bold">{{ channel.label }}</h3>
      </div>

      <p class="mb-5 text-rd-muted">{{ channel.description }}</p>

      <div
          :class="channel.isOn
          ? 'border-rd-success bg-[#14532d]/40 text-[#86efac]'
          : 'border-slate-700 bg-slate-900/60 text-slate-400'"
          class="mb-4 rounded-xl border px-3 py-2 text-center text-lg font-black tracking-wider"
      >
        {{ channel.isOn ? "ON" : "OFF" }}
      </div>
    </div>

    <div class="grid grid-cols-2 gap-2.5">
      <button
          :disabled="!isConnected || isBusy || channel.isOn"
          class="rounded-2xl bg-[#166534] px-4 py-3 font-extrabold text-green-50 transition hover:-translate-y-0.5 hover:bg-[#15803d] disabled:cursor-not-allowed disabled:opacity-45"
          @click="emit('turnOn', channel.id)"
      >
        ON
      </button>

      <button
          :disabled="!isConnected || isBusy || !channel.isOn"
          class="rounded-2xl bg-[#7f1d1d] px-4 py-3 font-extrabold text-red-50 transition hover:-translate-y-0.5 hover:bg-[#991b1b] disabled:cursor-not-allowed disabled:opacity-45"
          @click="emit('turnOff', channel.id)"
      >
        OFF
      </button>
    </div>
  </article>
</template>