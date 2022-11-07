<script setup>
import { computed } from "vue";

const props = defineProps({
  users: Array,
  default() {
    return [];
  },
});
const items = computed(() => props.users);
</script>
<template>
  <li v-for="(item, index) in items" :key="index">
    {{ item.id }} - {{ item.username }}
  </li>
</template>
