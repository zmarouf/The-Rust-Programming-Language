function process() {
	var x = 0;
	for (var i = 0; i < 15000000; i++) {
		x += 1;
	}
	console.log(x);
}
process();