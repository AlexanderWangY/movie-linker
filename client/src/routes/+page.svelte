<!-- Imports -->
<script lang="ts">
	import SixDegreeScreen from '$lib/components/SixDegreeScreen.svelte';

	// Supports weights 100-900
	import '@fontsource-variable/inter';

	import { Autocomplete } from '@skeletonlabs/skeleton';
	import type { AutocompleteOption } from '@skeletonlabs/skeleton';

	let input = '';
  let selected = false;

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
		{
			label: 'Astonishing Tales of Terror: Rocktapussy!',
			value: 'Astonishing Tales of Terror: Rocktapussy!'
		}
	];

	function onSelect(event: CustomEvent<AutocompleteOption<string>>): void {
		input = event.detail.label;
    selected = true;
		// TODO: SEND REQUEST TO SERVER THEN DISPLAY RESULT
	}
</script>

<div class="m-0 flex min-h-screen w-full flex-col justify-between bg-neutral-900">
	<div
		class="flex h-10 w-full items-center justify-center pb-10 pt-32 text-center text-5xl font-bold text-zinc-100"
	>
		Six Degrees
	</div>

	<!-- Autocomplete Search from skeleton lib -->
	<div class="flex min-h-full max-w-full grow flex-col items-center p-4 pt-16">
		<div class="w-1/2">
			<input
				class="input w-full rounded-md border-stone-700 bg-neutral-700 px-4 py-2 text-zinc-300"
				type="search"
				name="demo"
				bind:value={input}
        on:input={() => selected = false}
				placeholder="Search..."
			/>
			<!-- WHY IS IT NOT WIDE ENOUGH; ALSO SIZE CHANGES BASED ON RESULT; FIXES IF MIN W = FULL BUT THATS CLAPPED -->
			{#if input !== '' && !selected}
				<div class="card mt-2 w-full rounded-md bg-neutral-700 p-2 text-zinc-100" tabindex="-1">
					<Autocomplete bind:input options={test} on:selection={onSelect} />
				</div>
			{/if}
		</div>
	</div>

	<!-- <SixDegreeScreen /> -->

	<div class="flex flex-col items-center justify-center p-20">
		<h1 class="pb-4 text-3xl font-bold text-zinc-100">Results</h1>
		<!-- TODO: figure out getting server's response here -->
		<p class="p-1 text-zinc-100">meow conncets to miao by sahdjsada</p>
		<p class="p-1 text-zinc-100">miao connects to goon by msadnsamfasd</p>
		<p class="p-1 text-zinc-100">goon connects to tuah by dnsaddn</p>
	</div>

	<div class="mt-auto flex w-full flex-col items-center justify-center p-8 pb-32">
		<div class="flex w-full items-center justify-center">
			<div class="px-20">
				<div class="text-2xl font-bold text-zinc-100">Previous Guesses<br /></div>
				<div class="text-center text-zinc-100">placeholder</div>
			</div>
			<div class="px-20">
				<div class="text-2xl font-bold text-zinc-100">Guesses Left<br /></div>
				<div class="text-center text-zinc-100">placeholder</div>
			</div>
		</div>
	</div>
</div>
