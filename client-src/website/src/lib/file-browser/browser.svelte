<script lang="ts">
	import { copy } from '$lib/ts/copy';

	import { DialogContent } from 'svelte-dialogs';
	import { copyfs } from '../ts/io';
	const ENDPOINT = 'http://localhost:8000/getfiles';
	export let ls: Array<object>, path: string, current_name: string, current_file: string;
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
		console.log(current_file.replace('http://localhost:8000/', ''));
		path = path + e;
		current_name = e;
		getfile(path);
	}
</script>

<DialogContent>
	<h1 slot="header">MY COMPONENT</h1>
	<svelte:fragment slot="body">
		{#each ls as folder}
			{#if folder.md5 == 'dir' && folder.file.split('.')[1] == null}
				<p on:click={() => test(folder.file + '/')}>{folder.file}</p>
			{/if}
		{/each}
		<button on:click={copyfs(current_file.replace('http://localhost:8000/', ''), path)}
			>Copy the file here</button
		>
	</svelte:fragment>
</DialogContent>

<style>
	button {
		color: red;
	}
</style>
