<script lang="ts" setup>
import ChannelCard from "./ChannelCard.vue";
import type {Channel} from "../types/relay";

defineProps<{
  channels: Channel[];
  activeChannelsCount: number;
  isConnected: boolean;
  isBusy: boolean;
}>();

const emit = defineEmits<{
  setChannel: [channelId: number, isOn: boolean];
  runSequence: [];
  runBlinkAnimation: [];
}>();
</script>

<template>
  <section
      class="rounded-[28px] border border-rd-border bg-rd-panel p-6 shadow-[0_24px_70px_rgba(0,0,0,0.34)] backdrop-blur-[18px]"
  >
    <div class="mb-6 flex flex-col justify-between gap-4 sm:flex-row sm:items-center">
      <div>
        <p class="mb-2 text-xs font-extrabold uppercase tracking-[0.14em] text-rd-accent">
          Step 2
        </p>
        <h2 class="mb-0 text-xl font-bold">Relay Channels</h2>
      </div>

      <div
          class="inline-flex items-center gap-2 self-start whitespace-nowrap rounded-full bg-slate-800/90 px-4 py-2.5 text-slate-300"
      >
        {{ activeChannelsCount }}/{{ channels.length }} active
      </div>
    </div>

    <div class="grid gap-4 md:grid-cols-3">
      <ChannelCard
          v-for="channel in channels"
          :key="channel.id"
          :channel="channel"
          :is-busy="isBusy"
          :is-connected="isConnected"
          @turn-on="emit('setChannel', $event, true)"
          @turn-off="emit('setChannel', $event, false)"
      />
    </div>

    <button
        :disabled="!isConnected || isBusy"
        class="mt-5 min-h-12 w-full rounded-2xl bg-gradient-to-br from-yellow-400 to-[#38bdf8] px-4 py-3 font-extrabold text-blue-950 transition hover:-translate-y-0.5 disabled:cursor-not-allowed disabled:opacity-45"
        @click="emit('runSequence')"
    >
      Run Demo Sequence
    </button>

    <button
        :disabled="!isConnected || isBusy"
        class="mt-2.5 min-h-12 w-full rounded-2xl bg-gradient-to-br from-[#7c3aed] to-[#ec4899] px-4 py-3 font-extrabold text-white transition hover:-translate-y-0.5 disabled:cursor-not-allowed disabled:opacity-45"
        @click="emit('runBlinkAnimation')"
    >
      Blink Animation
    </button>
  </section>
</template>