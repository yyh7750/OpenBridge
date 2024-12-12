<script>
	import { onMount } from 'svelte';

	export let latitude = 41.05735;
	export let longitude = -71.27794;

	let currentTime = new Date();

	function updateTime() {
		currentTime = new Date();
	}

	onMount(() => {
		const interval = setInterval(updateTime, 1000);
		return () => clearInterval(interval);
	});

	function formatCoordinate(coord, isLatitude) {
		const absolute = Math.abs(coord);
		const degrees = Math.floor(absolute);
		const minutes = ((absolute - degrees) * 60).toFixed(3);
		const direction = isLatitude ? (coord >= 0 ? 'N' : 'S') : coord >= 0 ? 'E' : 'W';

		return `${String(degrees).padStart(2, '0')}°${String(minutes).padStart(6, '0')}' ${direction}`;
	}

	function formatTime(date) {
		return date.toLocaleTimeString('en-US', { hour12: false });
	}

	function formatDate(date) {
		return date.toLocaleDateString('en-US', { year: 'numeric', month: '2-digit', day: '2-digit' });
	}
</script>

<!-- HTML start -->
<div class="coordinate-time-container">
	<div class="time-date">
		<div>{formatCoordinate(latitude, true)}</div>
		<div>{formatCoordinate(longitude, false)}</div>
	</div>
	<div class="divider"></div>
	<div class="time-date">
		<div>{formatTime(currentTime)}</div>
		<div>{formatDate(currentTime)}</div>
	</div>
</div>

<!-- HTML end -->

<style>
	.coordinate-time-container {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 1rem;
		/* font-family: Arial, sans-serif; */
	}

	.time-date {
		width: 100%;
		display: flex;
		flex-direction: column;
		align-items: center;
		padding: 0.5rem 0;
		font-size: 1.2rem;
		font-weight: bold;
		color: #818181;
	}

	.divider {
		width: 100%;
		height: 1px;
		background-color: #ddd;
		margin: 0.5rem 0;
	}
</style>
