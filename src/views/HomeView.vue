<template>
<main class="home">
    <section class="home-banner full-width">
        <section class="home-banner-title">
            <LogoIcon class="home-banner-title-icon" size="xs"/>
            <h class="home-banner-title-text"> 
                Create: Schematics
            </h>
            <b class="home-banner-title-message">
                Share, Discover, Download Schematics 
            </b>
            <a class="home-banner-title-description">
                Easily find and share schematics for Create through our schematics platform built for ease of use and speed. With full ingame integration with our companion modWIP for Forge & Fabric
            </a>
        </section>
    </section>
    <section class="home-information full-width">
        <article class="home-information-about home-card">
            <b>Companion Mod</b>
            <hr/>
            <a>With our companion mod, coming soon for version 1.18.2 and 1.19.2 and both forge and fabric, quickly share, search for, and download schematics without ever needing to leave the game for a seamless and connected experience.</a>
        </article>
        <article class="home-information-help home-card">
            <b class="home-information-heading">How To Use Schematics</b>
            <hr/>
            <details class="home-information-details">
                <summary>Creating Schematics</summary>
                <hr/>
                <p>
                    Using a schematic & quill select the bounding areas of your creation then right click to save it, make sure to give it a memorable and descriptive name so you can easily find it later
                </p>
                <a href='https://www.youtube.com/watch?v=vFEkajTVCg8&t=80s'>Video Tutorial</a>
            </details>
            <details>
                <summary>
                    Sharing Schematics
                </summary>
                <hr/>
            </details>
            <details>
                <summary>Using Schematics Ingame</summary>
                <hr/>
                <p>Once saved or downloaded, schematics can be loaded at a schematic table and then positioned as a hologram projection ingame. In creative mode this projection can be placed instantly or in survival it can be assembled similar to mods such as litematica using the projection as a guide or using the schemati-cannon fueled with gunpowder which will automatically pull the required resources from nearby inventories</p>
            </details>
        </article>
        <article class="home-information-social home-card">
            <section>
                <img alt='reddit' src='reddit.webp'/>
                <b>Reddit</b>
                <br/>
                - <a href='https://www.reddit.com/r/CreateSchematics/'>r/CreateSchematics</a> 
                <br/>
                - <a href='https://www.reddit.com/r/Createmod/'>r/Createmod</a>
            </section>
            <section>
                <img alt='discord' src='discord.webp'/>
                <b>Discord</b>
                <br/>
                - <a href='https://www.reddit.com/r/CreateSchematics/'>Create Discord</a> 
                <br/>
                - <a href='https://www.reddit.com/r/CreateSchematics/'>Create Addon Hub</a>
            </section>
        </article>
    </section>
</main>
</template>

<script lang="ts">
import LogoIcon from '@/components/icons/LogoIcon.vue';

export default {
    components: {
        LogoIcon
    }
}
</script>

<style lang="scss">
</style>