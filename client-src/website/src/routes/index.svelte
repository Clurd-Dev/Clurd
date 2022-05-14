<script lang="ts">
	import { dialogs } from 'svelte-dialogs';
	import Reader from '../lib/editor.svelte';
	import { remove } from '../lib/ts/io';
	import { onMount } from 'svelte';
	import { rightClick, hideMenu } from '../lib/ts/menu';
	import Toolbox from '$lib/toolbox.svelte';
	import Contex from '$lib/contex/contex.svelte';
	const ENDPOINT = 'http://localhost:8000/getfiles';
	const ENDPOINT_RENAME = 'http://localhost:8000/rename';
	let current_name = '';
	let ls: Array<object> = [];
	let current_file = '';
	let path = './';
	function upload() {
		let photo = document.getElementById('inputdata').files[0]; // file from input
		var reader = new FileReader();
		reader.onload = function () {
			var arrayBuffer = this.result,
				array = new Uint8Array(arrayBuffer),
				binaryString = String.fromCharCode.apply(null, array);

			console.log(binaryString);
		};
		reader.readAsArrayBuffer(photo);
	}
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
		path = path + e;
		current_name = e;
		getfile(path);
	}
	function goback() {
		if (path == './') {
			dialogs.alert("Can't go back through home");
		}
		let tempath = path.split('/');
		tempath.pop();
		path = tempath.join('/');
		getfile(path);
	}
	async function rename(e) {
		let old = current_file.replace('http://localhost:8000/', './');
		console.log(old);
		let new_name: any = await dialogs.prompt('Insert the new name for this file');
		if (new_name == undefined) dialogs.alert('Please enter a correct name with extension');
		else {
			const xhr = new XMLHttpRequest();
			xhr.open('POST', ENDPOINT_RENAME, true);
			xhr.onreadystatechange = function () {
				if (this.readyState === XMLHttpRequest.DONE && this.status === 200) {
					getfile(path);
				}
			};
			xhr.send(JSON.stringify({ folder: old, new: new_name[0] }));
		}
	}
	onMount(async () => {
		document.onclick = hideMenu;
		getfile(path);
	});
	function contex(e) {
		current_file = 'http://localhost:8000' + (path.replace('.', '') + rightClick(e));
		console.log(current_file);
	}
</script>

<svelte:head>
	<title>Home</title>
	<meta name="description" content="Svelte demo app" />
	<script src="//naver.github.io/egjs-grid/release/latest/dist/grid.min.js"></script>
	<link rel="preconnect" href="https://fonts.gstatic.com" />
	<link
		href="https://fonts.googleapis.com/css2?family=Montserrat:wght@300;400;500;700&display=swap"
		rel="stylesheet"
	/>
	<link
		rel="stylesheet"
		href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/4.7.0/css/font-awesome.min.css"
	/>
	<link rel="stylesheet" href="/css/index.css" />
</svelte:head>

<Toolbox {getfile} {goback} {path} />

<Contex {current_file} {path} {remove} {getfile} {rename} {ls} {current_name} />
<section>
	<div class="grid-container" on:contextmenu={rightClick}>
		{#each ls as lsraw}
			{#if lsraw.md5 == 'dir'}
				<div class="grid-item" on:click={() => test(lsraw.file + '/')}>
					<img src="/images/folder.png" class="icon" alt="folder" />
					<p>{lsraw.file}</p>
				</div>
			{:else}
				<div
					class="grid-item"
					on:click={() => dialogs.modal(Reader, { filename: path + lsraw.file })}
					on:contextmenu={contex}
					id={lsraw.file}
				>
					{#if lsraw.file.split('.')[1] != 'js'}
						<img src="/images/file.png" class="icon" alt="file" />
					{:else}
						<img src="/images/js.png" alt="filejs" class="icon" />
					{/if}
					<p>{lsraw.file}</p>
				</div>
			{/if}
		{/each}
	</div>
	<!-- <input type="file" name="dummyname" id="inputdata" />
	<button on:click={upload}>Upload</button> -->
</section>

<style>
</style>
