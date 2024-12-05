<!-- Imports -->
<script lang="ts">
	import SixDegreeScreen from '$lib/components/SixDegrees/SixDegreeScreen.svelte';

	// Supports weights 100-900
	import '@fontsource-variable/inter';
	import movies from '$lib/data/output.json';
	import { Autocomplete } from '@skeletonlabs/skeleton';
	import type { AutocompleteOption } from '@skeletonlabs/skeleton';
	import { popup } from '@skeletonlabs/skeleton';
	import type { PopupSettings } from '@skeletonlabs/skeleton';
	import LinkedList from '../Visualization/LinkedList.svelte';
	import type { Connection } from '../Visualization/LinkedTypes';

	const path: Connection[] = [
		{
			from: 'Wonka',
			to: 'Dune',
			actor: ['Timothee Chalamet', 'Matthew', 'Alexander Wang']
		},
		{
			from: 'Dune',
			to: 'Xeon',
			actor: ['Alexander Wang']
		},
		{
			from: 'Xeon',
			to: 'Paper Crush',
			actor: ['Dominic']
		}
	];

	let popupSettings: PopupSettings = {
		event: 'focus-click',
		target: 'popupAutocomplete',
		placement: 'bottom'
	};

	let input: string = '';

	const movieOptions: AutocompleteOption<string>[] = movies.movies.map((val) => ({
		value: val,
		label: val
	}));

	function onSelect(event: CustomEvent<AutocompleteOption<string>>): void {
		input = event.detail.label;
		// TODO: SEND REQUEST TO SERVER THEN DISPLAY RESULT
	}
</script>

<div class="m-0 flex min-h-screen w-full flex-col justify-between bg-neutral-900">
	<div
		class="flex h-10 w-full items-center justify-center pb-8 pt-32 text-center text-5xl font-bold text-zinc-100"
	>
		Six Degrees
	</div>

	<!-- Autocomplete Search from skeleton lib -->
	<div class="flex min-h-full max-w-full grow flex-col items-center p-4 pt-16">
		<div class="flex w-1/2 flex-col gap-y-5">
			<input
				class="input w-full rounded-md border-stone-700 bg-neutral-700 px-4 py-2 text-zinc-300"
				type="search"
				name="autocomplete-search"
				bind:value={input}
				placeholder="Movie 1..."
				use:popup={popupSettings}
			/>
			<!-- <div 
        data-popup="popupAutocomplete"
        class="card mt-2 w-1/2 max-h-48 rounded-md bg-neutral-700 p-2 text-zinc-100 overflow-y-auto" tabindex="-1">
        <Autocomplete 
          bind:input={input}
          options={movieOptions}
          on:selection={onSelect} 
        />
      </div> -->

			<input
				class="input w-full rounded-md border-stone-700 bg-neutral-700 px-4 py-2 text-zinc-300"
				type="search"
				name="autocomplete-search"
				bind:value={input}
				placeholder="Movie 2..."
				use:popup={popupSettings}
			/>
		</div>
	</div>

	<!-- <SixDegreeScreen /> -->

	<div class="flex flex-col items-center justify-center p-32 gap-y-10">
		<h1 class="pb-4 text-3xl font-bold text-zinc-100">Results</h1>
		<!-- TODO: figure out getting server's response here -->
		<LinkedList connections={{ path: path }} />
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
