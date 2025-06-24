#!/home/lbz/venv/bin/python3
import io
import contextlib
import traceback

def run_code(code: str) -> str:
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
    while True:
        try:
            input_string = input()
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