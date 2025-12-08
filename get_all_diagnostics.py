import os
import subprocess
import re
import sys

BIN_PATTERN = "{day}_{part}"


def get_day_folders():
    dirs = [d for d in os.listdir('.') if os.path.isdir(d) and d.isdigit()]
    return sorted(dirs, key=int)


def parse_output(output):
    time_matches = re.findall(r"Time:\s+(.+)", output)
    mem_matches = re.findall(r"Mem:\s+(.+)", output)

    execution_time = time_matches[-1].strip() if time_matches else "N/A"
    memory = mem_matches[-1].strip() if mem_matches else "N/A"

    return execution_time, memory


def run_part(day, part):
    bin_name = BIN_PATTERN.format(day=int(day), part=part)

    cmd = ["cargo", "run", "--bin", bin_name, "--release"]

    try:
        result = subprocess.run(
            cmd,
            capture_output=True,
            text=True,
            check=True,
            encoding='utf-8',
        )
        return parse_output(result.stdout)
    except subprocess.CalledProcessError as e:
        return "ERROR", "CRASH"
    except Exception as e:
        return "FAIL", str(e)


def main():
    days = get_day_folders()

    if not days:
        print("No numbered folders found. Are you running this in the right directory?")
        print("Or perhaps you've given up early this year. I wouldn't blame you.")
        return

    results = []

    print(f"Found {len(days)} days. Preparing to heat up your CPU...")
    print("-" * 60)

    # The Loop of Pain
    for day in days:
        for part in [1, 2]:
            print(f"Running Day {day} Part {part}...", end="\r")
            sys.stdout.flush()

            time_val, mem_val = run_part(day, part)

            # Save raw data for the table
            results.append({
                "Day": day,
                "Part": part,
                "Time": time_val,
                "Mem": mem_val
            })

    # Clear the progress line
    print(" " * 60, end="\r")

    # ---------------------------------------------------------
    # The Table of Judgment
    # ---------------------------------------------------------
    table_data = [["Day", "Part", "Time", "Memory"]]
    
    for row in results:
        table_data.append([
            str(row["Day"]), 
            str(row["Part"]), 
            str(row["Time"]), 
            str(row["Mem"])
        ])

    col_widths = [max(len(cell) for cell in col) for col in zip(*table_data)]

    row_fmt = " | ".join([f"{{:<{w}}}" for w in col_widths])

    separator = "-" * (sum(col_widths) + 3 * (len(col_widths) - 1))
    
    print("\n🎄 PERFORMANCE SUMMARY 🎄")
    print(separator)
    
    header = table_data[0]
    print(row_fmt.format(*header))
    print(separator)
    
    for row in table_data[1:]:
        print(row_fmt.format(*row))
    
    print(separator)


if __name__ == "__main__":
    main()
