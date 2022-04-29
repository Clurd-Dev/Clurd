<script>
	export const prerender = true;
	import { onMount } from 'svelte';
	export let gap;
	export let align;
	let ls=[];
	onMount(()=>{
		fetch('http://localhost:3001/getfile', {
			method: 'POST', // or 'PUT'
			headers: {
				'Content-Type': 'application/json',
			},
			body: JSON.stringify({
				folder:'./'
			}),
			})
			.then(response => response.json())
			.then(data => {
			ls = data;
			console.log(data)
			})
			.catch((error) => {
			console.error('Error:', error);
			});
	})
</script>


<svelte:head>
	<title>Home</title>
	<meta name="description" content="Svelte demo app" />
	<script src="//naver.github.io/egjs-grid/release/latest/dist/grid.min.js"></script>
</svelte:head>


<section>
<div class="grid-container">
{#each ls as lsraw}
	{#if lsraw.file.split('.')[1] == null}
	<div class="grid-item">
		<img src="folder.png" class="icon"/>
		<p>{lsraw.file}</p>
	</div>
	{:else}
	<div class="grid-item">
		<img src="file.png" class="icon"/>
		<p>{lsraw.file}</p>
	</div>
	{/if}
{/each}
</div>
</section>

<style>
	.grid-container {
	  display: grid;
	  grid-template-columns: auto auto auto;
	  padding: 10px;
	}
	.grid-item {
	  border: 1px solid rgba(0, 0, 0, 0.8);
	  padding: 20px;
	  font-size: 30px;
	  text-align: center;
	}
	.icon{
		width:128px;
	}
	</style>
