var async = require("async"),
	execAsync = require('child_process').exec,
	execSync = require('child_process').execSync;

function processSync(callback) {
	execSync("node process.js");
	callback();
}

function processAsync(callback) {
	var child = execAsync("node process.js");
	child.on('exit', callback);
}

function processLoop(callback) {
	var x = 0;
	for (var i = 0; i < 15000000; i++) {
		x += 1;
	}
	callback();
}

var threads = [];
var process = processSync; // or ProcessAsync or ProcessLoop; In our use case ProcessSync is probably the closest to other implementations
for (var i = 0; i < 10; i++) {
	threads.push(function (callback) {
		process(callback);
	});
}
async.parallel(threads, function(err, results){
    console.log('done!');
});
