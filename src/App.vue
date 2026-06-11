<script lang="ts" setup>
import AppHero from "./components/AppHero.vue";
import ConnectionPanel from "./components/ConnectionPanel.vue";
import ChannelsPanel from "./components/ChannelsPanel.vue";
import EventLogPanel from "./components/EventLogPanel.vue";
import {useRelayDeck} from "./composables/useRelayDeck";

const {
  port,
  baudRate,
  slaveId,
  timeoutMs,
  isConnected,
  isBusy,
  logs,
  channels,
  activeChannelsCount,
  connectionStatusLabel,
  detectRelayPort,
  connectRelay,
  disconnectRelay,
  setChannel,
  runDemoSequence,
  runBlinkAnimation,
} = useRelayDeck();
</script>

<template>
  <main
      class="min-h-screen bg-[radial-gradient(circle_at_top_left,rgba(0,132,255,0.28),transparent_34%),linear-gradient(135deg,var(--color-rd-bg)_0%,var(--color-rd-bg-soft)_48%,#111827_100%)] p-4 text-rd-text md:p-8"
  >
    <AppHero
        :connection-status-label="connectionStatusLabel"
        :is-connected="isConnected"
    />

    <section class="mb-6 grid gap-6 lg:grid-cols-[minmax(280px,360px)_1fr]">
      <ConnectionPanel
          v-model:baud-rate="baudRate"
          v-model:port="port"
          v-model:slave-id="slaveId"
          v-model:timeout-ms="timeoutMs"
          :is-busy="isBusy"
          :is-connected="isConnected"
          @connect="connectRelay"
          @detect="detectRelayPort"
          @disconnect="disconnectRelay"
      />

      <ChannelsPanel
          :active-channels-count="activeChannelsCount"
          :channels="channels"
          :is-busy="isBusy"
          :is-connected="isConnected"
          @set-channel="setChannel"
          @run-sequence="runDemoSequence"
          @run-blink-animation="runBlinkAnimation"
      />
    </section>

    <EventLogPanel
        :is-busy="isBusy"
        :logs="logs"
    />
  </main>
</template>
