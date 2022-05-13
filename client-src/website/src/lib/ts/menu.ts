export function rightClick(e) {
	e.preventDefault();
	document.getElementById('contextMenu').style.display = 'block';
	var menu = document.getElementById('contextMenu');
	menu.style.display = 'block';
	menu.style.left = e.pageX + 'px';
	menu.style.top = e.pageY + 'px';
    console.log(e);
    console.log(e.path[1].id);
    return e.path[1].id;
}
export function hideMenu() {
	document.getElementById('contextMenu').style.display = 'none';
}
