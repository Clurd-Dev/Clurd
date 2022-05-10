<script lang="ts">
	import { dialogs } from "svelte-dialogs";
	import Reader from '../lib/editor.svelte';
	const ENDPOINT:String = 'http://localhost:8000/getfiles';
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
		var xhr = new XMLHttpRequest();
		xhr.open("POST", ENDPOINT , true);
		xhr.onreadystatechange = function() { 
			if (this.readyState === XMLHttpRequest.DONE && this.status === 200) {
				console.log(JSON.parse(this.response))
				ls = JSON.parse(this.response);
			}
		}
		xhr.send(JSON.stringify({folder:folder}));
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
	<div class="grid-item" on:click={() => dialogs.modal(Reader, { filename: lsraw.file })}>
			<img src="file.png" class="icon" alt="file"/>
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
