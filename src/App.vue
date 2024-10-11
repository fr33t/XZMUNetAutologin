<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core';
import { onMounted, ref } from 'vue';
const res = ref(0);


onMounted(async () => {
  const data: number = await invoke('test_network')
  res.value = data;

})


</script>

<template>
  <div>
    {{ res }}
  </div>
</template>
