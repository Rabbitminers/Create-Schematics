<template>
  <article class="schematic-card" @click="viewSchematic">
    <section class="schematic-card-image">
      <img :src="image" alt="schematic-image"/>
    </section>
    <section class="schematic-card-body">
      <section class="schematic-card-body-title">
        <h2 class="schematic-card-title">{{title}}</h2>
        <a>{{game}} {{mod}}</a>
      </section>
      <section class="schematic-card-body-rating">
        <StarRating 
          :rating="rating" 
          :selectable='true'
        />
      </section>
      <section class="schematic-card-body-tags">
        <a class="schematic-card-body-tags-item" v-for="i in tags">Hello world</a>
      </section>
      <section class="schematic-card-body-description">
        <p>{{description}}</p>
      </section>
    </section>
    <section class="schematic-card-information">
      <section v-for="i in 3" class="schematic-card-information-downloads">
        Test
        <hr/>
      </section>
    </section>
  </article>
</template>

<script lang="ts">
import router from '@/router'
import StarRating from '../StarRating.vue';

export default {
  components: {
    StarRating
  },
  props: {
    image: {
      type: String,
      default: "missing.png",
      validator(value: string) {
        const validExtensions = /\.(png|jpe?g|webp)$/i;
        return validExtensions.test(value);
      },
    },
    title: {
      type: String,
      default: "Invalid schematic",
    },
    description: {
      type: String,
      default: "Description missing"
    },
    tags: {
      type: Array as () => String[],
      default: () => ["Test tag", "Tag2"],
    },
    rating: {
      type: Number,
      default: 5,
      validator(value: number) {
        return !isNaN(value) && value >= 0;
      }
    },
    game: {

      type: String,
      default: "1.1x.x"
    },
    mod: {
      type: String,
      default: "0.x.x.x"
    },
    id: {
      type: String,
      default: ""
    }
  },
  methods: {
    viewSchematic() {
      // router.push(`/schematic/${this.id}`)
    }
  }
}
</script>

<style lang="scss">
$border-radius: 2em;

.schematic-card {
  background-color: var(--colour-off-white);
  border-radius: $border-radius;
  padding: 1em;
  display: flex;
  cursor: pointer;
  gap: 1em;

  &-image {
    border-radius: $border-radius;
    background-color: var(--colour-off-white-dark);
    width: 10em;
    height: 10em;

    > img {
      border-radius: $border-radius;
      width: 10em;
      height: 10em;
    }

    > img:hover {
      border: solid;
    }
  }

  &-body {
    flex-grow: 1;

    &-description {
      border-radius: 1em; 
      margin-top: 0.75em;
      padding-left: 8px;
      height: 3.5em;
      background-color: var(--colour-off-white-dark);
    }

    &-title {
      display: flex;
      justify-content: space-between;

      > h2 {
        line-height: 1;
        font-weight: 800;
        font-size: 2em;
        text-shadow: 2px 2px var(--colour-off-white-dark);
      }
    }

    &-tags-item { 
      margin-right: 0.5em;
      padding: 0.3em;
      border-radius: 10px;
      color: var(--colour-off-white);
      background-color: var(--colour-primary);
    }
  }

  &-information {
    width: 7.5em;
    background-color: var(--colour-off-white-dark);
    border-radius: $border-radius;
    display: flex;
    flex-direction: column;

    > * {
      margin: 10px;
      flex-grow: 1;
      text-align: center;
    }
  }
}
</style>