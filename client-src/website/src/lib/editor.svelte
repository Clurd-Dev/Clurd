<script>
	import { DialogContent, dialogs } from 'svelte-dialogs';
	import { onMount } from 'svelte';
	import Editor from 'cl-editor';
	export let filename = '';
	const ENDPOINT = 'http://localhost:8000/remove';
	async function get() {
		let text;
		await fetch('http://localhost:8000/' + filename).then(
			async (data) => (text = await data.text())
		);
		return text;
	}

	onMount(async () => {
		const editor = new Editor({
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
</script>

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
		<div id="editor" />
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
