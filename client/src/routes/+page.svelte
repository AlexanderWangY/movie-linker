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
    { label: 'Astonishing Tales of Terror: Rocktapussy!', value: 'Astonishing Tales of Terror: Rocktapussy!'}
  
  ];

  function onSelect(event: CustomEvent<AutocompleteOption<string>>): void {
    input = event.detail.label;
  }
</script>


<div class="m-0 w-full min-h-screen flex flex-col justify-between bg-neutral-900">
  <div class="flex w-full h-10 pt-32 pb-10 font-bold text-5xl justify-center items-center text-center text-zinc-100">Six Degrees</div>
  
  <!-- Autocomplete Search from skeleton lib -->
  <div class="min-h-full max-w-full p-4 pt-16 flex flex-col items-center grow">
    <input class="input max-w-2xl p-2 bg-neutral-700 border-stone-700 rounded-md text-zinc-300" type="search" name="demo" bind:value={input} placeholder="Search..." />
    <!-- WHY IS IT NOT WIDE ENOUGH; ALSO SIZE CHANGES BASED ON RESULT; FIXES IF MIN W = FULL BUT THATS CLAPPED -->
    <div class="card max-h-48 min-w-full p-2 overflow-y-auto bg-neutral-700 rounded-md mt-2 text-zinc-100" tabindex="-1">
      <Autocomplete bind:input={input} options={test} on:selection={onSelect} />
    </div>
  </div>

  
  <!-- <SixDegreeScreen /> -->


  <div class="flex flex-col w-full justify-center items-center p-8 pb-32 mt-auto">
    <div class="flex w-full justify-center items-center">
      <div class="px-20">
        <div class="text-zinc-100 text-xl">Previous Guesses<br></div>
        <div class="text-zinc-100 text-center">placeholder</div>
      </div>
      <div class="px-20">
        <div class="text-zinc-100 text-xl">Guesses Left<br></div>
        <div class="text-zinc-100 text-center">placeholder</div>
      </div>
    </div>
  </div>
</div>
