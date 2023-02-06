<script setup>
  import { defineComponent,ref,shallowRef,onMounted } from 'vue'
  import { VideoPlayer } from '@videojs-player/vue'

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

  onMounted(() => {
      window.addEventListener('resize', updateSize)
  })

  const updateSize = () => {
    windowWidth.value = window.innerWidth
    windowHeight.value = window.innerHeight
  }

  const video_mounted = (payload) => {
        state.value = payload.state
        player.value = payload.player
  }


</script>

<template>
  <VideoPlayer
    ref="videoPlayer"
    src="test.mp4"
    @mounted="video_mounted"
    :width="windowWidth"
    :height="windowHeight"
    :loop="true"
    :controls="true"
    :options="{autoplay:true,preferFullWindow:true}"
  />
</template>
