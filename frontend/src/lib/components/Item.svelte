<script lang="ts">
	import { Badge, Button, Card, CardPlaceholder, Rating } from 'flowbite-svelte';

	export let dataImported = false;
	export let id: number;
	const getImageUrl = () => {
		const link = `https://api.slingacademy.com/public/sample-products/${id}.png`;
		console.log(link);
		return link;
	};
	let imageUrl = getImageUrl();
</script>

<div class="w-full">
	{#if dataImported}
		<Card padding="none">
			<a href="/">
				<img class="rounded-20 mx-auto p-3" src={imageUrl} alt="product 1" />
			</a>
			<div class="px-5 pb-5">
				<a href="/">
					<h5 class="text-xl font-semibold tracking-tight text-gray-900 dark:text-white">
						Apple Watch Series 7 GPS, Aluminium Case, Starlight Sport
					</h5>
				</a>
				<Rating rating={5} size={24} class="mb-5 mt-2.5">
					<Badge slot="text" class="ms-3">4</Badge>
				</Rating>
				<div class="flex items-center justify-between">
					<span class="text-3xl font-bold text-gray-900 dark:text-white">$599</span>
					<Button href="/">Buy now</Button>
				</div>
			</div>
		</Card>
	{:else}
		<CardPlaceholder class="w-full" padding="none" size="xl" />
	{/if}
</div>

<style>
	.rounded-20 {
		border-radius: 20px;
	}
</style>
