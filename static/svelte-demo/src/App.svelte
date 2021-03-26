<svelte:options tag="svelte-demo" />

<script>
	export let something = "Default";
</script>

<svelt-demo {something}>
	<div class="svelte-main">
		<h1>
			Hello {something}!
		</h1>
		<p>
			Visit the <a href="https://svelte.dev/tutorial">Svelte tutorial</a> to
			learn how to build Svelte apps.
		</p>
	</div>
</svelt-demo>

<style>
	.svelte-main {
		text-align: center;
		padding: 1em;
		max-width: 240px;
		margin: 0 auto;
	}

	h1 {
		color: #ff3e00;
		text-transform: uppercase;
		font-size: 4em;
		font-weight: 100;
	}

	@media (min-width: 640px) {
		.svelte-main {
			max-width: none;
		}
	}
</style>
