import os
import numpy

path_bench = "reduction"
num_threads = [1, 2, 4, 6, 8, 12, 16]
sizes = ["500000000", "1000000000", "2000000000"]
langs = ['c++', 'rust']
power = ["0", "1"]


def pretty_print(l):
    for val in l:
        print(val, end=', ')
    print("")


for lang in langs:
    print(lang)
    for p in power:
        for size in sizes:
            print(size)

            for t in num_threads:
                f = open("{}/{}/thread_{}_{}_{}.log".format(path_bench,
                                                            lang, t, size, p), 'r')
                lines = f.readlines()
                # print("Threads:", t)
                for line in lines:
                    if "par" in line:
                        parts = line.split()
                        print(parts[-2][1:-1], end=',')

                f.close()
            print("\n")
    print("")
