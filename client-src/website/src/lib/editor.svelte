<script lang="ts">
	import { DialogContent, dialogs } from 'svelte-dialogs';
	import { onMount, onDestroy } from 'svelte';
	import { ComponentWindow } from '../lib/ts/pdf';
	import STDEditor from 'cl-editor';
	import PdfViewer from 'svelte-pdf';
	import { AudioPlayer } from 'svelte-mp3';
	import 'bytemd/dist/index.css';
	import { Editor, Viewer } from 'bytemd';
	import gfm from '@bytemd/plugin-gfm';

	let value;
	const plugins = [gfm()];

	function handleChange(e) {
		value = e.detail.value;
	}
	let componentWindow = new ComponentWindow();
	let component;
	export let filename = '',
		image: boolean,
		video: boolean,
		url: string,
		audio: boolean,
		pure_filename: string;
	const ENDPOINT = 'http://localhost:8000/remove';
	async function get() {
		let text;
		await fetch('http://localhost:8000/' + filename).then(
			async (data) => (text = await data.text())
		);
		return text;
	}

	onMount(async () => {
		const editor = new STDEditor({
			// <HTMLElement> required
			target: document.getElementById('editor'),
			// optional
			props: {
				actions: [
					'b',
					'i',
					'u',
					'strike',
					'ul',
					'ol',
					{
						name: 'copy', // required
						icon: '<b>C</b>', // string or html string (ex. <svg>...</svg>)
						title: 'Copy',
						result: () => {
							// copy current selection or whole editor content
							const selection = window.getSelection();
							if (!selection.toString().length) {
								const range = document.createRange();
								range.selectNodeContents(editor.refs.editor);
								selection.removeAllRanges();
								selection.addRange(range);
							}
							editor.exec('copy');
						}
					},
					'h1',
					'h2',
					'p'
				],
				// default 300px
				height: '300px',
				// initial html
				html: await get(),
				// remove format action clears formatting, but also removes some html tags.
				// you can specify which tags you want to be removed.
				removeFormatTags: ['h1', 'h2', 'blackquote'] // default
			}
		});
	});
	function bin() {
		var xhr = new XMLHttpRequest();
		xhr.open('POST', ENDPOINT, true);
		xhr.onreadystatechange = function () {
			if (this.readyState === XMLHttpRequest.DONE && this.status === 200) {
				if (this.response == 1) {
					dialogs.alert('File correctly removed');
					location.reload();
				} else dialogs.alert('Problem during the removing of file');
			}
		};
		xhr.send(JSON.stringify({ folder: filename }));
	}
	onDestroy(() => componentWindow.destroy());

	$: if (component) component.$set({ url });

	async function openComponentWindow() {
		if (componentWindow.isOpened) {
			componentWindow.focus();
			return;
		}

		component = await componentWindow.attachComponent(PdfViewer, {
			props: {
				url: 'http://localhost:8000/' + filename
			}
		});

		componentWindow.focus();
	}

	async function openmd() {
		if (componentWindow.isOpened) {
			componentWindow.focus();
			return;
		}

		component = await componentWindow.attachComponent(Editor, {
			props: {
				value: await get()
			}
		});

		componentWindow.focus();
	}
</script>

<svelte:head>
	<link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/@vime/core@^5/themes/default.css" />
</svelte:head>
<DialogContent>
	<div slot="header" class="grid-container" align="center">
		<div class="grid-item">
			<div align="center">
				<a href={'http://localhost:8000/' + filename}>
					<img src="/images/download.png" alt="icondownload" />
				</a>
			</div>
		</div>
		<div class="grid-item">
			<div align="center">
				<img src="/images/bin.png" alt="bin" on:click={bin} />
			</div>
		</div>
	</div>

	<svelte:fragment slot="body">
		{#if image}
			<img src={'http://localhost:8000/' + filename} alt={filename} />
		{:else if video}
			<video src={'http://localhost:8000/' + filename} controls />
		{:else if audio}
			<AudioPlayer urls={['http://localhost:8000/' + filename]} />
		{:else if pure_filename.split('.')[1] == 'pdf'}
			<button on:click={openComponentWindow}>View PDF file</button>
		{:else if pure_filename.split('.')[1] == 'md'}
			<button on:click={openmd} on:change={handleChange}>Open MarkDown editor</button>
		{:else}
			<div id="editor" />
		{/if}
	</svelte:fragment>
</DialogContent>

<style>
	.grid-container {
		margin: auto;
		width: 50%;
		display: grid;
		grid-template-columns: auto auto auto;
	}
	.grid-item {
		border: 1px solid rgba(0, 0, 0, 0.8);
		padding: 20px;
		font-size: 30px;
		text-align: center;
	}
</style>
