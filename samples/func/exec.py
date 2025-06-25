#!/home/lbz/venv/bin/python3
import io
import contextlib
import traceback
from typing import Tuple

def run_code(code: str) -> Tuple[str, str]:
    result = ""
    error = ""
    try:
        stdout = io.StringIO()
        with contextlib.redirect_stdout(stdout):
            exec(code, {}, {})
        result = stdout.getvalue()
    except Exception:
        error = "ERROR:\n" + traceback.format_exc()
    return result, error

if __name__ == "__main__":
    input_string = input()
    # execute the input string as a Python code snippet and capture the output
    output, error = run_code(input_string)
    if error:
        print(error)
    else:
        print(output)