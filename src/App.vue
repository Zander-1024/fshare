<script setup>
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import SelectFile from "./components/SelectFile.vue";
import { useTheme } from 'vuetify'


const theme = useTheme()

function toggleTheme() {
  theme.global.name.value = theme.global.current.value.dark ? 'light' : 'dark'
}
</script>

<template>
  <v-app overflow-y-hidden>
    <v-app-bar title="File Share Tool" flat density="compact">
      <template v-slot:append>
        <v-switch @click="toggleTheme" :label="theme.global.name.value" hide-details inset>
        </v-switch>
      </template>
    </v-app-bar>
    <v-main class="d-flex align-center justify-center">
      <SelectFile />

    </v-main>
  </v-app>
</template>

<style scoped>
html { 
  overflow-y: hidden;
}
#app {
  overflow: hidden;
}


</style>
