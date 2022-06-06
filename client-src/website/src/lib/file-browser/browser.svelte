<script lang="ts">
	import 'uikit/dist/css/uikit.css';
	import 'uikit/dist/js/uikit.js';
	import 'uikit/dist/js/uikit-icons.js';
	import { copy } from '$lib/ts/copy';
	import { onMount } from 'svelte';
	import { DialogContent } from 'svelte-dialogs';
	import { copyfs, movefs } from '../ts/io';
	export let ls: Array<object>, path: string, current_name: string, current_file: string, file: string, old_path: string;
	let url: string;
	let ENDPOINT: string;
	function getfile(pathpost: string) {
		const xhr = new XMLHttpRequest();
		xhr.open('POST', ENDPOINT, true);
		xhr.onreadystatechange = function () {
			if (this.readyState === XMLHttpRequest.DONE && this.status === 200) {
				ls = JSON.parse(this.response);
				console.log(ls);
			}
		};
		xhr.send(JSON.stringify({ folder: pathpost }));
	}
	async function test(e: string) {
		path = path + "/" + e;
		console.log(path);
		current_name = e;
		getfile(path);
	}
	onMount(()=> {
		url = location.origin + '/';
		ENDPOINT = url + 'getfiles';
	});
</script>

<DialogContent>
	<h1 slot="header">Select the path where you want to move file</h1>
	<svelte:fragment slot="body">
		{#each ls as folder}
			{#if folder.dir == true && folder.file.split('.')[1] == null}
				<p on:click={() => test(folder.file + '/')}>{folder.file}</p>
			{/if}
		{/each}
		<p>
			<button on:click={copyfs(old_path + "/" + current_file.replace(url, ''), path, file)}
				>Copy the file here</button
			>
		</p>
		<p>
			<button on:click={movefs(old_path + "/" + current_file.replace(url, ''), path, file)}
				>Move the file here</button
			>
		</p>
	</svelte:fragment>
</DialogContent>

<style>
	button {
		color: red;
	}
</style>
