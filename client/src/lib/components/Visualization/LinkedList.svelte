<script lang="ts">
	import type { Connection, Path } from './LinkedTypes';
	import Arrow from './molecules/Arrow.svelte';
	import MovieCard from './molecules/MovieCard.svelte';

	interface Props {
		connections: Path;
	}

	type ItemType = 'Movie' | 'Actors';

	interface ChainItem {
		value: string | string[];
		type: ItemType;
	}

	let { connections }: Props = $props();

	const trimMovies = (conn: Connection[]) => {
		if (conn.length === 0) return [];
		const chain: ChainItem[] = [];
		const seenMovie = new Set();

		conn.forEach((connection, index) => {
			if (!seenMovie.has(connection.from)) {
				chain.push({
					value: connection.from,
					type: 'Movie'
				});
				seenMovie.add(connection.from);
			}

			chain.push({
				value: connection.actor,
				type: 'Actors'
			});

			if (!seenMovie.has(connection.to)) {
				chain.push({
					value: connection.to,
					type: 'Movie'
				});
				seenMovie.add(connection.to);
			}
		});

		return chain;
	};

	const trimmed = trimMovies(connections.path);
</script>

<div class="flex flex-col items-center justify-center gap-y-4 w-full">
	{#each trimmed as { value, type }}
		{#if type === 'Movie'}
			<MovieCard movie={value as string} />
		{:else}
            <Arrow actors={value as string[]} />
        {/if}
	{/each}
</div>
