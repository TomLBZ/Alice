FROM python:slim-bookworm
SHELL [ "/bin/bash", "-c" ]
ARG UNAME=lbz
ARG GNAME=lbz
ARG UID=1000
ARG GID=1000
RUN groupadd -o -g $GID $GNAME && useradd -m -r -u $UID -g $GID $UNAME -o
USER lbz
WORKDIR /home/lbz
# prepare a venv
COPY requirements.txt .
RUN python3 -m venv venv
RUN echo "source /home/lbz/venv/bin/activate" >> ~/.bashrc
RUN source /home/lbz/venv/bin/activate && pip3 install --upgrade pip
RUN source /home/lbz/venv/bin/activate && pip3 install -r requirements.txt
WORKDIR /app
# For production, copy over instead of mounting
# COPY . /app/
CMD ["/bin/bash" , "-c", "source /home/lbz/venv/bin/activate && python3 app.py"]