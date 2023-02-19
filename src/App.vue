<script setup>
import { onMounted } from 'vue';
import MyPlayer from './components/MyPlayer.vue'
import { invoke } from '@tauri-apps/api/tauri'
import { exit } from '@tauri-apps/api/process';
import { register } from '@tauri-apps/api/globalShortcut'

onMounted(async () => {

  await register('Shift+Q', async () => {
    await exit()
  })

  await register('Shift+W', async () => {
    await invoke('switch_fullscreen')
  })

  await register('Esc', async () => {
    await invoke('exit_fullscreen')
  })

})


</script>

<template>
  <div>
    <MyPlayer />
  </div>
</template>

