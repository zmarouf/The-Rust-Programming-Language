import threading

def process():
	x = 0
	for i in range(15000000):
 		x += 1

threads = []

for n in range(10):
    thread = threading.Thread(target=process)
    thread.start()

    threads.append(thread)

for thread in threads:
    thread.join()

print "done."