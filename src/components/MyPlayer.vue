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
}

const changeVideo = () => {
  video_index.value = (video_index.value + 1) % video_path.value.length
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

