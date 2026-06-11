import {computed, onMounted, ref} from "vue";
import {invoke} from "@tauri-apps/api/core";

import type {Channel, LogEntry, LogLevel} from "../types/relay";

export function useRelayDeck() {
    const port = ref("COM3");
    const baudRate = ref(9600);
    const slaveId = ref(1);
    const timeoutMs = ref(1000);
    const isConnected = ref(false);
    const isBusy = ref(false);

    const logs = ref<LogEntry[]>([
        {
            id: 1,
            time: getCurrentTime(),
            level: "info",
            message: "RelayDeck is ready. Connect to the Modbus relay to start the demo.",
        },
    ]);

    const channels = ref<Channel[]>([
        {
            id: 1,
            label: "Channel 1",
            description: "Main demo lamp",
            isOn: false,
        },
        {
            id: 2,
            label: "Channel 2",
            description: "Second output",
            isOn: false,
        },

    ]);

    const activeChannelsCount = computed(
        () => channels.value.filter((channel) => channel.isOn).length,
    );

    const connectionStatusLabel = computed(() =>
        isConnected.value ? "Connected" : "Disconnected",
    );

    onMounted(async () => {
        await loadConnectionStatus();
    });

    function addLog(message: string, level: LogLevel = "info") {
        logs.value = [
            {
                id: Date.now(),
                time: getCurrentTime(),
                level,
                message,
            },
            ...logs.value,
        ].slice(0, 8);
    }

    async function detectRelayPort() {
        if (isConnected.value || isBusy.value) return;

        isBusy.value = true;
        addLog("Scanning serial ports for a Modbus relay...", "info");

        try {
            const detectedPort = await invoke<string | null>("detect_relay_connected_at_port", {
                baud: baudRate.value,
                slaveId: slaveId.value,
            });

            if (!detectedPort) {
                addLog("No compatible relay was detected. Check USB/RS485 connection and slave ID.", "warning");
                return;
            }

            port.value = detectedPort;
            addLog(`Detected relay on ${detectedPort}.`, "success");
        } catch (error) {
            console.error(error);
            addLog(`Relay detection failed: ${String(error)}`, "error");
        } finally {
            isBusy.value = false;
        }
    }

    async function connectRelay() {
        if (isBusy.value) return;

        isBusy.value = true;

        try {
            await invoke("connect_relay", {
                portName: port.value,
                baud: baudRate.value,
                slaveId: slaveId.value,
            });

            isConnected.value = true;
            addLog(`Connected to relay on ${port.value} at ${baudRate.value} baud.`, "success");
            await refreshChannelStates();
        } catch (error) {
            console.error(error);
            addLog(`Failed to connect: ${String(error)}`, "error");
        } finally {
            isBusy.value = false;
        }
    }

    async function disconnectRelay() {
        if (isBusy.value) return;

        isBusy.value = true;

        try {
            await allChannelsOff();
            await invoke("disconnect_relay");

            isConnected.value = false;
            addLog("Disconnected from relay.", "warning");
        } catch (error) {
            addLog(`Failed to disconnect: ${String(error)}`, "error");
        } finally {
            isBusy.value = false;
        }
    }

    async function setChannel(channelId: number, isOn: boolean) {
        if (!isConnected.value || isBusy.value) return;

        const channel = channels.value.find((item) => item.id === channelId);
        if (!channel) return;

        isBusy.value = true;

        try {
            await invoke("relay_set", {
                channel: channelId,
                on: isOn,
            });

            channel.isOn = isOn;
            addLog(`${channel.label} turned ${isOn ? "ON" : "OFF"}.`, isOn ? "success" : "info");
            await refreshChannelStates();
        } catch (error) {
            addLog(`Failed to control ${channel.label}: ${String(error)}`, "error");
        } finally {
            isBusy.value = false;
        }
    }

    async function runDemoSequence() {
        if (!isConnected.value || isBusy.value) return;

        isBusy.value = true;
        addLog("Running demo sequence...", "info");

        try {
            for (const channel of channels.value) {
                await invoke("relay_set", {
                    channel: channel.id,
                    on: true,
                });

                channel.isOn = true;
                addLog(`${channel.label} ON`, "success");
                await fakeDelay(450);
            }

            await fakeDelay(700);

            for (const channel of [...channels.value].reverse()) {
                await invoke("relay_set", {
                    channel: channel.id,
                    on: false,
                });

                channel.isOn = false;
                addLog(`${channel.label} OFF`, "info");
                await fakeDelay(350);
            }

            await refreshChannelStates();
            addLog("Demo sequence completed.", "success");
        } catch (error) {
            addLog(`Demo sequence failed: ${String(error)}`, "error");
        } finally {
            isBusy.value = false;
        }
    }

    async function runBlinkAnimation() {
        if (!isConnected.value || isBusy.value) return;

        isBusy.value = true;
        addLog("Running blink animation...", "info");

        try {
            for (const channel of channels.value) {
                await invoke("relay_blink", {
                    channel: channel.id,
                    times: 2,
                    intervalMs: 300,
                });

                addLog(`${channel.label} blinked`, "success");
                await refreshChannelStates();
                await fakeDelay(200);
            }

            addLog("Blink animation completed.", "success");
        } catch (error) {
            addLog(`Blink animation failed: ${String(error)}`, "error");
        } finally {
            isBusy.value = false;
        }
    }

    async function allChannelsOff() {
        if (!isConnected.value) {
            for (const channel of channels.value) {
                channel.isOn = false;
            }
            return;
        }

        for (const channel of channels.value) {
            await invoke("relay_set", {
                channel: channel.id,
                on: false,
            });

            channel.isOn = false;
        }
    }

    async function refreshChannelStates() {
        if (!isConnected.value) return;

        const states = await invoke<boolean[]>("relay_get_all", {
            count: channels.value.length,
        });

        channels.value = channels.value.map((channel, index) => ({
            ...channel,
            isOn: states[index] ?? false,
        }));
    }

    async function loadConnectionStatus() {
        try {
            const connectedPort = await invoke<string | null>("relay_status");

            if (!connectedPort) {
                isConnected.value = false;
                return;
            }

            port.value = connectedPort;
            isConnected.value = true;
            addLog(`Existing relay connection detected on ${connectedPort}.`, "success");
            await refreshChannelStates();
        } catch (error) {
            addLog(`Failed to read relay status: ${String(error)}`, "error");
        }
    }

    return {
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
    };
}

function getCurrentTime() {
    return new Intl.DateTimeFormat("en", {
        hour: "2-digit",
        minute: "2-digit",
        second: "2-digit",
    }).format(new Date());
}

function fakeDelay(durationMs: number) {
    return new Promise((resolve) => window.setTimeout(resolve, durationMs));
}
