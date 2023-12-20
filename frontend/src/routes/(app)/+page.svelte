<script>
	import Item from '$lib/components/Item.svelte';
	import { Button } from 'flowbite-svelte';

	let dataImported = false;
	let ids = Array.from({ length: 100 }, (_, i) => i + 1);
</script>

<h1>HOME</h1>
<Button on:click={() => (dataImported = !dataImported)}>Toggle</Button>
<div class="mx-10 grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5 gap-4">
	{#each ids as id}
		<Item {id} />
	{/each}
</div>
