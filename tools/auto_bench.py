# Benchmark solutions and record the time in the README.md file table.
#
# With no arguments, benchmarks all days and parts. Arguments can be supplied to pick individual days and parts,
# where each argument specifies a day to benchmark (both parts) or a specific part of a day. E.g.
#   > auto_bench.py 1 2 3 20 # Bench days 1, 2, 3, and 20 (both parts for each)
#   > auto_bench.py 1.1, 5.1, 10.2, 15 # Bench day 1 part 1, day 5 part 1, day 10 part 2, and day 15 both parts
#
# Also record the total time, where each part is pulled from the table if it's not part of this benchmark run.
# If any times are completely missing they're not included in the total.
#
# File table should have a format like this:
# Day | Part | Time
# :--:| :--: | :-------:
# 1   | 1    |
# 1   | 2    |
# 2   | 1    |

import re
import subprocess
import sys

unit_factors = {
    's': 1,
    'ms': 1000,
    'us': 1000000,
    'ns': 1000000000,
}

time_regex = re.compile(r'time:\s*\[\d+(?:\.\d+)?\s\w+ (\d+(?:\.\d+))\s(\w+) \d+(?:\.\d+)?\s\w+\]')
table_time_regex = re.compile('''(\d+)\s+\|\s+(\d+)\s+\|\s+(\d+\.?\d+)\s(\w+)''')

def bench(days_parts):
    # Read README.md so we can insert the benchmark results as we go
    with open('README.md', 'r') as readme:
        readme_text = readme.read()

    # Pull existing time values, if any. These will be useful to get the total if all parts aren't freshly benched.
    time_db = {}
    for line in readme_text.splitlines():
        result = table_time_regex.match(line)
        if result:
            day = int(result.group(1))
            part = int(result.group(2))
            time = float(result.group(3))
            unit = result.group(4)

            time_adjusted = time / unit_factors[unit]
            time_db[(day, part)] = time_adjusted

    for (day, part) in days_parts:
        # Run the bench command
        result = subprocess.run(['cargo', 'aoc', 'bench', '-d', f'{day}', '-p',f'{part}'], stdout=subprocess.PIPE, stderr=subprocess.DEVNULL)
        result_output = result.stdout.decode('utf-8')

        # Extract the median time
        (val, unit) = time_regex.search(result_output).groups()
        print(f'Day {day} part {part}: {val} {unit}')

        # Calculate the adjusted time (units of seconds)
        time_adjusted = float(val) / unit_factors[unit]
        time_db[(day, part)] = time_adjusted

        # Insert the time into the readme
        readme_text = re.sub(rf'^({day}\s+\| {part}\s+\|)(.*?)$', rf'\g<1> {val} {unit}', readme_text, flags=re.MULTILINE)

    # Calculate the total time
    total = sum(time_db.values())

    # Insert the total time into the readme
    print(f'Total time: {total:.5} s')
    readme_text = re.sub(r'^Total:.*?$', rf'Total: {total:.5} s', readme_text, flags=re.MULTILINE)

    with open('README.md', 'w') as readme:
        readme.write(readme_text)

if __name__ == "__main__":
    if len(sys.argv) == 1:
        days_parts = [(day, part) for day in range(1, 26) for part in range(1, 3) if (day, part) != (25, 2)]
    else:
        days_parts = []
        for arg in sys.argv[1:]:
            split = arg.split('.')
            if len(split) == 1:
                # The argument is a day, so queue both parts
                day = int(split[0])
                days_parts.append((day, 1))
                if day != 25:
                    days_parts.append((day, 2))
            else:
                # The argument is a specific day + part
                day = int(split[0])
                part = int(split[1])
                days_parts.append((day, part))

    bench(days_parts)
