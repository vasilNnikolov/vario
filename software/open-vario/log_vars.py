import os
import re
import signal
import subprocess


def run_command(command: list[str]):
    last_line = ""
    os.environ["PYTHONUNBUFFERED"] = "1"
    process = subprocess.Popen(
        command,
        stdout=subprocess.PIPE,
    )

    try:
        while True:
            output = process.stdout.read(1).decode("utf-8")
            if output == "" and process.poll() is not None:
                break

            print(output, end="", flush=True)

            last_line = last_line + output
            # print(last_line)
            if result := re.search("VAR (.*) (.*)", last_line):
                var_name, var_value = result.group(1), result.group(2)
                print(f"Name {var_name}, value {var_value}")
                last_line = ""

    except KeyboardInterrupt:
        if process:
            os.kill(process.pid, signal.SIGTERM)


if __name__ == "__main__":
    run_command("cargo run --release".split(" "))
