<script>
	import { onMount } from 'svelte';
	import { invoke } from '@tauri-apps/api/core';

	import NavigationPanel from '$lib/components/navigation-panel/NavigationPanel.svelte';
	import NavigationInfo from '$lib/components/navigation-info/NavigationInfo.svelte';
	import StwDisplay from '$lib/components/control-panel/stw/StwDisplay.svelte';

	let data = null;

	function fetchData() {
		setInterval(async () => {
			try {
				const result = await invoke('get_latest_data');
				data = result;
			} catch (error) {
				console.error('Error fetching data:', error);
			}
		}, 1000);
	}

	onMount(() => {
		fetchData();
	});
</script>

<div class="container">
	<div class="navigation-panel">
		<NavigationPanel compass = { data } />
	</div>
	<div class="navigation-info">
		<NavigationInfo />
	</div>
	<div class="control-panel">
		<StwDisplay />
	</div>
</div>

<style>
	.container {
		display: grid;
		grid-template-columns: 1.5fr 2fr;
		grid-template-rows: 1.5fr 1fr;
		gap: 10px; /* 영역 간 간격 */
		height: 100vh;
    background-color: #F7F7F7;
	}

	.navigation-panel,
	.navigation-info,
	.control-panel {
		display: flex;
		justify-content: center;
		align-items: center;
		width: 100%; /* 부모 컨테이너의 너비에 맞게 꽉 차게 */
		height: 100%; /* 부모 컨테이너의 높이에 맞게 꽉 차게 */
	}

	.navigation-panel {
		grid-column: 1 / 2;
		grid-row: 1 / 2;
	}

	.navigation-info {
		grid-column: 1 / 2;
		grid-row: 2 / 3;
	}

	.control-panel {
		grid-column: 2 / 3;
		grid-row: 1 / 3;
    background-color: white;
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
		border: 1px solid #ddd;
		border-radius: 8px;
	}
</style>
