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
	let only_file: string;
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
		console.log(path);
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
		only_file = rightClick(e);
		current_file = 'http://localhost:8000' + (path.replace('.', '') + rightClick(e));
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

<Contex {current_file} {path} {remove} {getfile} {rename} {ls} {current_name} {only_file} />
<section>
	<div class="grid-container" on:contextmenu={contex} align="center">
		{#each ls as lsraw}
			{#if lsraw.md5 == 'dir'}
			
				<div class="grid-item" on:click={() => test(lsraw.file + '/')} id={lsraw.file} align="center">
					<div id="over">
						<img src="/images/folder.png" class="icon" alt="folder" />
					</div>
					<p>{lsraw.file}</p>
				</div>
			{:else}
				<div
					class="grid-item"
					on:click={() => dialogs.modal(Reader, { filename: path + lsraw.file, image:lsraw.image, video:lsraw.video, url: current_file, audio:lsraw.audio, pure_filename: lsraw.file })}
					on:contextmenu={contex}
					id={lsraw.file}
					align="center"
				>
				<div id="over">
					{#if lsraw.image}
						<img src={"http://localhost:8000/" + path + lsraw.file} alt="fileimg" class="icon" />
					{:else if lsraw.video}
						<img src="/images/video.png" alt="filevideo" class="icon" />
					{:else if lsraw.audio}
						<img src="/images/audio.png" alt="fileaudio" class="icon"/>
					{:else if lsraw.file.split('.')[1] == 'pdf'}
						<img src="/images/pdf.png" alt="filepdf" class="icon"/>
					{:else}
						<img src="/images/file.png" alt="file" class="icon"/>
					{/if}
					<p>{lsraw.file}</p>
					</div>
				</div>
			{/if}
		{/each}
	</div>
	<!-- <input type="file" name="dummyname" id="inputdata" />
	<button on:click={upload}>Upload</button> -->
	<hr />
</section>

<style>
	#over img {
  margin-left: auto;
  margin-right: auto;
  display: block;
}
</style>
