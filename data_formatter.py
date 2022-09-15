raw_file = open("raw_data.trc")
nice_file = open("formatted_data.trc", "w")

for line in raw_file:
    if line[0] == ";":
        continue
    sline = line.split()
    #print(data)
    ms = sline[1]
    id = sline[3]
    data = "".join(sline[6:])
    print(ms, id, data)
    formatted = "    ".join([ms, "#".join([id, data])]) + "\n"
    #nice_file.write(formatted)