<script>
	let roll = 45;
	let pitch = 0;
	let heave = 0;

	$: rollTransform = `rotate(${roll}, 106.5, 100)`; // Roll 중심 기준 회전
	$: pitchTransform = `rotate(${pitch}, 106.5, 100)`; // Pitch 중심 기준 회전
	$: heaveOffset = -heave;
</script>

<div class="roll-pitch-heave-container">
	<div class="visual">
		<svg
			width="207"
			height="200"
			viewBox="0 0 207 200"
			fill="none"
			xmlns="http://www.w3.org/2000/svg"
		>
			<!-- 배 모양 -->

			<!-- Roll -->
			<line
				x1="106.5"
				y1="100"
				x2="38.5"
				y2="100"
				stroke="#325B9A"
				stroke-width="2"
				transform={rollTransform}
			/>

			<!-- Pitch -->
			<line
				x1="106.5"
				y1="100"
				x2="106.5"
				y2="168"
				stroke="#325B9A"
				stroke-width="2"
				transform={pitchTransform}
			/>

			<!-- Heave -->
			
			
		</svg>
	</div>
	<div class="data">
		<div class="data-item">
			<div class="value">{roll.toFixed(1)}°</div>
			<div class="label">Roll</div>
		</div>
		<div class="data-item">
			<div class="value">{pitch.toFixed(1)}°</div>
			<div class="label">Pitch</div>
		</div>
		<div class="data-item">
			<div class="value">{heave.toFixed(1)}</div>
			<div class="label">Heave</div>
		</div>
	</div>
</div>

<style>
	.roll-pitch-heave-container {
		padding: 1.5rem;
		font-family: Arial, sans-serif;
	}

	.visual {
		margin-bottom: 1.5rem;
	}

	.data {
		display: flex;
		justify-content: space-between;
		width: 100%;
	}

	.data-item {
		text-align: center;
	}

	.value {
		font-size: 1.2rem;
		font-weight: bold;
		color: #3366cc;
	}

	.label {
		font-size: 0.9rem;
		color: #666;
	}
</style>
