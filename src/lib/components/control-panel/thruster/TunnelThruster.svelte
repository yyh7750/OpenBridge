<script>
	import '@oicl/openbridge-webcomponents/dist/navigation-instruments/badge-command/badge-command.js';
	import '@oicl/openbridge-webcomponents/dist/navigation-instruments/thruster/thruster.js';

	let badgeCommand = {
		large: true,
		status: 'in-command'
	};

	let tunnelThruster = {
		tunnel: true,
		thrust: 50,
		setpoint: 50,
		state: 'inCommand'
	};
</script>

<div class="bow-thruster-wrapper">
	<div class="badge">
		<obc-badge-command {...badgeCommand}></obc-badge-command>
		<span>1. BOW THRUSTER</span>
	</div>
	<obc-thruster {...tunnelThruster}></obc-thruster>
</div>

<style>
</style>
