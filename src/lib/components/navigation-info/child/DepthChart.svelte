<script>
	import Chart from 'chart.js/auto';
	import { onMount } from 'svelte';

	let data = [10, 20, 15, 40, 35];
	let chartCanvas;

	onMount(() => {
		const ctx = chartCanvas.getContext('2d');
		const chart = new Chart(ctx, {
			type: 'line',
			data: {
				labels: data.map((_, index) => index + 1), // 데이터의 인덱스를 기준으로 라벨 생성
				datasets: [
					{
                        tension: 0.4, // 그래프 곡선으로 표시
						backgroundColor: 'rgba(75, 192, 192, 0.2)',
						borderColor: 'rgba(75, 192, 192, 1)',
						borderWidth: 2,
						fill: true,
						data: data
					}
				]
			},
			options: {
				scales: {
					y: {
						position: 'right' // Y축 라벨 반전 (오른쪽)
					},
					x: {
						display: false
					}
				},
				plugins: {
					legend: {
						display: false
					}
				}
			}
		});
	});
</script>

<canvas bind:this={chartCanvas} id="myChart"></canvas>
