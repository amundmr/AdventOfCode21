import pandas as pd

with open("inputs/3.txt") as f:
    data = f.readlines()


oxy_data = data

## OXY first
for i in range(0, 12):

    bitsum = 0
    for line in oxy_data:
        bitsum += int(line[i])

    if 2*bitsum >= len(oxy_data):
        #1 is dominating, or 1 and 0 is even
        dominating = 1
    else:
        #0 is dominating
        dominating = 0
    
    new_oxy_data = []
    for line in oxy_data:
        if line[i] == str(dominating):
            new_oxy_data.append(line)
    if len(new_oxy_data) == 1:
        print("The oxygen binary left is: {}".format(new_oxy_data))
        oxy_int = int(new_oxy_data[0].strip("\n"), 2)
        print("OXY INT: {}".format(oxy_int))
        break

    oxy_data = new_oxy_data
    

co2_data = data

## CO2 first
for i in range(0, 11):

    bitsum = 0
    for line in co2_data:
        bitsum += int(line[i])

    if 2*bitsum >= len(co2_data):
        #1 is dominating, or 1 and 0 is even
        notdominating = 0
    else:
        #0 is dominating
        notdominating = 1
    
    new_co2_data = []
    for line in co2_data:
        if line[i] == str(notdominating):
            new_co2_data.append(line)

    if len(new_co2_data) == 1:
        print("The CO2 binary left is: {}".format(new_co2_data))
        co2_int = int(new_co2_data[0].strip("\n"), 2)
        print("CO2 INT: {}".format(co2_int))
        break

    co2_data = new_co2_data

print("FINAL NUMBER = {}, magnus num is: 4996233".format(oxy_int*co2_int))