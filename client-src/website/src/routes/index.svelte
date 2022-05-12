<script lang="ts">
	import { dialogs } from 'svelte-dialogs';
	import Reader from '../lib/editor.svelte';
	const ENDPOINT: String = 'http://localhost:8000/getfiles';
	export const prerender = true;
	import { onMount } from 'svelte';
	let path = './';
	let ls: Array<object> = [];
	async function test(e: String) {
		path = path + e;
		//console.log(e);
		await getfile(e);
	}
	async function getfile() {
		ls = [];
		//console.log(path);
		
		//console.log(path);
		var xhr = new XMLHttpRequest();
		xhr.open('POST', ENDPOINT, true);
		xhr.onreadystatechange = function () {
			if (this.readyState === XMLHttpRequest.DONE && this.status === 200) {
				//console.log(JSON.parse(this.response));
				ls = JSON.parse(this.response);
			}
		};
		xhr.send(JSON.stringify({ folder: path }));
	}
	function goback() {
		if(path == "./"){
			dialogs.alert("Can't go back through home");
		}
		let tempath = path.split("/");
		tempath.pop();
		path = tempath.join('/');
		getfile();
	}
	onMount(() => {
		getfile();
	});
</script>

<svelte:head>
	<title>Home</title>
	<meta name="description" content="Svelte demo app" />
	<script src="//naver.github.io/egjs-grid/release/latest/dist/grid.min.js"></script>
</svelte:head>
<img src="/images/back.png" alt="back" on:click={goback}/>
<section>
	<div class="grid-container">
		{#each ls as lsraw}
			{#if lsraw.md5 == 'dir'}
				<div class="grid-item" on:click={() => test(lsraw.file + "/")}>
					<img src="/images/folder.png" class="icon" alt="folder" />
					<p>{lsraw.file}</p>
				</div>
			{:else}
				<div
					class="grid-item"
					on:click={() => dialogs.modal(Reader, { filename: path + lsraw.file })}
				>
				{#if lsraw.file.split(".")[1] != "js"}
					<img src="/images/file.png" class="icon" alt="file" />
					{:else}
					<img src="/images/js.png" alt="filejs" class="icon" />
				{/if}
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
	.icon {
		width: 128px;
	}
</style>
