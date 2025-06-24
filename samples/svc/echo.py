#!/home/lbz/venv/bin/python3
if __name__ == "__main__":
    while True:
        try:
            input_string = input()
            print(f"Echo: {input_string}")
        except EOFError:
            print("\nExiting echo service.")
            break
        except KeyboardInterrupt:
            print("\nExiting echo service.")
            break