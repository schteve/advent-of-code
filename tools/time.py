# Calculate the total time to run all of the days and parts and print out info on the worst offenders.
# Default is to show the top 5 parts but the user can provide the top N to show on the command line (up to 49)

import re
import sys

unit_factors = {
    's': 1,
    'ms': 1000,
    'us': 1000000,
    'ns': 1000000000,
}

def time_db(n):
    db = {}
    with open('README.md') as fin:
        regex = re.compile('''(\d+)\s+\|\s+(\d+)\s+\|\s+(\d+\.?\d+)\s(\w+)''')
        for line in fin:
            result = regex.match(line)
            if result:
                day = int(result.group(1))
                part = int(result.group(2))
                time = float(result.group(3))
                unit = result.group(4)

                time_adjusted = time / unit_factors[unit]
                db[(day, part)] = time_adjusted

    # Turn the db into a list and sort by time (descending)
    db_list = sorted(db.items(), key=lambda x: x[1], reverse=True)

    total = sum(db.values())

    print(f'Total time: {total:.5f} s ({len(db)} parts)')
    print(f'Showing biggest {n} parts...')
    print('Day\tPart\tTime\t\tTotal without\t% total\t\t% total so far')
    running_pct = 0.0
    for i in range(n):
        (day, part) = db_list[i][0]
        time = db_list[i][1]
        without = total - time
        pct = (time / total) * 100
        running_pct += pct
        print(f'{day}\t{part}\t{time:.5f} s\t{without:.5f} s\t{pct:4.1f} %\t\t{running_pct:4.1f} %')

if __name__ == "__main__":
    if len(sys.argv) == 1:
        n = 5
    else:
        n = int(sys.argv[1])
    time_db(n)
