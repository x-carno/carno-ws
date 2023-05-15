<script setup>
import { ref, computed } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

const clients = ref("");
async function client_list() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    clients.value = await invoke("client_list");
}

// a computed ref
const hasCheckedClient = computed(() => {
    var cList = clients.value;
    for (let i = 0; i < cList.length; i++) {
        const element = cList[i];
        if (element.checked) {
            return true
        }
    }
    return false
})

client_list();
</script>

<template>
    <p>{{ clients }}</p>
    <p>{{ clients.length }}</p>
    <p>Has checked client:
        <span>{{ hasCheckedClient }}</span>
    </p>

    <li v-for="item in clients">
        {{ item.addr }}:{{ item.checked }}
    </li>

    <a-list size="small" bordered :data-source="clients">
        <template #renderItem="{ item }">
            <a-list-item>
                <a-checkbox v-model:checked="item.checked">{{ item.addr }}</a-checkbox>
            </a-list-item>
        </template>
    </a-list>
</template>