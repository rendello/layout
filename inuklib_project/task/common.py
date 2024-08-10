import pathlib
from pathlib import Path
from datetime import datetime
from zoneinfo import ZoneInfo
from sys import exit


PROJECT_ROOT_NAME = "inuklib_project"

def get_project_root():
    cwd = pathlib.Path.cwd()
    match _get_project_root(cwd.parts):
        case None:
            build_exit(
                f'''Project root, \"{PROJECT_ROOT_NAME}\" not '''
                f'''found in current working directory or ancestor: "{cwd}".''',
            )
        case parts:
            return Path(*parts).resolve()


def _get_project_root(parts):
    if parts == ():
        return None
    elif parts[-1] == PROJECT_ROOT_NAME:
        return parts
    else:
        return _get_project_root(parts[:-1])


PROJECT_ROOT = get_project_root()
ASSET_PATH = PROJECT_ROOT / "task/assets"

def build_time():
    return datetime.now(ZoneInfo("Canada/Eastern")).astimezone().strftime("%H:%M")


def build_print(s: str):
    print(f"Inuklib {build_time()}\t{s}")


def build_exit(s: str):
    exit(f"Inuklib [error] {build_time()}\t{s}")