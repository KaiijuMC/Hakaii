import os
import sys
import threading

from hakaii.hakaii import *

if __name__ == "__main__":
    try:
        dirname = sys.argv[1]
        duration = int(sys.argv[2])
        threads = int(sys.argv[3])
    except:
        print("Usage:")
        print("python -m hakaii <world dir> <min inhabited time> <threads>")
        exit()

    print(f"We will delete chunks inhabited less than {duration/20} secs / {duration} ticks.")

    regions = os.listdir(os.path.join(dirname, "region"))
    subtasks = divide_array(regions, threads)
    threads = []
    for task in subtasks:
        thread = threading.Thread(target=clean_regions, args=(dirname, duration, task))
        thread.start()
        threads.append(thread)

    for thread in threads:
        thread.join()

