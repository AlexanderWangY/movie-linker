<script lang='ts'>
    import SixDegreeScreen from '$lib/components/SixDegreeScreen.svelte'
</script>

<div class="m-0 w-full h-full flex-col bg-neutral-900">
  <div class="flex w-full h-10 pt-20 pb-10 font-bold text-4xl justify-center items-center text-center text-neutral-100">Six Degrees</div>
  <!-- <SixDegreeScreen /> -->
  <div class="flex-col w-full justify-center items-center p-12">
    <h1 class="flex w-full justify-center items-center text-neutral-100">searchbar goes here :3 + ADD AUTOCOMPLETE</h1>

    <div class="flex w-full min-h-screen justify-center items-center">
      <!-- min h screen makes margin clapped but idk how to avoid the whitespace  -->
      <div class="p-10">
        <div class="text-neutral-100 text-xl">Previous Guesses<br></div>
        <div class="text-neutral-100 text-center">placeholder</div>
      </div>
      <div class="p-10">
        <div class="text-neutral-100 text-xl">Guesses Left<br></div>
        <div class="text-neutral-100 text-center">placeholder</div>
      </div>
    </div>
  </div>
</div>
