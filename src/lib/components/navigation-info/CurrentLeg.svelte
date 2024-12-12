<script>
	export let course = 114;
	export let distanceToWOL = 0; // 웨이포인트까지의 거리
	export let timeToWOL = '00:00'; // 웨이포인트까지의 시간

	// 화살표 회전을 위한 동적 스타일 업데이트
	$: arrowRotation = `rotate(${course}deg)`;
</script>

<div class="current-leg-container">
	<div class="title">Current leg</div>
	<div class="content">
		<div class="info">
			<div class="item">
				<div class="value">{course}°</div>
				<div class="label">Course</div>
			</div>
			<div class="item">
				<div class="value">{distanceToWOL}</div>
				<div class="label">Dist to WOL NM</div>
			</div>
			<div class="item">
				<div class="value">{timeToWOL}</div>
				<div class="label">Time to WOL</div>
			</div>
		</div>
		<div class="visual">
			<div class="arrow-box">
				<svg width="150" height="150" viewBox="0 0 100 100">
					<!-- 화살표 -->
					<g transform="translate(50, 50) rotate({course}) translate(-50, -50)">
						<!-- 화살표 머리 -->
						<path d="M50 10 L45 25 L55 25 Z" fill="#3366CC" />
						<!-- 화살표 몸통 -->
						<rect x="48" y="25" width="4" height="20" fill="#3366CC" />
					</g>
					<!-- 점선 -->
					<line
						x1="50"
						y1="50"
						x2="50"
						y2="90"
						stroke="#666"
						stroke-dasharray="4"
						stroke-width="2"
					/>
					<!-- 원 -->
					<circle cx="50" cy="50" r="4" fill="#3366CC" />
				</svg>
			</div>
		</div>
	</div>
</div>

<style>
	.current-leg-container {
		display: flex;
		flex-direction: column;
		padding: 1rem;
	}

	.title {
		font-size: 1rem;
		font-weight: bold;
		color: #333;
		margin-bottom: 1rem;
	}

	.content {
		display: flex;
		justify-content: space-between;
		align-items: stretch; /* 자식 요소의 높이를 동일하게 맞춤 */
		height: 100%; /* 부모 높이를 명시적으로 설정 */
	}

	.info {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
		width: 45%;
	}

	.item {
		text-align: left;
	}

	.value {
		font-size: 1.2rem;
		font-weight: bold;
		color: rgba(50, 91, 154, 1);
	}

	.label {
		font-size: 0.7rem;
		color: #666;
	}

	.visual {
		display: flex;
		justify-content: center;
		align-items: center;
		background-color: #f8f8f8;
		border-radius: 8px;
		width: 55%;
		height: auto;
	}

	.arrow-box {
		width: 150px;
		height: 150px;
	}
</style>
