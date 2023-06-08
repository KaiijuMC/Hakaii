import io
import os
import sys
import random
import threading

from hakaii.linear import *
from nbt import nbt


def load_file(filename):
    #print(f"Opening file {filename}...")
    if ".mca" in filename:
        region = open_region_anvil(filename)
        write = write_region_anvil
    else:
        region = open_region_linear(filename)
        write = write_region_linear
    return region, write

def divide_array(tab, n):
    if n <= 0:
        raise ValueError("Number of smaller arrays (n) should be greater than 0.")
    elif len(tab) < n:
        raise ValueError("Number of elements in the array should be greater than or equal to n.")

    random.shuffle(tab)

    quotient = len(tab) // n  # Number of elements in each smaller array
    remainder = len(tab) % n  # Remainder elements

    divided_arrays = []
    start_index = 0

    for _ in range(n):
        if remainder > 0:
            end_index = start_index + quotient + 1
            remainder -= 1
        else:
            end_index = start_index + quotient

        divided_arrays.append(tab[start_index:end_index])
        start_index = end_index

    return divided_arrays


def clean_regions(dirname, duration, files):
    region_dir = os.path.join(dirname, "region")
    entities_dir = os.path.join(dirname, "entities")
    poi_dir = os.path.join(dirname, "poi")
    for name in files:
        count = 0
        nulled = 0
        write = None
        todelete = []

        # REGION PART
        filename = os.path.join(region_dir, name)
        rregion, write = load_file(filename)
        if rregion is not None:
            i = 0
            for chunk in rregion.chunks:
                if chunk is None:
                    nulled += 1
                    i += 1
                    continue
                buffer = io.BytesIO(chunk.raw_chunk)
                data = nbt.NBTFile(buffer=buffer)
                time = data["InhabitedTime"].value
                if time < duration:
                    count += 1
                    todelete.append(i)
                    rregion.chunks[i] = None
                    rregion.timestamps[i] = 0
                    #print(f"[{chunk.x}, {chunk.z}] deletion is scheduled.")
                i += 1
            write(filename, rregion)

        # ENTITIES PART
        filename = os.path.join(entities_dir, name)
        eregion, write = load_file(filename)
        if eregion is not None:
            for i in todelete:
                eregion.chunks[i] = None
                eregion.timestamps[i] = 0
            write(filename, eregion)

        # POI PART
        filename = os.path.join(poi_dir, name)
        pregion, write = load_file(filename)
        if pregion is not None:
            for i in todelete:
                pregion.chunks[i] = None
                pregion.timestamps[i] = 0
            write(filename, pregion)

        print(f"[REGION {rregion.region_x:5} {rregion.region_z:5}] Deleted {count:4} chunks, {nulled:4} ungenerated chunks, keeping {1024 - count - nulled:4} chunks")

