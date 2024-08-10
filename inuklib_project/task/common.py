import importlib
import pathlib
from datetime import datetime
from zoneinfo import ZoneInfo
from sys import exit





def build_time():
	return datetime.now(ZoneInfo("Canada/Eastern")).astimezone().strftime("%H:%M")


def build_exit(s: str):
	exit(f"Inuklib [error] {build_time()}\t{s}")


def build_print(s: str):
	print(f"Inuklib {build_time()}\t{s}")


def asset_path(file_name):
    return importlib.resources.files(__package__) / "assets" / file_name
