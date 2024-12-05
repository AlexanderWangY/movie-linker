<script lang="ts">
	interface Props {
		actors: string[];
	}

	const { actors }: Props = $props();
</script>

<div class="flex flex-col items-center justify-center">
	<div class="h-10 w-1 bg-zinc-500"></div>
	<div class="text-center my-2">
		{#each actors as actor}
			<p class="text-zinc-300 text-lg">{actor}</p>
		{/each}
	</div>
    <div class="h-10 w-1 bg-zinc-500 relative">
        <div class="arrow absolute h-3 w-3 rotate-45 bg-zinc-500 top-3/4 left-1/2 -translate-x-1/2"></div>
    </div>
</div>
