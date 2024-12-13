<script>

    // todo 객체 하나 통으로 받아서 분리하기
	export let instrumentFieldArgsCourse = {
		value: 10,
		unit: '°',
		tag: 'Course',
        size: 'small'
	};

	export let instrumentFieldArgsRot = {
		value: 10,
		unit: '°/min',
		tag: 'ROT'
	};

	export let instrumentFieldArgsRadius = {
		value: 10,
		unit: '°',
		tag: 'Radius'
	};

	// Next Leg 데이터
	export let nextLegData = {
		course: 0,
		distanceToWOL: 0.9,
		timeToWOL: '0:12'
	};
</script>

<div class="steering-mode-container">
	<div class="header">
		<div class="title">Steering mode</div>
		<div style="color: red;">TODO 드롭박스로</div>
	</div>

	<div class="instrument-fields">
		<obc-instrument-field {...instrumentFieldArgsCourse}></obc-instrument-field>
		<obc-instrument-field {...instrumentFieldArgsRot}></obc-instrument-field>
		<obc-instrument-field {...instrumentFieldArgsRadius}></obc-instrument-field>
	</div>

	<div class="divider"></div>

	<div class="next-leg">
		<div class="next-leg-item">
			<div class="value">{nextLegData.course}</div>
			<div class="label">Course</div>
		</div>
		<div class="next-leg-item">
			<div class="value">{nextLegData.distanceToWOL}</div>
			<div class="label">DST WOL NM</div>
		</div>
		<div class="next-leg-item">
			<div class="value">{nextLegData.timeToWOL}</div>
			<div class="label">Time to WOL</div>
		</div>
	</div>
</div>

<style>
	.steering-mode-container {
		padding: 1rem;
	}

    .title {
		font-size: 1rem;
		font-weight: bold;
		color: #333;
		margin-bottom: 1rem;
	}

	.header {
		display: flex;
		justify-content: space-between;
		margin-bottom: 1rem;
	}

	.instrument-fields {
		display: flex;
		justify-content: space-between;
		margin-bottom: 1rem;
	}

	.divider {
		width: 100%;
		height: 1px;
		background-color: #ddd;
		margin-bottom: 1rem;
	}

	.next-leg {
		display: flex;
		justify-content: space-between;
	}

	.next-leg-item {
		text-align: center;
	}

	.value {
		font-size: 1rem;
		font-weight: bold;
		color: #3366CC;
	}

	.label {
		font-size: 0.8rem;
		color: #666;
	}
</style>
