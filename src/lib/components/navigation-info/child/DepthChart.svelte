<script>
	import Chart from 'chart.js/auto';
	import { onMount } from 'svelte';

	let data = [0, 0, 0, 0];
	let chartCanvas;
	let chart;

	function updateChart(newValue) {
		// 이전 데이터를 한 칸씩 왼쪽으로 밀기
		data.shift();
		data.push(newValue);

		chart.data.datasets[0].data = [...data];
		chart.update();
	}

	onMount(() => {
		const ctx = chartCanvas.getContext('2d');
		chart = new Chart(ctx, {
			type: 'line',
			data: {
				labels: data.map((_, index) => index + 1), // X축 라벨 (인덱스)
				datasets: [
					{
						label: '',
						backgroundColor: 'rgba(75, 192, 192, 0.2)',
						borderColor: 'rgba(75, 192, 192, 1)',
						borderWidth: 2,
						fill: true,
						tension: 0.4, // 곡선
						data: data
					}
				]
			},
			options: {
				scales: {
					y: {
						position: 'right', // Y축 라벨을 오른쪽으로 이동
					},
					x: {
						ticks: {
							display: false
						}
					}
				},
				plugins: {
					legend: {
						display: false // label hide
					}
				}
			}
		});

		// Test
		setInterval(() => {
			const newValue = Math.floor(Math.random() * 10);
			updateChart(newValue);
		}, 500);
	});
</script>

<canvas bind:this={chartCanvas} id="myChart"></canvas>
