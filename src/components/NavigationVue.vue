<template>
<header class="site-header" role="presentation">
    <section>
        <RouterLink to="/home">
            <DownloadIcon/>
        </RouterLink>
    </section>
    <section>
        <RouterLink to="/home"></RouterLink>
    </section>
</header>
</template>

<script setup lang="ts">
import DownloadIcon from './icons/DownloadIcon.vue';
</script>

<style lang="scss">
.site-header {
    background-color: var(--colour-andesite);
    display: flex;
    width: 100%;
    gap: 10px;
    height: 10vh;

    > section {
        padding: 10px;
        margin-left: auto;
        margin-right: auto;
        width: 3em;
        height: 3em;
    }
}

@media only screen and (min-width: 720px) {
    .site-header {
        flex-direction: column;
        width: 10em;
        height: 100vw;
    }
}
</style>