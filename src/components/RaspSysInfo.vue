<template>
    <f7-page name="raspSysInfo">
        <f7-block-title>System Info</f7-block-title>
        <div class="card data-table data-table-collapsible data-table-init">
            <div class="card-header">
                <div class="data-table-title">Current Info Table</div>
                <div class="data-table-actions">
                    <f7-link><f7-icon f7="line_horizontal_3_decrease"></f7-icon></f7-link>
                    <f7-link><f7-icon f7="ellipsis_vertical_circle"></f7-icon></f7-link>
                </div>
            </div>
            <div class="card-content">
                <table>
                    <thead>
                        <tr>
                            <th class="numeric-cell">Cpu Usage (%)</th>
                            <th class="numeric-cell">Memory Total / MB</th>
                            <th class="numeric-cell">Memory Used / MB</th>
                            <th class="numeric-cell">Memory Free / MB</th>
                            <th class="numeric-cell">Disk Total / MB</th>
                            <th class="numeric-cell">Disk Used / MB</th>
                            <th class="numeric-cell">Disk Free / MB</th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr>
                            <td class="numeric-cell">{{ cpuUsage * 100 }} %</td>
                            <td class="numeric-cell">{{ (memoryTotal / (1024 * 1024)).toFixed(2) }}</td>
                            <td class="numeric-cell">{{ (memoryUsed / (1024 * 1024)).toFixed(2) }}</td>
                            <td class="numeric-cell">{{ (memoryFree / (1024 * 1024)).toFixed(2) }}</td>
                            <td class="numeric-cell">{{ (diskTotal / (1024 * 1024)).toFixed(2) }}</td>
                            <td class="numeric-cell">{{ (diskUsed / (1024 * 1024)).toFixed(2) }}</td>
                            <td class="numeric-cell">{{ (diskFree / (1024 * 1024)).toFixed(2) }}</td>
                        </tr>
                    </tbody>
                </table>
            </div>
        </div>
        <f7-block-title>CPU USAGE</f7-block-title>
        <f7-block strong-ios outline-ios class="text-align-center">
            <f7-gauge type="circle" :value="cpuUsage" :size="250" border-color="#2196f3" :border-width="10"
                :value-text="`${cpuUsage * 100}%`" :value-font-size="41" value-text-color="#2196f3"
                label-text="cpu usage" />
        </f7-block>
        <f7-block-title>MEMORY / DISK USAGE</f7-block-title>
        <f7-block strong-ios outline-ios>
            <div class="grid grid-cols-2 grid-gap">
                <div class="text-align-center">
                    <f7-gauge type="semicircle" :value="memoryUsed / memoryTotal"
                        :value-text="`${(memoryUsed / memoryTotal * 100).toFixed(2)}%`" value-text-color="#f44336"
                        label-text="memory usage" border-color="#f44336" />
                </div>
                <div class="text-align-center">
                    <f7-gauge type="semicircle" :value="diskUsed / diskTotal"
                        :value-text="`${(diskUsed / diskTotal * 100).toFixed(2)}%`" value-text-color="#e91e63"
                        border-color="#e91e63" label-text="disk usage" label-text-color="#333" />
                </div>
            </div>
        </f7-block>
    </f7-page>
</template>

<script setup>
import { onMounted, ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { f7Page, f7Link, f7Icon } from "framework7-vue";
import WebSocket from '@tauri-apps/plugin-websocket';

const cpuUsage = ref("");
const memoryUsed = ref("");
const memoryTotal = ref("");
const memoryFree = ref("");
const diskTotal = ref("");
const diskUsed = ref("");
const diskFree = ref("");

onMounted(async () => {
    const ws = await WebSocket.connect('ws://118.25.85.152:8081');

    ws.addListener((event) => {
        let data = JSON.parse(event.data);
        cpuUsage.value = data.cpu_usage;
        memoryUsed.value = data.memory_used;
        memoryTotal.value = data.memory_total;
        memoryFree.value = data.memory_free;
        diskTotal.value = data.disk_total;
        diskUsed.value = data.disk_used;
        diskFree.value = data.disk_free;
    });
})
</script>

<style scoped></style>