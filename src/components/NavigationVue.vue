<template>
<header class="site-header" role="presentation">
    <section>
        <RouterLink to="/home">
            <v-icon name="px-download"/> 
        </RouterLink>
    </section>
    <section>
        <RouterLink to="/home"></RouterLink>
    </section>
</header>
</template>

<script lang="ts">
import { OhVueIcon } from "oh-vue-icons";

export default {
  components: {
    "v-icon": OhVueIcon
  }
};
</script>

<style lang="scss">
.site-header {
    position: absolute;
    background-color: var(--colour-background-soft);
    display: flex;
    width: 100vw;
    gap: 10px;
    height: 10vh;

    > section {
        text-align: center;
    }
}

@media only screen and (min-width: 720px) {
    .site-header {
        flex-direction: column;
        width: 10em;
        height: 100vh;
    }
}
</style>