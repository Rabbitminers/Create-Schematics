<template>
<header class="site-header" role="presentation">
    <div class="site-header-banner">
        <section class="site-header-logo">
            <LogoIcon :size="top ? 'xl' : 's'" class="site-header-logo-icon"/>
            <div class="top-only">
                <h1 class="site-header-logo-text">Create Schematics</h1>
                <hr class="site-header-logo-accent"/>
            </div>
        </section>

        <section class="site-header-toggle bottom-only">
            <button 
                class="site-header-toggle-expandinator"
                @click="toggleDrawer"
            >
                <p>^</p>
            </button>
        </section>

        <section class="site-header-account">
            <AccountIcon class="site-header-account-icon"/>
        </section>
    </div>
    <div class="site-header-navigation" :class="isExpanded ? 'expanded' : 'collapsed'">
        <section class="site-header-search">
            <input class="site-header-search-text"
                type="text"
            >
            <button class="site-header-search-button">
                Search!
            </button>
        </section>
        <section class="site-header-navigation-route">
            <RouterLink to="/home">Home</RouterLink>
        </section>
        <section class="site-header-navigation-route">
            <RouterLink to="/about">About</RouterLink>
        </section>
    </div>
</header>
</template>

<script lang="ts">
import LogoIcon from './icons/LogoIcon.vue';
import AccountIcon from './icons/AccountIcon.vue'

export default {
    components: {
        LogoIcon,
        AccountIcon
    },
    computed: {
        top() {
            return window.innerWidth > 12 * 96;
        }
    },
    data() {
        return {
            isExpanded: false
        }
    },
    methods: {
        toggleDrawer() {
            this.isExpanded = !this.isExpanded;
        }
    }
}
</script>

<style lang="scss">
.site-header {
    width: 100vw;

    > * {
        box-shadow: 0px 7px 10px #121212dd;
        transition: all;
    }

    &-banner {
        background-color: var(--colour-primary-dark);
        height: 8em;
        display: flex;
        justify-content: space-between;
    }

    &-logo {
        display: flex;

        > &-icon {
            padding: 10px;
            grid-area: 'icon';
            height: 8em;
        }

        > &-text {
            grid-area: 'title';
            font-size: 3em;
            color: var(--colour-off-white);
        }
    }    

    &-navigation {
        z-index: -1;
        display: flex;
        background-color: var(--colour-primary);
        width: 100vw;
        height: 3em;

        &-route {
            background-color: var(--colour-primary);
        }
    }

    &-account {
        display: flex;
        color: var(--colour-off-white);

        &-icon {
            align-self: center;
            margin: 3px;
            height: 3em;
        }
    }

    &-search {
        display: flex;
        align-items: center;

        > * {
            margin-left: 10px;
            border-radius: 1em;
            height: 2em;
            border-style: none;
        }

        &-text {
            background-color: var(--colour-off-white);
        }

        &-button {
            font-family: Minecraftia;
            color: var(--colour-off-white);
            background-color: var(--colour-primary-dark);
        }
    }

    &-toggle-expandinator {
        font-size: 3em;
        background: none;
        border-style: none;
        color: var(--colour-off-white);
        font-family: Minecraftia;
    }
}

@media only screen and (min-width: 12in) {
    .bottom-only {
        display: none;
    }
}

@media only screen and (max-width: 12in) {
    .site-header {
        position: fixed;
        bottom: 0;
        z-index: 50;

        &-navigation {
            transition: height cubic-bezier(0.77, 0, 0.175, 1) 1.2s;
        }

        &-logo-icon {
            height:10vh;
        }

        &-banner {
            height: 10vh;
            display: flex;
            flex-wrap: wrap;
            
            > * {
                flex: 0 1 0;
            }
        }
    }

    .top-only {
        display: none;
    }

    .expanded {
        height: 10em;
    }

    .collapsed {
        height: 0vh;
    }
}
</style>