<script lang="ts">
	import Header from '$lib/header/Header.svelte';
	import { onMount } from 'svelte';

	import '../app.css';
	let available: string = "0",
		total: string = "0";
	const ENDPOINT: string = 'http://localhost:8000/space';
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
		xhr.open('POST', ENDPOINT, true);
		xhr.onreadystatechange = function () {
			if (this.readyState === XMLHttpRequest.DONE && this.status === 200) {
				//console.log(JSON.parse(this.response));
				let temp = JSON.parse(this.response);
				available = (parseFloat((parseInt(temp.total) / 1000000000).toFixed(3)) - parseFloat((parseInt(temp.available) / 1000000000).toFixed(3))).toFixed(3);
				total = (parseInt(temp.total) / 1000000000).toFixed(3);
			}
		};
		xhr.send(JSON.stringify({ folder: './' }));
	});
</script>

<svelte:head>
	<!-- UIkit CSS -->
	<link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/uikit@3.14.1/dist/css/uikit.min.css" />

	<!-- UIkit JS -->
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
</div>
<main>
	<slot />
</main>
<footer>
	<div align="center">
		<p>Spazio utilizzato {available} GB of {total} GB</p>
	</div>
	<progress class="uk-progress" value={available} max={total} />
</footer>

<style>
</style>
