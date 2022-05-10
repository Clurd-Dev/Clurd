<script lang="ts">
	export const prerender = true;
	import { onMount } from 'svelte';
	export let gap;
	export let align;
	let ls:Array<object>=[];
		function test(e:String) {
			console.log(e)
			getfile(e)
		}
	function getfile(folder:String) {
		ls = [];
		fetch('http://localhost:3001/getfile', {
			method: 'POST', // or 'PUT'
			headers: {
				'Content-Type': 'application/json',
			},
			body: JSON.stringify({
				folder:folder
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
	}
	onMount(()=>{
		getfile("./")
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
	{#if lsraw.md5 == "dir"}
	<div class="grid-item" on:click={() => test(lsraw.file)}>
		<img src="folder.png" class="icon" alt="folder"/>
		<p>{lsraw.file}</p>
	</div>
	{:else}
	<div class="grid-item">
		<a href={"localhost:3001/" + lsraw.filename}>
			<img src="file.png" class="icon" alt="file"/>
			<p>{lsraw.file}</p>
		</a>
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
