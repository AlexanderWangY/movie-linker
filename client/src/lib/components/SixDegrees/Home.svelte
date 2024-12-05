<!-- Imports -->
<script lang="ts">
	import SixDegreeScreen from '$lib/components/SixDegrees/SixDegreeScreen.svelte';

	// Supports weights 100-900
	import '@fontsource-variable/inter';
	import movies from '$lib/data/output.json';
	import type { AutocompleteOption } from '@skeletonlabs/skeleton';
	import { popup } from '@skeletonlabs/skeleton';
	import type { PopupSettings } from '@skeletonlabs/skeleton';
	import LinkedList from '../Visualization/LinkedList.svelte';
	import type { Connection } from '../Visualization/LinkedTypes';
	import { RadioGroup, RadioItem } from '@skeletonlabs/skeleton';

	const MovieSet = new Set();

	movies.movies.forEach((movie) => MovieSet.add(movie));

	interface Response {
		path: Connection[];
		time: number;
		traversal_count: number;
	}

	interface Path {
		path: Connection[];
	}

	let path: Path = {
		path: []
	};

	let popupSettings: PopupSettings = {
		event: 'focus-click',
		target: 'popupAutocomplete',
		placement: 'bottom'
	};

	let movie1_input: string = '';
	let movie2_input: string = '';

	let value: number = 0;

	const fetchConnections = async () => {
		if (!MovieSet.has(movie1_input) || !MovieSet.has(movie2_input)) {
			console.log('Error, movies not found');
			return;
		}

		path.path = [];

		const res = await fetch(
			`http://localhost:8080/${value === 1 ? 'dfs_connected' : 'bfs_connected'}?from=${movie1_input}&to=${movie2_input}`,
			{ mode: 'cors' }
		);

		const data: Response = await res.json();

		console.log(data.path);

		path.path = data.path;
	};
</script>

<div class="m-0 flex min-h-screen w-full flex-col justify-between bg-neutral-900">
	<div
		class="flex h-10 w-full items-center justify-center pb-8 pt-32 text-center text-5xl font-bold text-zinc-100"
	>
		Six Degrees
	</div>

	<div class="flex flex-col items-center justify-center pt-10">
		<RadioGroup active="bg-neutral-900 text-zinc-300" hover="hover:bg-neutral-400">
			<RadioItem bind:group={value} name="BFS" value={0}>BFS</RadioItem>
			<RadioItem bind:group={value} name="DFS" value={1}>DFS</RadioItem>
		</RadioGroup>
	</div>

	<!-- Autocomplete Search from skeleton lib -->
	<div class="flex min-h-full max-w-full grow flex-col items-center p-4 pt-16">
		<div class="flex w-1/2 flex-col gap-y-5">
			<input
				class="input w-full rounded-md border-stone-700 bg-neutral-700 px-4 py-2 text-zinc-300"
				type="search"
				name="autocomplete-search"
				bind:value={movie1_input}
				placeholder="Movie 1..."
				use:popup={popupSettings}
			/>

			<input
				class="input w-full rounded-md border-stone-700 bg-neutral-700 px-4 py-2 text-zinc-300"
				type="search"
				name="autocomplete-search"
				bind:value={movie2_input}
				placeholder="Movie 2..."
				use:popup={popupSettings}
			/>
		</div>

		<button
			on:click={fetchConnections}
			class="mt-10 w-1/4 rounded-lg bg-blue-800 px-5 py-3 hover:cursor-pointer hover:bg-blue-950"
			>Find!</button
		>
	</div>

	<!-- <SixDegreeScreen /> -->

	<div class="flex flex-col items-center justify-center gap-y-10 p-32">
		<h1 class="pb-4 text-3xl font-bold text-zinc-100">Results</h1>
		<!-- TODO: figure out getting server's response here -->
		{#if path.path.length !== 0}
			<LinkedList connections={path} />
		{/if}
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
