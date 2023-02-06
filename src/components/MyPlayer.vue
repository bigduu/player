<template>
  <button @click="play">Play</button>
  <button @click="pause">Pause</button>
  <button @click="changeVideo">ChangeVideo</button>
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
import {BaseDirectory, readDir} from '@tauri-apps/api/fs';
import { convertFileSrc } from '@tauri-apps/api/tauri';
import {}  from '@tauri-apps/api/app'

defineComponent({
  components: {
    VideoPlayer
  }
})

const windowWidth = ref(window.innerWidth)
const windowHeight = ref(window.innerHeight)
const player = shallowRef()
const state = shallowRef()
const videoPlayer = ref(null)
const video_path = ref([])
const video_index = ref(0)
const current_video = ref("http://commondatastorage.googleapis.com/gtv-videos-bucket/sample/ElephantsDream.mp4")

onMounted(async () => {
  window.addEventListener('resize', updateSize)
  const baseDirPath = BaseDirectory.Desktop;
  await readDir("", {dir: baseDirPath, recursive: true}).then((files) => {
    for (const file of files) {
      if (file.path.includes(".mp4")) {
        const videoPath = convertFileSrc(file.path)
        console.log(videoPath)
        video_path.value.push(videoPath)
      }
    }
  });
})

const changeVideo = () => {
  video_index.value = (video_index.value + 1) % video_path.value.length
  console.log(video_path.value[video_index.value])
  current_video.value = video_path.value[video_index.value]
}

const play = () => {
  player.value.play()
}

const pause = () => {
  player.value.pause()
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

