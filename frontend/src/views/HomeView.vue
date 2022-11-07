<script setup>
import axios from "axios";
import { ref, onMounted } from "vue";
import UserList from "../components/UserList.vue";

const url = "http://localhost:3000/users";
const users = ref([]);

function fetchUsers() {
  axios.get(url).then((response) => {
    users.value = response.data;
    console.log(users.value);
  });
}

onMounted(() => {
  fetchUsers();
});
</script>

<template>
  <main>
    <div>Home Page</div>
    <UserList :users="users" />
  </main>
</template>
