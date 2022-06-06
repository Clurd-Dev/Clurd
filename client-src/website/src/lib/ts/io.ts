import { dialogs } from 'svelte-dialogs';
async function checkresponse(response: string, refresh, absolute) {
	if (response == '1') {
		let confirm;
		do {
			confirm = await dialogs.alert('File deleted successfully');
		} while (confirm);
		refresh(absolute);
	} else {
		let confirm;
		do {
			confirm = await dialogs.alert(
				'File deleted unsuccessfully, check server log to see error and if you think this is a bug open an issue on github.'
			);
		} while (confirm);
	}
}
export async function remove(path: string, refresh, absolute: string, only_file: string) {
	console.log(absolute + "/" + only_file);
	const ENDPOINT = location.origin + '/remove';
	const xhr = new XMLHttpRequest();
	xhr.open('POST', ENDPOINT, true);
	xhr.onreadystatechange = async function () {
		if (this.readyState === XMLHttpRequest.DONE && this.status === 200) {
			checkresponse(this.responseText, refresh, absolute);
		}
	};
	xhr.send(JSON.stringify({ folder: absolute + "/" + only_file}));
}

export async function copyfs(old_file: string, new_file: string, file: string) {
	console.log(old_file);
	console.log(new_file);
	console.log(file);
	const ENDPOINT = location.origin + '/copy';
	const xhr = new XMLHttpRequest();
	xhr.open('POST', ENDPOINT, true);
	xhr.onreadystatechange = async function () {
		if (this.readyState === XMLHttpRequest.DONE && this.status === 200) {
			if (this.responseText == '1')
				dialogs.alert('File successfully copied').then(() => location.reload());
			else dialogs.alert('File unsuccessfully copied, check the error in log of server or retry');
		}
	};
	xhr.send(JSON.stringify({ folder: old_file, new: new_file + file }));
}

export async function movefs(old_file: string, new_file: string, file: string) {
	const ENDPOINT = location.origin + '/move';
	const xhr = new XMLHttpRequest();
	xhr.open('POST', ENDPOINT, true);
	xhr.onreadystatechange = async function () {
		if (this.readyState === XMLHttpRequest.DONE && this.status === 200) {
			if (this.responseText == '1')
				dialogs.alert('File successfully moved').then(() => location.reload());
			else dialogs.alert('File unsuccessfully moved, check the error in log of server or retry');
		}
	};
	xhr.send(JSON.stringify({ folder: old_file, new: new_file + file }));
}

export async function get_config(endpoint: string) {
	const ENDPOINT = endpoint + 'getconfig';
	const request = await fetch(ENDPOINT);
	if (request.ok) {
		const json = await request.json();
		return json.path;
	} else {
		console.log('HTTP-Error: ' + request.status);
	}
}

export async function get_information(endpoint: string) {
	const ENDPOINT = endpoint + 'getinfo';
	const request = await fetch(ENDPOINT);
	if (request.ok) {
		const json = await request.json();
		return json;
	} else {
		console.log('HTTP-Error: ' + request.status);
	}
}
