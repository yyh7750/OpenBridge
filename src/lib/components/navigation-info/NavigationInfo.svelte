<script>
	import '@oicl/openbridge-webcomponents/dist/navigation-instruments/instrument-field/instrument-field.js';
	import CoordinateTimeDisplay from '$lib/components/navigation-info/CoordinateTimeDisplay.svelte';

	let instrumentFieldArgs = {
		hasSetpoint: false,
		value: 1.3,
		fractionDigits: 2,
		degree: false,
		tag: 'Offset',
		unit: 'm',
		hasSource: false
	};
</script>

<div class="container">
	<div class="gird-item">
		<CoordinateTimeDisplay></CoordinateTimeDisplay>
	</div>
	<div class="gird-item">
		<obc-instrument-field {...instrumentFieldArgs}></obc-instrument-field>
		<obc-instrument-field {...instrumentFieldArgs}></obc-instrument-field>
	</div>
	<div class="gird-item">
		<obc-instrument-field {...instrumentFieldArgs}></obc-instrument-field>
		<obc-instrument-field {...instrumentFieldArgs}></obc-instrument-field>
	</div>
	<div class="gird-item">
		<obc-instrument-field {...instrumentFieldArgs}></obc-instrument-field>
		<obc-instrument-field {...instrumentFieldArgs}></obc-instrument-field>
	</div>
	<div class="gird-item">
		<obc-instrument-field {...instrumentFieldArgs}></obc-instrument-field>
		<obc-instrument-field {...instrumentFieldArgs}></obc-instrument-field>
	</div>
	<div class="gird-item">
		<obc-instrument-field {...instrumentFieldArgs}></obc-instrument-field>
		<obc-instrument-field {...instrumentFieldArgs}></obc-instrument-field>
	</div>
</div>

<style>
	.container {
        width: 100%;
        height: 100%;
		display: grid;
		grid-template-columns: 1fr 1fr 1fr;
		grid-template-rows: 1fr 1fr;
		gap: 4px; /* 요소 간의 간격 */
		margin-top: 4px;
	}
	.container > div {
        border: solid lightgray 1px;
        text-align: -webkit-center;
		align-content: center;
		background-color: white;
		box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
		border: 1px solid #ddd;
		border-radius: 8px;
	}
</style>
