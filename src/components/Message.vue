<script setup>
import { ref } from "vue";
// import { clients } from "./Clients.vue"
// import { invoke } from "@tauri-apps/api/tauri";

const msgSend = ref("");
// const name = ref("");

function sendMsg() {
    console.log(msgSend.value);
}

</script>

<template>
    <a-textarea v-model:value="msgSend" placeholder="Message to Send" style="height: 90%;" allowClear="true"
        :bordered="false"></a-textarea>
    <a-row>
        <a-col :span="6" :offset="18">
            <a-button type="primary" @click="sendMsg">Send</a-button>
        </a-col>
    </a-row>
</template>