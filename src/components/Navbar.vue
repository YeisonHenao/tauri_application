<script setup lang="ts">
  import { RouterLink } from 'vue-router';
</script>

<template>
  <nav>
    <ul>
      <li>
        <RouterLink to="/">Dashboard</RouterLink>
      </li>
      <li>
        <RouterLink to="/login">Iniciar sesión</RouterLink>
      </li>
    </ul>
  </nav>
</template>