<script lang="ts">
	import Header from '$lib/header/Header.svelte';
	import { onMount } from 'svelte';

	import '../app.css';
	let available: string = '0',
		total: string = '0';
	function myFunction() {
		var x = document.getElementById('myTopnav');
		if (x.className === 'topnav') {
			x.className += ' responsive';
		} else {
			x.className = 'topnav';
		}
	}
	onMount(() => {
		var xhr = new XMLHttpRequest();
		xhr.open('POST', location.origin + '/space', true);
		xhr.onreadystatechange = function () {
			if (this.readyState === XMLHttpRequest.DONE && this.status === 200) {
				//console.log(JSON.parse(this.response));
				let temp = JSON.parse(this.response);
				available = (
					parseFloat((parseInt(temp.total) / 1000000000).toFixed(3)) -
					parseFloat((parseInt(temp.available) / 1000000000).toFixed(3))
				).toFixed(3);
				total = (parseInt(temp.total) / 1000000000).toFixed(3);
			}
		};
		xhr.send(JSON.stringify({ folder: './' }));
	});
</script>

<svelte:head>
	<link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/uikit@3.14.1/dist/css/uikit.min.css" />
	<script src="https://cdn.jsdelivr.net/npm/uikit@3.14.1/dist/js/uikit.min.js"></script>
	<script src="https://cdn.jsdelivr.net/npm/uikit@3.14.1/dist/js/uikit-icons.min.js"></script>
	<link
		rel="stylesheet"
		href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/4.7.0/css/font-awesome.min.css"
	/>
	<link rel="stylesheet" href="/css/nav.css" />
</svelte:head>
<Header />
<div class="topnav" id="myTopnav">
	<a href="/" class="active">Home</a>
	<a href="/settings">Settings</a>
</div>
<main>
	<slot />
</main>
<footer>
	<div align="center">
		<p>Space in use: {available} GB of {total} GB</p>
	</div>
	<div class="space">
		<progress class="uk-progress" value={available} max={total} />
	</div>
</footer>

<style>
	.space {
		margin: 20px;
	}
</style>
