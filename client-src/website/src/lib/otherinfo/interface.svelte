<script lang="ts">
    export let interfaces: Array<string> = [] ;
</script>
<svelte:head>
    <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/uikit@3.14.1/dist/css/uikit.min.css" />
	<script src="https://cdn.jsdelivr.net/npm/uikit@3.14.1/dist/js/uikit.min.js"></script>
	<script src="https://cdn.jsdelivr.net/npm/uikit@3.14.1/dist/js/uikit-icons.min.js"></script>
</svelte:head>
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
.textcenter{
    text-align: center;
}
</style>