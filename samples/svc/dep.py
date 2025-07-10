#!/home/lbz/venv/bin/python3
import subprocess
import sys
from typing import List

def check_package_installed(package: str) -> bool:
    """Check if a package is installed."""
    try:
        subprocess.check_call([sys.executable, "-m", "pip", "show", package], stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
        return True
    except subprocess.CalledProcessError:
        return False

def pip_install(package):
    try:
        if check_package_installed(package):
            print(f"{package} is already installed.")
            return
        print(f"Installing {package}...")
        subprocess.check_call([sys.executable, "-m", "pip", "install", package])
        print(f"Successfully installed {package}")
    except subprocess.CalledProcessError as e:
        print(f"Failed to install {package}: {e}")

def comma_separated_input(prompt: str) -> List[str]:
    print(prompt, end="")
    input_string = input()
    return [pkg.strip() for pkg in input_string.split(",") if pkg.strip()]

if __name__ == "__main__":
    while True:
        try:
            inputs = comma_separated_input("Enter dependencies to install: ")
            if not inputs:
                print("No dependencies provided.")
                continue
            for input_string in inputs:
                if not input_string or input_string == "\x04":  # ASCII End of Transmission (EOT)
                    continue
                else:
                    pip_install(input_string)
        except EOFError:
            print("\nExiting dependency service.")
            break
        except KeyboardInterrupt:
            print("\nExiting dependency service.")
            break