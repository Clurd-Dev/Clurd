<script>
	import 'uikit/dist/css/uikit.css';
	import 'uikit/dist/js/uikit.js';
	import 'uikit/dist/js/uikit-icons.js';
	export let goback, getfile, path;
</script>

<svelte:head>
	<link rel="stylesheet" href="/css/toolbox.css" />
</svelte:head>
<div class="uk-text-center" uk-grid>
	<div class="uk-width-1-6@m">
		<img src="/images/back.png" alt="back" on:click={goback} class="toolicon" />
	</div>
	<div class="uk-width-1-6@m">
		<img
			src="/images/refresh.png"
			alt="refresh"
			on:click={getfile(path)}
			style="vertical-align:baseline"
			class="toolicon"
		/>
	</div>
</div>
<p class="path">Current path: {path}</p>

<style>
	.path {
		margin: 20px;
	}
</style>
