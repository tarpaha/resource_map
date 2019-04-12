import xml.etree.ElementTree as et
import time

def bench_file_parsing():
	time1 = time.time()
	xml = et.parse("resource_map.xml")
	bundlesXmlRoot = xml.find("Bundles")
	count = 0
	for bundleXmlData in bundlesXmlRoot.iter("Bundle"):
		count += 1
	time2 = time.time()
	print("Bundles count: %d, seconds: %0.3f" % (count, time2-time1))

if __name__== "__main__" :
    bench_file_parsing();
