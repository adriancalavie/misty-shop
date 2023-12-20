<script lang="ts">
	import { Badge, Button, Card, CardPlaceholder, Rating } from 'flowbite-svelte';

	let link: string | null = null;
	export let id: number;

	async function getImageUrl() {
		console.log(`loading ${id}`);
		return new Promise((resolve, reject) => {
			link = `https://api.slingacademy.com/public/sample-products/${id}.png`;
			resolve(link);
		});
	}
</script>

<div class="w-full">
	{#await getImageUrl()}
		<CardPlaceholder class="w-full" padding="none" size="xl" />
	{:then src}
		<Card padding="none">
			<a href="/">
				<img class="rounded-20 mx-auto p-3" src={link} alt="product 1" />
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
	{:catch error}
		<span>{`Error loading ${id}: ${error.message}`} </span>
	{/await}
</div>

<style>
	.rounded-20 {
		border-radius: 20px;
	}
</style>
