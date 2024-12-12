<script>
	import '@oicl/openbridge-webcomponents/dist/navigation-instruments/compass/compass.js';
	import '@oicl/openbridge-webcomponents/dist/navigation-instruments/instrument-field/instrument-field.js';

	export let compass;

	$: headingAdvices = compass?.heading_advices || {};
	$: instrumentFieldArgs = compass?.instrumentFields || {};
</script>

{#if compass}
	<div class="container">
		<div>
			<obc-instrument-field {...instrumentFieldArgs.hdg}></obc-instrument-field>
			<obc-instrument-field {...instrumentFieldArgs.cog}></obc-instrument-field>
			<obc-instrument-field {...instrumentFieldArgs.rot}></obc-instrument-field>
		</div>
		<div class="wrapper">
			<obc-compass {...compass.headingAdvices}></obc-compass>
		</div>
		<div>
			<obc-instrument-field {...instrumentFieldArgs.wind}></obc-instrument-field>
			<obc-instrument-field {...instrumentFieldArgs.current}></obc-instrument-field>
		</div>
	</div>
{:else}
	<p>Loading data...</p>
{/if}

<style>
	.container {
		width: 100%;
		height: 100%;
		display: flex;
		justify-content: space-around;
		align-items: center;
		background-color: white;
		box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
		border: 1px solid #ddd;
		border-radius: 8px;
	}

	.container > div {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.wrapper {
		width: 512px;
		height: 512px;
	}
</style>
