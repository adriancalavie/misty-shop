<script>
	import Item from '$lib/components/Item.svelte';
	import { Button } from 'flowbite-svelte';

	let dataImported = false;
	let ids = Array.from({ length: 100 }, (_, i) => i + 1);
</script>

<h1>HOME</h1>
<Button on:click={() => (dataImported = !dataImported)}>Toggle</Button>
<div
	class="3xl:grid-cols-6 4xl:grid-cols-7 mx-10 grid grid-cols-1 gap-4 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5 2xl:grid-cols-5"
>
	{#each ids as id}
		<Item {id} />
	{/each}
</div>
