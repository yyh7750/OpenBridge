<script>
    import StwDisplay from "./stw/StwDisplay.svelte";
    import TunnelThruster from "./thruster/TunnelThruster.svelte";
</script>

<div class="stw">
    <StwDisplay></StwDisplay>
</div>
<div class="thruster">
    <TunnelThruster></TunnelThruster>
</div>

<style>
    .stw {
        width: 500px;
        height: 100%;
    }

    .thruster {
        height: 100%;
    }
</style>