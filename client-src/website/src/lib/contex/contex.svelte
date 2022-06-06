<script lang="ts">
	export let current_file: string,
		path: string,
		remove: any,
		getfile: any,
		rename: any,
		ls: Array<object>,
		current_name: string,
		only_file: string, old_path: string;
	let url: string;
	import { dialogs } from 'svelte-dialogs';
	import { copy } from '../ts/copy';
	import { onMount } from 'svelte';
	import FSBrowser from '../file-browser/browser.svelte';
	import Details from '$lib/details/details.svelte';
	import './contex.css';
	onMount(()=>{
		url = location.origin;
		console.log(path);
	});
</script>

<div id="contextMenu" class="context-menu" style="display: none">
	<ul class="menu">
		<li class="rename">
			<a href="#0" on:click={rename}><i class="fa fa-pencil" aria-hidden="true" /> Rename</a>
		</li>
		<li class="link">
			<a href="#0" on:click={copy(current_file)}
				><i class="fa fa-link" aria-hidden="true" /> Copy Link Address</a
			>
		</li>
		<li class="copy">
			<a
				href="#0"
				on:click={() =>
					dialogs.modal(FSBrowser, {
						ls: ls,
						path: path,
						current_name: current_name,
						current_file: current_file,
						file: only_file,
						old_path: old_path
					})}
			>
				<i class="fa fa-copy" aria-hidden="true" /> Copy/Move to
			</a>
		</li>
		<li class="download">
			<a href={current_file}><i class="fa fa-download" aria-hidden="true" /> Download</a>
		</li>
		<li
			class="trash"
			on:click={remove(current_file.replace(url, ''), getfile, path, only_file)}
		>
			<a href="#"><i class="fa fa-trash" aria-hidden="true" /> Delete</a>
		</li>
		<li
			class="trash"
			on:click={() =>
				dialogs.modal(Details, {
					ls: ls,
					name_file: current_file,
					path: path,
					current_name: current_name,
					file_name: only_file
				})}
		>
			<a href="#"><i class="fa fa-trash" aria-hidden="true" /> Details</a>
		</li>
	</ul>
</div>
