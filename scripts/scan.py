"""Scan a folder recursively to get size info of every .zip file.

Intended to diagnostic if we should or not remove small dictionaries from a release.
"""

import argparse
from pathlib import Path


def format_size(size_bytes: float) -> str:
    for unit in ["B", "KB", "MB", "GB", "TB"]:
        if size_bytes < 1024:
            return f"{size_bytes:.2f} {unit}"
        size_bytes /= 1024
    return f"{size_bytes:.2f} PB"


def scan_zip_sizes(folder: Path):
    intervals = [0, 10, 50, 100, 500, 1000, 5000, 10000]
    intervals_bytes = [x * 1024 for x in intervals]

    tally = {
        f"{intervals[i]}-{intervals[i + 1]} KB": [0, 0]
        for i in range(len(intervals) - 1)
    }
    tally[f"{intervals[-1]}+ KB"] = [0, 0]

    total_size = 0
    zip_files = list(folder.rglob("*.zip"))

    for zip_file in zip_files:
        size = zip_file.stat().st_size
        total_size += size

        placed = False
        for i in range(len(intervals_bytes) - 1):
            if intervals_bytes[i] <= size < intervals_bytes[i + 1]:
                key = f"{intervals[i]}-{intervals[i + 1]} KB"
                tally[key][0] += 1
                tally[key][1] += size
                placed = True
                break
        if not placed:
            tally[f"{intervals[-1]}+ KB"][0] += 1
            tally[f"{intervals[-1]}+ KB"][1] += size

    for k, (n_files, size) in tally.items():
        print(f"{k}:\t{n_files} files\t{format_size(size)}")
    print()
    print(f"{len(zip_files)} zip files ({format_size(total_size)})")


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("folder", type=Path, help="Folder to scan")
    args = parser.parse_args()
    if not args.folder.is_dir():
        print(f"Error: {args.folder} is not a directory")
        exit(1)
    scan_zip_sizes(args.folder)


if __name__ == "__main__":
    main()
