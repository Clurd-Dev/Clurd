<script lang="ts">
	import 'uikit/dist/css/uikit.css';
	import 'uikit/dist/js/uikit.js';
	import 'uikit/dist/js/uikit-icons.js';
	export let interfaces: Array<string> = [];
</script>

<table class="uk-table uk-table-divider center">
	<tbody>
		{#each interfaces as card}
			<tr>
				<td>{card}</td>
			</tr>
			<h1>&nbsp;</h1>
		{/each}
	</tbody>
</table>

<style>
	.center {
		margin-left: auto;
		margin-right: auto;
	}
	.textcenter {
		text-align: center;
	}
</style>
