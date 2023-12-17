<script lang="ts">
	import { page } from '$app/stores';
	import { Button, Navbar, NavLi, NavUl } from 'flowbite-svelte';
	$: activeUrl = $page.url.pathname;

</script>

{#key activeUrl}
	<Navbar let:hidden>
		<NavUl {hidden} {activeUrl}>
			<NavLi href="/">Home</NavLi>
			<NavLi href="/about">About</NavLi>
			<NavLi href="/contact">Contact</NavLi>
			<NavLi href="/orders">Orders</NavLi>
			<NavLi href="/profile">Profile</NavLi>
		</NavUl>
		<Button>click me</Button>
	</Navbar>
{/key}

<slot />
