/* eslint-disable no-unused-vars */
<template>
  <!-- <button @click="play">Play</button> -->
  <!-- <button @click="pause">Pause</button> -->
  <!-- <button @click="changeVideo">ChangeVideo</button> -->
  <VideoPlayer
      ref="videoPlayer"
      :src="current_video"
      @mounted="video_mounted"
      @ended="changeVideo"
      :width="windowWidth"
      :height="windowHeight"
      :options="{autoplay:true,preferFullWindow:true}"
  />
</template>

<script setup>
import {defineComponent, onMounted, ref, shallowRef} from 'vue'
import {VideoPlayer} from '@videojs-player/vue'
import {getVideoList} from '../api/client'
import {listen} from '@tauri-apps/api/event'
import {register} from '@tauri-apps/api/globalShortcut'
import {invoke} from '@tauri-apps/api/tauri'
import { exit } from '@tauri-apps/api/process';

defineComponent({
  components: {
    VideoPlayer
  }
})

const windowWidth = ref(window.innerWidth)
const windowHeight = ref(window.innerHeight)
const player = shallowRef()
const state = shallowRef()
const video_path = ref([])
const video_index = ref(0)
const current_video = ref("")
const is_playing = ref(true)

onMounted(async () => {
  window.addEventListener('resize', updateSize)
  await fetchVideoList()
})

const fetchVideoList = async () => {
  const res = await getVideoList()
  res.forEach((item) => {
    video_path.value.push(item)
    current_video.value = video_path.value[video_index.value]
  })

  await listen('play', () => {
    play()
  })

  await listen('pause', () => {
    pause()
  })

  await listen('changeVideo', () => {
    changeVideo()
  })

  await register('Shift+S',async () => {
    changeVideo()
  })

  await register('Shift+W',async () => {
    await exit()
  })

  await register('Shift+C',async () => {
    await invoke('switch_fullscreen')
  })

  await register('Enter',async () => {
    await invoke('switch_fullscreen')
  })

  await register('Esc',async () => {
    await invoke('exit_fullscreen')
  })

  await register('Shift+P',async () => {
    if (is_playing.value) {
      pause()
    } else {
      play()
    }
  })
}

const changeVideo = () => {
  video_index.value = (video_index.value + 1) % video_path.value.length
  current_video.value = video_path.value[video_index.value]
}


const play = () => {
  player.value.play()
  is_playing.value = true
}

const pause = () => {
  player.value.pause()
  is_playing.value = false
}

const updateSize = () => {
  windowWidth.value = window.innerWidth
  windowHeight.value = window.innerHeight
}

const video_mounted = (payload) => {
  state.value = payload.state
  player.value = payload.player
}
</script>

