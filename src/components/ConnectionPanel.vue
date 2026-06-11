<script lang="ts" setup>
const port = defineModel<string>("port", {required: true});
const baudRate = defineModel<number>("baudRate", {required: true});
const slaveId = defineModel<number>("slaveId", {required: true});
const timeoutMs = defineModel<number>("timeoutMs", {required: true});

defineProps<{
  isConnected: boolean;
  isBusy: boolean;
}>();

const emit = defineEmits<{
  detect: [];
  connect: [];
  disconnect: [];
}>();
</script>

<template>
  <aside
      class="flex flex-col gap-4 rounded-[28px] border border-rd-border bg-rd-panel p-6 shadow-[0_24px_70px_rgba(0,0,0,0.34)] backdrop-blur-[18px]"
  >
    <div class="mb-6">
      <p class="mb-2 text-xs font-extrabold uppercase tracking-[0.14em] text-rd-accent">Step 1</p>
      <h2 class="mb-0 text-xl font-bold">Connection</h2>
    </div>

    <label class="grid gap-2 text-sm font-bold text-rd-muted">
      Serial port
      <input
          v-model="port"
          :disabled="isConnected || isBusy"
          class="w-full rounded-2xl border border-rd-border-strong bg-rd-panel-strong px-4 py-3 text-rd-text outline-none focus:border-rd-accent focus:shadow-[0_0_0_4px_rgba(56,189,248,0.12)] disabled:opacity-65"
          placeholder="COM3 or /dev/ttyUSB0"
      />
    </label>

    <label class="grid gap-2 text-sm font-bold text-rd-muted">
      Baud rate
      <input
          v-model.number="baudRate"
          :disabled="isConnected || isBusy"
          class="w-full rounded-2xl border border-rd-border-strong bg-rd-panel-strong px-4 py-3 text-rd-text outline-none focus:border-rd-accent focus:shadow-[0_0_0_4px_rgba(56,189,248,0.12)] disabled:opacity-65"
          type="number"
      />
    </label>

    <label class="grid gap-2 text-sm font-bold text-rd-muted">
      Slave ID
      <input
          v-model.number="slaveId"
          :disabled="isConnected || isBusy"
          class="w-full rounded-2xl border border-rd-border-strong bg-rd-panel-strong px-4 py-3 text-rd-text outline-none focus:border-rd-accent focus:shadow-[0_0_0_4px_rgba(56,189,248,0.12)] disabled:opacity-65"
          min="1"
          type="number"
      />
    </label>

    <label class="grid gap-2 text-sm font-bold text-rd-muted">
      Timeout, ms
      <input
          v-model.number="timeoutMs"
          :disabled="isConnected || isBusy"
          class="w-full rounded-2xl border border-rd-border-strong bg-rd-panel-strong px-4 py-3 text-rd-text outline-none focus:border-rd-accent focus:shadow-[0_0_0_4px_rgba(56,189,248,0.12)] disabled:opacity-65"
          min="100"
          type="number"
      />
    </label>

    <div class="mt-auto grid gap-2.5 pt-3">
      <button
          v-if="!isConnected"
          :disabled="isBusy"
          class="min-h-12 w-full rounded-2xl border border-rd-border-strong bg-slate-800/90 px-4 py-3 font-extrabold text-blue-100 transition hover:-translate-y-0.5 hover:bg-slate-700 disabled:cursor-not-allowed disabled:opacity-45"
          @click="emit('detect')"
      >
        Auto-detect Relay
      </button>

      <button
          v-if="!isConnected"
          :disabled="isBusy || !port"
          class="min-h-12 w-full rounded-2xl bg-gradient-to-br from-cyan-300 to-[#38bdf8] px-4 py-3 font-extrabold text-sky-950 transition hover:-translate-y-0.5 disabled:cursor-not-allowed disabled:opacity-45"
          @click="emit('connect')"
      >
        Connect Relay
      </button>

      <button
          v-else
          :disabled="isBusy"
          class="min-h-12 w-full rounded-2xl bg-gradient-to-br from-rose-400 to-[#e11d48] px-4 py-3 font-extrabold text-rose-50 transition hover:-translate-y-0.5 disabled:cursor-not-allowed disabled:opacity-45"
          @click="emit('disconnect')"
      >
        Disconnect
      </button>
    </div>
  </aside>
</template>