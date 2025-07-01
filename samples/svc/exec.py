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

def multiline_input(prompt: str, verbose = False) -> str:
    if verbose:
        print(prompt, end="")
    lines = []
    while True:
        try:
            line = input()
            if line == "\x04":  # ASCII End of Transmission (EOT)
                break
            lines.append(line)
        except EOFError:
            break
        except KeyboardInterrupt:
            raise
    return "\n".join(lines)

if __name__ == "__main__":
    while True:
        try:
            input_string = multiline_input("Enter code (end with EOT 0x04):\n", False)
            output, error = run_code(input_string)
            if error:
                print(error)
            else:
                print(output)
        except EOFError:
            print("\nExiting execution service.")
            break
        except KeyboardInterrupt:
            print("\nExiting execution service.")
            break