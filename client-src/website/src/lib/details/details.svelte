<script lang="ts">
	import { DialogContent } from 'svelte-dialogs';

	export let ls: Array<object>,
		name_file: string,
		path: string,
		current_name: string,
		file_name: string;
	name_file = name_file.replace('http://localhost:8000/', '');
	console.log(file_name);
	console.log(ls);
</script>

<DialogContent>
	<h1 slot="header">{file_name}</h1>
	<svelte:fragment slot="body">
		{#each ls as file}
			{#if file.file == file_name}
				<p>Filename: {file.file}</p>
				<p>Path: {path}</p>
				<p>Read-File: {file.read_only}</p>
				{#if file.size > 100000}
					<p>Size: {file.size / 1000000} Mb</p>
				{:else}
					<p>Size: {file.size / 1000} Kb</p>
				{/if}
				{#if file.md5 != 'dir'}
					<p>MD5: {file.md5}</p>
				{/if}
			{/if}
		{/each}
	</svelte:fragment>
</DialogContent>
