import os, sys
import struct

types = {
    "geo": "string",
    "type": "string",
    "texture": "string",
    "size": "vec3", 
    "pos": "vec3"
}

def toFloatByteArray(float_str):
    return struct.pack("f", float(float_str))

def toIntByteArray(int_val):
    return int_val.to_bytes(4, 'big')

def toStringByteArray(str_val):
    #return bytearray().extend(map(ord, str_val))
    return str_val.encode('utf-8')

def toSplit(cur_entity, cur_line):
    cur_line = cur_line.strip()
    name = cur_line.split("=")[0]
    val = cur_line.split("=")[1]
    cur_entity[name] = val

def toByteArray(val, cur_type):
    byte_array_list = []
    # Type Enum (4 bytes)
    # Size (4 bytes)
    # Name Enum (4 bytes)

    if cur_type == "string":
        byte_array_list.append(toIntByteArray(0))
        str_byte_array = toStringByteArray(val)
        print(len(str_byte_array))
        byte_array_list.append(toIntByteArray(len(str_byte_array)))
        byte_array_list.append(str_byte_array)
    elif cur_type == "vec3":
        byte_array_list.append(toIntByteArray(1))
        byte_array_list.append(toIntByteArray(12))
        all_vals = val.split(",")
        for cur_val in all_vals:
            byte_array_list.append( toFloatByteArray(cur_val))
    return byte_array_list
        

def buildFile(cur_entity):

    # Number Properties (4 byes)
    # Type Enum (4 bytes)
    # Size (4 bytes)
    # Name Enum (4 bytes)
    # Data (n bytes)
    all_bytes = toIntByteArray(len(cur_entity))
    for cur_name in cur_entity:
        val = cur_entity[cur_name]
        cur_type = types[cur_name]
        byte_array_list = toByteArray(val, cur_type)
        for cur_byte_array in byte_array_list:
            #print(byte_array_list)
            all_bytes += cur_byte_array
    all_bytes += toIntByteArray(1)
    return all_bytes

def buildData(fname):
    data = []
    dumps = open(fname + ".prs", "r").readlines()
    cur_entity = {}
    array_data_list = []
    array_data = toStringByteArray("START")
    for line in dumps:
        line = line.strip()
        if line == "START":
            cur_entity = {}
        elif line == "END":
            data.append(cur_entity)
            array_data_list.append(buildFile(cur_entity))
        elif '=' in line:
            toSplit(cur_entity, line)
    for cur_array_data in array_data_list:
        array_data += cur_array_data
    open(fname + ".bin", 'wb').write(array_data)

            

fname = sys.argv[1]
buildData(fname)