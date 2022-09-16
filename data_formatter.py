raw_file = open("candump_raw.trc")
nice_file = open("formatted_data.trc", "w")

for line in raw_file:
    if line[0] == ";":
        continue
    sline = line.split()

    #print(data)
    ms = sline[0]
    data = sline[2]
    #data = "".join(sline[6:])
    #print(ms, id)
    #formatted = "    ".join([ms, "#".join([id, data])]) + "\n"
    nice_file.write(ms + "     " + data+"\n")