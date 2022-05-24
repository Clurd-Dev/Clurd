<script lang="ts">
	import 'uikit/dist/css/uikit.css';
	import 'uikit/dist/js/uikit.js';
	import 'uikit/dist/js/uikit-icons.js';
	export let disks: Array<string> = [];
</script>

<div align="center">
	<table class="uk-table uk-table-divider center" align="center">
		<tbody>
			{#each disks as disk}
				<tr class="textcenter">
					<td>{disk}</td>
				</tr>
				<h1>&nbsp;</h1>
			{/each}
		</tbody>
	</table>
</div>

<style>
	.center {
		margin-left: auto;
		margin-right: auto;
	}
	.textcenter {
		text-align: center;
	}
</style>
