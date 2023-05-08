<script setup>
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import Greet from "./components/Greet.vue";
import Clients from "./components/Clients.vue"
import Message from "./components/Message.vue"


import { Splitpanes, Pane } from 'splitpanes'
import 'splitpanes/dist/splitpanes.css'

</script>



<template>
  <a-layout style="height: 100%;">
    <a-layout-header>
      header
    </a-layout-header>
    <a-layout-content style="height: 100%;">
      <splitpanes class="default-theme" style="margin-top: 4px;">
        <pane>
          <a-button type="primary">Primary Button</a-button>
          <Clients></Clients>
        </pane>
        <pane>
          <Message></Message>
        </pane>
        <pane>
          <div>
            3
          </div>
        </pane>
      </splitpanes>
    </a-layout-content>
  </a-layout>
</template>