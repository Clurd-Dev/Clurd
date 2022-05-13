import { dialogs } from "svelte-dialogs";
async function checkresponse(response:string, refresh, absolute){
	if(response == "1"){
		let confirm;
		do {
		  confirm = await dialogs.alert("File deleted successfully");
		} while (confirm );
			refresh(absolute);
	  }
	else{
		let confirm;
		do {
		  confirm = await dialogs.alert("File deleted unsuccessfully, check server log to see error and if you think this is a bug open an issue on github.");
		} while (confirm );
	  }
}
export async function remove(path:string, refresh, absolute:string){
    const ENDPOINT = 'http://localhost:8000/remove';
    const xhr = new XMLHttpRequest();
		xhr.open('POST', ENDPOINT, true);
		xhr.onreadystatechange = async function () {
			if (this.readyState === XMLHttpRequest.DONE && this.status === 200) {

			
				checkresponse(this.responseText, refresh, absolute);
				
			}
		}
			xhr.send(JSON.stringify({ folder: path }));
}
			
	
