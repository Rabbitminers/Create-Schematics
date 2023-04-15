<template>
  <article class="star-rating">
    <section
      v-for="index in maxRating"
      :key="index"
      class="star"
      :class="starClasses(index)"
      @click="selectable && setCurrentRating(index)"
      @mouseover=" selectable && setHoveredRating(index)"
      @mouseleave="clearHoveredRating"
    >
      <i>*</i>
    </section>
  </article>
</template>

<script lang="ts">
import { ref } from 'vue';

export default {
    name: 'StarRating',
    props: {
        maxRating: {
            type: Number,
            default: 5,
            validator: (value: number) => value > 0,
        },
        rating: {
            type: Number,
            default: 5,
            validator: (value: number) => value > 0,
        },
        selectable: {
            type: Boolean,
            default: true,
        }
    },
    setup(props) {
        const currentRating = ref(props.rating);
        const hoveredRating = ref(0);

        const setCurrentRating = (rating: number) => { currentRating.value = rating; };
        const setHoveredRating = (rating: number) => { hoveredRating.value = rating; };
        const clearHoveredRating = () => { hoveredRating.value = 0;};

        const starClasses = (index: number) => {
            const classes = ['star'];
            if (index <= currentRating.value) {
                classes.push('selected');
            }
            if (index <= hoveredRating.value) {
                classes.push('hovered');
            }
            return classes.join(' ');
        };

        return {
            currentRating,
            hoveredRating,
            setCurrentRating,
            setHoveredRating,
            clearHoveredRating,
            starClasses,
        };
    },
};
</script>

<style lang="scss">
.star-rating {
    display: flex;

    --star-size: 1.2em;
    --selected-color: var(--colour-primary);
    --hover-color: var(--colour-primary-light);

    .star {
        cursor: pointer;
        margin-right: 5px;
        font-size: var(--star-size);

        i {
            font-size: var(--star-size);
        }

        &.selected i {
            color: var(--selected-color);
        }

        &.hovered i {
            color: var(--hover-color);
        }
    }
}
</style>