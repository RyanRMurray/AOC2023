#! /usr/bin/python3

import argparse
import glob
import os
import sys
from pathlib import Path


parser = argparse.ArgumentParser(
    prog="start_day",
    description="Create the solution file for a given day"
)
parser.add_argument("-d", "--day", type=int, required=True, choices=range(1,26))


if __name__ == "__main__":
    args = parser.parse_args()
    day = "{:02d}".format(args.day)

    if os.path.isfile(f"./src/solutions/day{day}.rs"):
        sys.exit("File already exists")
    
    # create file
    with open("./templates/solution_template.template", "r") as f:
        sol_template = f.read()
    
    sol_file = sol_template.replace("{{ day_value }}", day)

    with open(f"./src/solutions/day{day}.rs", "w") as f:
        f.write(sol_file)
    
    print("Created file.")

    # create mod.rs
    with open("./templates/mod_template.template", "r") as f:
        mod_template = f.read()
    
    files = [Path(f).stem for f in glob.glob("./src/solutions/day*.rs")]
    mods = "\n".join([f"mod {f};" for f in files])
    sols = "\n".join([f"\t{f}::{f}," for f in files])
    
    mod_template = mod_template.replace("{{ mods }}", mods)
    mod_template = mod_template.replace("{{ solutions }}", sols)
    mod_template = mod_template.replace("{{ sol_count }}", str(len(files)))

    with open(f"./src/solutions/mod.rs", "w") as f:
        f.write(mod_template)
    
    print("Updated mod.rs.")
