FROM python:3.9-slim

RUN apt-get -y update && apt-get install -y \
    libpq-dev \
    cron \
    curl \
    python3-pip


WORKDIR /app


COPY requirements.txt ./
RUN pip install --no-cache-dir -r requirements.txt


COPY email_sender.py .


CMD ["python", "email_sender.py"]
