<script lang="ts">
	import { dialogs } from 'svelte-dialogs';
	import Reader from '../lib/editor.svelte';
	import { remove, get_config } from '../lib/ts/io';
	import { onMount } from 'svelte';
	import { rightClick, hideMenu } from '../lib/ts/menu';
	import { Circle2 } from 'svelte-loading-spinners'
	import Toolbox from '$lib/toolbox.svelte';
	import Contex from '$lib/contex/contex.svelte';
	import 'uikit/dist/css/uikit.css';
	import 'uikit/dist/js/uikit.js';
	import 'uikit/dist/js/uikit-icons.js';
	let location_website: string;
	let current_name = '';
	let ls: Array<object> = [];
	let current_file = '';
	let path: string;
	let only_file: string;
	let loading: boolean;
	let virt_path: string = "";
	function getfile(path: string) {
		loading = true;
		const xhr = new XMLHttpRequest();
		let url = location_website + '/' + 'getfiles';
		xhr.open('POST', url, true);
		xhr.onreadystatechange = function () {
			if (this.readyState === XMLHttpRequest.DONE && this.status === 200) {
				ls = JSON.parse(this.response);
				loading = false;
			}
		};
		xhr.send(JSON.stringify({ folder: path }));
	}

	async function test(e: string) {
		virt_path = virt_path + e;
		path = path + e;
		current_name = e;
		getfile(path);
		console.log(virt_path)
	}

	function goback() {
		if (path == './') {
			dialogs.alert("Can't go back through home");
		} else {
			let tempath = path.split('/');
			tempath.pop();
			path = tempath.join('/');
			let temp_virt_path = virt_path.split('/');
			temp_virt_path.pop();
			virt_path = temp_virt_path.join('/');
			if (path == '.') 
				path += '/';
			if (virt_path == "")
				virt_path = "/";
			getfile(path);
			console.log("VIRT:" + virt_path);
		}
	}

	async function rename(e) {
		let old = path + "/" + only_file;
		let new_name: any = await dialogs.prompt('Insert the new name for this file');
		if (new_name == undefined) {
			dialogs.alert('Please enter a correct name with extension');
		}else {
			const xhr = new XMLHttpRequest();
			xhr.open('POST', location_website + '/rename', true);
			xhr.onreadystatechange = function () {
				if (this.readyState === XMLHttpRequest.DONE && this.status === 200) {
					getfile(path);
				}else{
					dialogs.alert("Error during rename");
					getfile(path);
				}
			};
			// console.log(old);
			// console.log(new_name[0]);
			// console.log(path);
			xhr.send(JSON.stringify({ folder: old, new: path + "/" + new_name[0] }));
		}
	}

	onMount(async () => {
		loading = true;
		location_website = location.origin;
		path = await get_config(location_website + "/");
		document.onclick = hideMenu;
		getfile(path);
	});

	function contex(e) {
		only_file = rightClick(e);
		current_file = location_website + (virt_path + "/" + rightClick(e));
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

<Contex {current_file} {path} {remove} {getfile} {rename} {ls} {current_name} {only_file} old_path={path}/>
<section>
	{#if loading == true}
		<div align="center">
			<Circle2 size="256" unit="px" duration="1s"></Circle2>
		</div>
	{:else}
	<div class="grid-container" on:contextmenu={contex} align="center">
		{#each ls as lsraw}
			{#if lsraw.dir == true}
				<div
					class="grid-item"
					on:click={() => test("/" + lsraw.file)}
					id={lsraw.file}
					align="center"
				>
					<div id={lsraw.file}>
						<img src="/images/folder.png" class="icon" alt="folder" />
					</div>
					<p>{lsraw.file}</p>
				</div>
			{:else}
				<div
					class="grid-item"
					on:click={() =>
						dialogs.modal(Reader, {
							filename: virt_path + lsraw.file,
							image: lsraw.image,
							video: lsraw.video,
							url: current_file,
							audio: lsraw.audio,
							pure_filename: lsraw.file
						})}
					on:contextmenu={contex}
					id={lsraw.file}
					align="center"
				>
					<div id={lsraw.file}>
						{#if lsraw.image}
							<img src="/images/image.png" alt="fileimg" class="icon" />
						{:else if lsraw.video}
							<img src="/images/video.png" alt="filevideo" class="icon" />
						{:else if lsraw.audio}
							<img src="/images/audio.png" alt="fileaudio" class="icon" />
						{:else if lsraw.file.split('.')[1] == 'pdf'}
							<img src="/images/pdf.png" alt="filepdf" class="icon" />
						{:else}
							<img src="/images/file.png" alt="file" class="icon" />
						{/if}
						<p>{lsraw.file}</p>
					</div>
				</div>
			{/if}
		{/each}
	</div>
	<hr />
	{/if}
</section>

<style>
	#over img {
		margin-left: auto;
		margin-right: auto;
		display: block;
	}
</style>
