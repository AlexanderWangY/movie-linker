<!-- Imports -->
<script lang='ts'>
  import SixDegreeScreen from '$lib/components/SixDegreeScreen.svelte'
  import { Autocomplete } from '@skeletonlabs/skeleton';
  import type { AutocompleteOption } from '@skeletonlabs/skeleton';

  
  let input = '';

  const test: AutocompleteOption<string>[] = [
    { label: 'Meow', value: 'meow' },
    { label: 'Miao', value: 'miao' },
    { label: 'Goon', value: 'goon' },
    { label: 'Hawk', value: 'Hawk' },
    { label: 'Tuah', value: 'Tuah' },
    { label: 'Hawk Tuah', value: 'Hawk Tuah' },
    { label: 'Tuah', value: 'Tuah' },
    { label: 'Tuah', value: 'Tuah' },
    { label: 'Bruzz', value: 'Bruzz' },
    { label: 'Tuah', value: 'Tuah' },
  
  ];

  function onSelect(event: CustomEvent<AutocompleteOption<string>>): void {
    input = event.detail.label;
  }
</script>


<div class="m-0 w-full min-h-screen flex-col justify-between bg-neutral-900">
  <div class="flex w-full h-10 pt-20 pb-10 font-bold text-5xl justify-center items-center text-center text-neutral-100">Six Degrees</div>
  
  <!-- Autocomplete Search from skeleton lib -->
  <div class="min-h-full max-w-full p-4 pt-16 flex flex-col items-center grow">
    <input class="input max-w-2xl p-2 bg-neutral-300 border-stone-300 rounded-md text-neutral-900" type="search" name="demo" bind:value={input} placeholder="Search..." />
    <!-- WHY IS IT NOT WIDE ENOUGH -->
    <div class="card max-h-48 min-w-96 p-2 overflow-y-auto bg-neutral-300 rounded-md mt-2" tabindex="-1">
      <Autocomplete bind:input={input} options={test} on:selection={onSelect} />
    </div>
  </div>

  
  <!-- <SixDegreeScreen /> -->


  <!-- NEED TO BE AT "FOOTER" OF SITE -->
  <div class="flex-col w-full justify-center items-center p-8 h-10">
    <div class="flex w-full justify-center items-center">
      <div class="px-20">
        <div class="text-neutral-100 text-xl">Previous Guesses<br></div>
        <div class="text-neutral-100 text-center">placeholder</div>
      </div>
      <div class="px-20">
        <div class="text-neutral-100 text-xl">Guesses Left<br></div>
        <div class="text-neutral-100 text-center">placeholder</div>
      </div>
    </div>
  </div>
</div>
