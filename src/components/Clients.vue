<script setup>
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

const greetMsg = ref("");
// const name = ref("");

async function client_list() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    greetMsg.value = await invoke("client_list");
}

client_list();
</script>

<template>
    <p>{{ greetMsg }}</p>
    <p>{{ greetMsg.length }}</p>

    <li v-for="item in greetMsg">
        {{ item }}
    </li>

    <a-list size="small" bordered :data-source="greetMsg">
        <template #renderItem="{ item }">
            <a-list-item>
                <a-checkbox>{{ item }}</a-checkbox>
            </a-list-item>
        </template>
    </a-list>
</template>