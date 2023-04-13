<template>
<div class="note">
    Hello world
    <hr/>
</div>
</template>

<script lang="ts">
export default {

}
</script>

<style lang="scss">

div .note {
  position:relative;
  padding:40px;
  background: var(--colour-primary);
  max-width:500px;
  margin:10px auto 40px;
  box-shadow: 0 1px 6px rgba(0,0,0,0.05);
  font-size: 1.2em;
  color: var(--colour-text);
  
  &::after {
    display: block;
    content:" ";
    position: absolute;
    left:0;
    bottom:-2px;
    width:100%;
    height:3px;
    background: var(--colour-primary);
    box-shadow: 0 3px 6px rgba(0,0,0,0.2);
    -webkit-clip-path: polygon(0% 0%, 5%  70%, 10% 0%, 15%  100%, 20% 0%, 25% 45%, 30% 0%, 35%  100%, 40% 0%, 45%  60%, 50% 0%, 55%  100%, 60% 0%, 65%  100%, 70% 0%, 75%  100%, 80% 0%, 85%  100%, 90% 0%, 95%  50%, 100% 0%);
    clip-path: polygon(0% 0%, 5%  70%, 10% 0%, 15%  100%, 20% 0%, 25% 45%, 30% 0%, 35%  100%, 40% 0%, 45%  60%, 50% 0%, 55%  100%, 60% 0%, 65%  100%, 70% 0%, 75%  100%, 80% 0%, 85%  100%, 90% 0%, 95%  50%, 100% 0%);
	}
  
  &.rough {
    &::after {
      bottom:-6px;
      height:6px;
    }
  }
}

</style>