<script lang='ts'>
    interface Props {
        movie: string
    }

    const { movie }: Props = $props();
</script>

<div class="bg-zinc-600 px-5 py-6 min-w-fit max-w-1/2 w-1/2 rounded-lg hover:bg-zinc-700 hover:cursor-pointer">
    <p class="text-white text-xl select-none text-center">{movie}</p>
</div>