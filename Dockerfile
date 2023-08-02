FROM rust:1.71.0-buster

RUN useradd -ms /bin/bash appuser
USER appuser

COPY --chown=appuser:appuser . /home/appuser/app
WORKDIR /home/appuser/app

# ENTRYPOINT ["python", "main.py"]

# Build like this:
# docker build -t rust-sketchpad .

# Run like this:
# docker run -it rust-sketchpad
