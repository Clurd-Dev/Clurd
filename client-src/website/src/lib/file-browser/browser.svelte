<script lang="ts">
	import 'uikit/dist/css/uikit.css';
	import 'uikit/dist/js/uikit.js';
	import 'uikit/dist/js/uikit-icons.js';
	import { copy } from '$lib/ts/copy';
	import { onMount } from 'svelte';
	import { DialogContent } from 'svelte-dialogs';
	import { copyfs, movefs } from '../ts/io';
	export let ls: Array<object>, path: string, current_name: string, current_file: string, file: string;
	let url: string;
	let ENDPOINT: string;
	function getfile(path: string) {
		const xhr = new XMLHttpRequest();
		xhr.open('POST', ENDPOINT, true);
		xhr.onreadystatechange = function () {
			if (this.readyState === XMLHttpRequest.DONE && this.status === 200) {
				ls = JSON.parse(this.response);
			}
		};
		xhr.send(JSON.stringify({ folder: path }));
	}
	async function test(e: string) {
		console.log(current_file.replace(url, ''));
		path = path + e;
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
			{#if folder.md5 == 'dir' && folder.file.split('.')[1] == null}
				<p on:click={() => test(folder.file + '/')}>{folder.file}</p>
			{/if}
		{/each}
		<p>
			<button on:click={copyfs(current_file.replace(url, ''), path, file)}
				>Copy the file here</button
			>
		</p>
		<p>
			<button on:click={movefs(current_file.replace(url, ''), path, file)}
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
