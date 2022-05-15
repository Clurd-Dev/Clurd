<script lang="ts">
    import { get_config, get_information } from '$lib/ts/io';
    import { onMount, onDestroy } from 'svelte';
    import Disk from '$lib/otherinfo/disk.svelte';
    import Interfaces from '$lib/otherinfo/interface.svelte';
    let location_website: string, path: string, information:any, core: string, os: string, memory: string, memoryused: string, swap: string, swapused: string, hostname: string, fronted: string, backend: string, disks: any;
    onMount(async()=>{{
        location_website = 'http://' + location.hostname + ':8000/'
        path = await get_config(location_website),
        information = await get_information(location_website);
        console.log(information)
        core = information.core;
        os = information.system_name.replace('Some("', '').replace('")', '') + " " + information.system_version.replace('Some("', '').replace('")', '') + " " + information.kernel_version.replace('Some("', '').replace('")', '') + " ";
        memory = information.total_memory;
        memoryused = information.used_memory;
        swap = information.total_swap;
        swapused = information.used_swap;
        hostname = (information.hostname).replace('Some("', '');
        hostname = hostname.replace('")', '');
        fronted = information.frontend_version;
        backend = information.backend_version;
        disks = information.disks
    }});
    let componentWindow = new ComponentWindow();
	let component;
    import {ComponentWindow} from '../lib/ts/pdf';
import Interface from '$lib/otherinfo/interface.svelte';
    	onDestroy(() => componentWindow.destroy())
	
	$: if (component) component.$set({ disks })
	
	async function opendisk() {
		if (componentWindow.isOpened) {
			componentWindow.focus()
			return
		}

		component = await componentWindow.attachComponent(Disk, {
			props: {
				disk: information.disks			
			}
		})
		
		componentWindow.focus()
	}

    async function openinterface() {
		if (componentWindow.isOpened) {
			componentWindow.focus()
			return
		}

		component = await componentWindow.attachComponent(Interface, {
			props: {
				interfaces: information.interface		
			}
		})
		
		componentWindow.focus()
	}
</script>
<hr/>
<div align="center">
    <fieldset class="uk-fieldset container">

        <legend class="uk-legend">Settings for Clurd backend(read-only)</legend>
        <br/>
        <p>This are the current settings for Clurd, if you want to change, modify the file config.toml in Clurd installation dir</p>
        <br/>
        <div class="uk-margin">
            <p>Current path:</p>
            <br/>
            <input class="uk-input" type="text" placeholder="Current path:" bind:value={path} readonly>
            <h1>&nbsp;</h1>
            <h1>System information:</h1>
            <h1>&nbsp;</h1>
            <table class="uk-table uk-table-divider">
              
                <tbody>
                    <tr>
                        <td>CPU core</td>
                        <td>{core}</td>
                    </tr>
                    <tr>
                        <td>OS info</td>
                        <td>{os}</td>
                    </tr>
                    <tr>
                        <td>Total memory</td>
                        <td>{(parseInt(memory)/1000000).toFixed(2)} Gb</td>
                    </tr>
                    <tr>
                        <td>Used memory</td>
                        <td>{(parseInt(memoryused)/1000000).toFixed(2)} Gb</td>
                    </tr>
                    <tr>
                        <td>Total swap</td>
                        <td>{(parseInt(swap)/1000000).toFixed(2)} Gb</td>
                    </tr>
                    <tr>
                        <td>Used swap</td>
                        <td>{(parseInt(swapused)/1000000).toFixed(2)} Gb</td>
                    </tr>
                    <tr>
                        <td>Hostname</td>
                        <td>{hostname}</td>
                    </tr>
                    <tr>
                        <td>Disk</td>
                        <a href="#0" on:click={opendisk}><td>Show</td></a>
                    </tr>
                    <tr>
                        <td>Interface</td>
                        <a href="#0" on:click={openinterface}><td>Show</td></a>
                    </tr>
                    <tr>
                        <td>Fronted version</td>
                        <td>{fronted}</td>
                    </tr>
                    <tr>
                        <td>Backend version</td>
                        <td>{backend}</td>
                    </tr>
                </tbody>
            </table>
        </div>

    </fieldset>
</div>
<style>
    .container{
        margin:20px;
    }
</style>
